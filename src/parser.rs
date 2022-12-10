#![allow(non_camel_case_types)]

use std::fmt;
use std::io;

pub type bytes = Vec<u8>;

pub struct MyVec(pub bytes);

impl fmt::Display for MyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for c in &self.0 {
            s.push(*c as char);
        }
        write!(f, "{}", s)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Bencode {
    Dict(std::collections::BTreeMap<Vec<u8>, Self>),
    List(Vec<Self>),
    Int(i64),
    Str(bytes),
}

// impl From<Bencode> for Result<i64, std::io::Error> {
//     fn from(bencode: Bencode) -> Self {
//         match bencode {
//             Bencode::Int(val) => Ok(val),
//             _ => Err(io::Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 format!("missing key"),
//             )),
//         }
//     }
// }

// impl From<Bencode> for Result<bytes, std::io::Error> {
//     fn from(bencode: Bencode) -> Self {
//         match bencode {
//             Bencode::Str(val) => Ok(val),
//             _ => Err(io::Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 format!("missing key"),
//             )),
//         }
//     }
// }

// impl Into<Result<i64, std::io::Error>> for Bencode {
//     fn into(self) -> Result<i64, std::io::Error> {
//         match self {
//             Bencode::Int(val) => Ok(val),
//             _ => Err(io::Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 format!("missing key"),
//             )),
//         }
//     }
// }

// impl Into<Result<bytes, std::io::Error>> for Bencode {
//     fn into(self) -> Result<bytes, std::io::Error> {
//         match self {
//             Bencode::Str(val) => Ok(val),
//             _ => Err(io::Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 format!("missing key"),
//             )),
//         }
//     }
// }

// impl Into<Result<Vec<Bencode>, std::io::Error>> for Bencode {
//     fn into(self) -> Result<Vec<Bencode>, std::io::Error> {
//         match self {
//             Bencode::List(val) => Ok(val),
//             _ => Err(io::Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 format!("missing key"),
//             )),
//         }
//     }
// }

// impl Into<Result<std::collections::BTreeMap<Vec<u8>, Bencode>, std::io::Error>> for Bencode {
//     fn into(self) -> Result<std::collections::BTreeMap<Vec<u8>, Bencode>, std::io::Error> {
//         match self {
//             Bencode::Dict(val) => Ok(val),
//             _ => Err(io::Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 format!("missing key"),
//             )),
//         }
//     }
// }

impl Bencode {
    pub fn serialize(&self, out: &mut dyn io::Write) -> io::Result<()> {
        match *self {
            Self::Int(val) => {
                let temp = format!("i{}e", val);
                out.write_all(temp.as_bytes())
            }
            Self::Str(ref bytes) => Self::serialize_bytes(bytes, out),
            Self::List(ref list) => {
                out.write_all(b"l")?;
                for item in list {
                    item.serialize(out)?;
                }
                out.write_all(b"e")
            }
            Self::Dict(ref dict) => {
                out.write_all(b"d")?;
                for (key, item) in dict {
                    Self::serialize_bytes(key, out)?;
                    item.serialize(out)?;
                }
                out.write_all(b"e")
            }
        }
    }

    pub fn serialize_bytes(bytes: &[u8], out: &mut dyn io::Write) -> io::Result<()> {
        let temp = format!("{}:", bytes.len());
        out.write_all(temp.as_bytes())?;
        out.write_all(bytes)
    }

    // pub fn to_writer(&self, writer: &mut io::Write) -> io::Result<()> {
    //     let mut encoder = Encoder::new(writer);
    //     self.encode(&mut encoder)
    // }

    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut writer = vec![];
        match self.serialize(&mut writer) {
            Ok(_) => Ok(writer),
            Err(err) => Err(err),
        }
    }

    pub fn into_int(&self) -> Result<i64, std::io::Error> {
        match self {
            Bencode::Int(val) => Ok(*val),
            _ => Err(io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("missing key"),
            )),
        }
    }

    pub fn into_str(&self) -> Result<&bytes, std::io::Error> {
        match self {
            Bencode::Str(val) => Ok(val),
            _ => Err(io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("missing key"),
            )),
        }
    }

    pub fn into_list(&self) -> Result<&Vec<Bencode>, std::io::Error> {
        match self {
            Bencode::List(val) => Ok(val),
            _ => Err(io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("missing key"),
            )),
        }
    }

    pub fn into_dict(
        &self,
    ) -> Result<&std::collections::BTreeMap<Vec<u8>, Bencode>, std::io::Error> {
        match self {
            Bencode::Dict(val) => Ok(val),
            _ => Err(io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("missing key"),
            )),
        }
    }
}

pub struct Parser {
    scanner: Scanner,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Error {
    Character(usize),
}

impl Parser {
    pub fn new(reader: &mut dyn io::Read) -> Parser {
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();
        Parser {
            scanner: Scanner {
                cursor: 0,
                characters: buffer,
            },
        }
    }

    pub fn parse(&mut self) -> Result<Bencode, Error> {
        if let Some(b) = self.scanner.peek() {
            match b {
                b'i' => {
                    self.scanner.pop();
                    self.parse_integer()
                }
                b'l' => {
                    self.scanner.pop();
                    self.parse_dict()
                }
                b'd' => {
                    self.scanner.pop();
                    self.parse_dict()
                }
                b'0'..=b'9' => Ok(Bencode::Str(self.parse_str()?)),
                _ => Err(Error::Character(self.scanner.cursor())),
            }
        } else {
            Err(Error::Character(self.scanner.cursor()))
        }
    }

    pub fn parse_dict(&mut self) -> Result<Bencode, Error> {
        let mut result = std::collections::BTreeMap::<Vec<u8>, Bencode>::new();
        loop {
            if let Some(b) = self.scanner.peek() {
                if b == b'e' {
                    self.scanner.pop();
                    break;
                }
                let key = self.parse_str()?;
                let value = self.parse()?;
                result.insert(key, value);
            }
        }
        Ok(Bencode::Dict(result))
    }

    pub fn parse_list(&mut self) -> Result<Bencode, Error> {
        let mut result = Vec::<Bencode>::new();
        loop {
            if let Some(b) = self.scanner.peek() {
                if b == b'e' {
                    self.scanner.pop();
                    break;
                }
                result.push(self.parse()?);
            } else {
                return Err(Error::Character(self.scanner.cursor()));
            }
        }
        Ok(Bencode::List(result))
    }

    pub fn parse_str(&mut self) -> Result<Vec<u8>, Error> {
        let length = self.parser_str_len()?;
        let mut res = Vec::new();
        let bytes_s = self.scanner.take_n_bytes(length)?;
        res.extend_from_slice(bytes_s);
        Ok(res)
    }

    pub fn parse_integer(&mut self) -> Result<Bencode, Error> {
        let mut s = String::new();
        loop {
            if let Some(b) = self.scanner.pop() {
                if b == b'e' {
                    break;
                }
                let ok = if s == "" {
                    b == b'_' || b'0' <= b && b <= b'9'
                } else if s == "-" {
                    b'1' <= b && b <= b'9'
                } else if s == "0" {
                    false
                } else {
                    b'0' <= b && b <= b'9'
                };
                if !ok {
                    return Err(Error::Character(self.scanner.cursor()));
                }
                s.push(b as char);
            } else {
                return Err(Error::Character(self.scanner.cursor()));
            }
        }

        if s == "" || s == "" {
            return Err(Error::Character(self.scanner.cursor()));
        }

        s.parse::<i64>()
            .map(Bencode::Int)
            .map_err(|_| Error::Character(self.scanner.cursor()))
    }

    fn parser_str_len(&mut self) -> Result<usize, Error> {
        let mut s = String::new();
        loop {
            if let Some(b) = self.scanner.pop() {
                if b == b':' {
                    break;
                }
                if b < b'0' || b > b'9' || s == "0" {
                    return Err(Error::Character(self.scanner.cursor()));
                }
                s.push(b as char);
            } else {
                return Err(Error::Character(self.scanner.cursor()));
            }
        }

        s.parse::<usize>()
            .map_err(|_| Error::Character(self.scanner.cursor()))
    }
}

pub struct Scanner {
    cursor: usize,
    characters: bytes,
}

impl Scanner {
    pub fn new(source: &bytes) -> Self {
        Scanner {
            cursor: 0,
            characters: source.clone(),
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<u8> {
        match self.characters.get(self.cursor) {
            Some(character) => Some(*character),
            None => None,
        }
    }

    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    pub fn pop(&mut self) -> Option<u8> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;
                Some(*character)
            }
            None => None,
        }
    }

    pub fn take_n_bytes(&mut self, n: usize) -> Result<&[u8], Error> {
        if self.cursor + n > self.characters.len() {
            return Err(Error::Character(self.cursor()));
        }
        let res = &self.characters[self.cursor..self.cursor + n];
        self.cursor += n;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::{Bencode, Parser};
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{self, Write};

    #[test]
    fn test_int() {
        let mut input = "i100e".as_bytes();
        let mut parser = Parser::new(&mut input);
        let res = parser.parse();
        match res {
            Ok(val) => {
                assert_eq!(val, Bencode::Int(100));
            }
            _ => panic!("parse failed"),
        }
    }
    #[test]
    fn test_str() {
        let mut input = "8:announce".as_bytes();
        let mut parser = Parser::new(&mut input);
        let res = parser.parse();
        match res {
            Ok(val) => {
                assert_eq!(
                    val,
                    Bencode::Str(vec!(b'a', b'n', b'n', b'o', b'u', b'n', b'c', b'e'))
                );
            }
            _ => panic!("parse failed"),
        }
    }

    #[test]
    fn test_torrent() {
        let mut f = File::open(
            "/Users/epsilonv/Source/bittorrent-parser/src/debian-9.4.0-amd64-netinst.iso.torrent",
        )
        .unwrap();
        let mut parser = Parser::new(&mut f);
        let res = parser.parse().unwrap();
        let mut stdout = io::stdout();
        res.serialize(&mut stdout).unwrap();
    }
}
