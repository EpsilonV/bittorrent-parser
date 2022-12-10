use std::collections::HashSet;
use std::fs::File;
use std::io::Error;
use std::{collections::BTreeMap, io};
mod hash;
mod parser;
mod utils;

use hyper::Client;
use parser::{bytes, Bencode, Parser};

use hash::{calculate_sha1, generate_peer_id, Sha1};

type Dict = BTreeMap<bytes, Bencode>;

struct SingleFile<'a> {
    name: &'a Vec<u8>,
    length: i64,
    md5sum: Option<bytes>,
}

struct MultipleFileMetainfo {
    length: i64,
    md5sum: bytes,
    path: Vec<bytes>,
}

struct MultipleFile {
    name: bytes,
    files: Vec<MultipleFileMetainfo>,
}

enum FileInfo<'a> {
    Single(SingleFile<'a>),
    Mul(MultipleFile),
}

struct Info<'a> {
    piece_length: i64,
    pieces: &'a bytes,
    private: Option<i64>,
    // file: FileInfo<'a>,
    name: &'a Vec<u8>,
    length: i64,
    md5sum: Option<bytes>,
}
struct Metainfo<'a> {
    info: Info<'a>,
    announce: &'a bytes,
    peer_id: String,
    info_hash: Sha1,
    announce_list: Option<Vec<bytes>>,
    creation_date: Option<u64>,
    comment: Option<bytes>,
    created_by: Option<bytes>,
    encoding: Option<bytes>,
}

fn get_value_form_dict(dict: &Dict, key: Vec<u8>) -> Result<&Bencode, Error> {
    if let Some(value) = dict.get(&key) {
        Ok(value)
    } else {
        return Err(io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("missing {:?}", key),
        ));
    }
}

fn parse_singlefile(info: &Dict) -> Result<SingleFile, Error> {
    let name = get_value_form_dict(info, b"name".to_vec())?.into_str()?;
    // let name: bytes = name.into()?;
    let length: i64 = (*get_value_form_dict(info, b"length".to_vec())?).into_int()?;
    // if let Some(md5sum) = info.get(&b"md5sum".to_vec()) {
    //     let md5sum: bytes = (*md5sum).into()?;
    // }
    Ok(SingleFile {
        name: name,
        length: length,
        md5sum: None,
    })
}

fn parse_fileinfo(info: &Dict) -> Result<FileInfo, Error> {
    let multi_file = info.contains_key(&b"files".to_vec());
    if multi_file {
        //todo
        return Err(io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("not support multiple file"),
        ));
    } else {
        let singlefile = parse_singlefile(info)?;
        return Ok(FileInfo::Single(singlefile));
    }
}

fn parse_info(info: &Dict) -> Result<Info, Error> {
    let piece_length = get_value_form_dict(info, b"piece length".to_vec())?.into_int()?;
    let pieces = get_value_form_dict(info, b"pieces".to_vec())?.into_str()?;
    let fileinfo = parse_fileinfo(info)?;
    let name = get_value_form_dict(info, b"name".to_vec())?.into_str()?;
    let length: i64 = (*get_value_form_dict(info, b"length".to_vec())?).into_int()?;
    Ok(Info {
        piece_length,
        pieces,
        private: None,
        name,
        length,
        md5sum: None,
    })
}

impl<'a> Metainfo<'a> {
    fn new(bencode: &'a Bencode) -> Result<Metainfo, Error> {
        match bencode {
            Bencode::Dict(dict) => {
                //get info dict
                let info = get_value_form_dict(dict, b"info".to_vec())?;
                let info_bytes = info.to_bytes()?;
                let infodict = info.into_dict()?;
                let info_hash = calculate_sha1(&info_bytes);
                let info = parse_info(infodict)?;
                let announce = get_value_form_dict(dict, b"announce".to_vec())?.into_str()?;
                let peer_id = generate_peer_id();
                Ok(Metainfo {
                    info,
                    announce,
                    peer_id,
                    info_hash,
                    announce_list: None,
                    creation_date: None,
                    comment: None,
                    created_by: None,
                    encoding: None,
                })
            }
            _ => Err(io::Error::new(
                std::io::ErrorKind::InvalidData,
                "bencode not a valid dict!",
            )),
        }
    }
}

fn start(metaInfo: Metainfo) {
    let annouce = String::from_utf8(metaInfo.announce.clone()).unwrap();
    let mut httpclient = Client::new();
    let length = metaInfo.info.length;
    let params = vec![
        ("info_hash", metaInfo.info_hash),
        ("peer_id", metaInfo.peer_id),
        ("uploaded", 0),
        ("downloaded", 0),
        ("port", 6881),
        ("left", length),
        ("event", "started"),
    ];

    let url = format!("{}?{}", annouce, encode_url_query(params));
    print!("url is {}", url);
}

fn main() {
    let mut f = File::open(
        "/Users/epsilonv/Source/bittorrent-parser/src/debian-9.4.0-amd64-netinst.iso.torrent",
    )
    .unwrap();
    let mut parser = Parser::new(&mut f);
    let res = parser.parse().unwrap();
    let metainfo = Metainfo::new(&res).unwrap();
    println!(
        "metainfo announce{:?} {:?}",
        String::from_utf8(metainfo.announce.clone()),
        metainfo.info_hash
    );
    start(metaInfo);
}
