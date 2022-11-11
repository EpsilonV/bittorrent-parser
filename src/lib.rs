#![allow(non_camel_case_types)]

// use std::error::Error;
use std::io;
// use std::{collections::BTreeMap, fmt::Error};
// type bytes = Vec<u8>;
// enum Bencode {
//     Dict(std::collections::BTreeMap<Vec<u8>, Self>),
//     List(Vec<Self>),
//     Int(i64),
//     Str(bytes),
// }

// struct Parser<'a> {
//     data: &'a bytes,
//     index: usize,
// }

// impl<'a> Parser<'a> {
//     fn new(data: &'a bytes) -> Self {
//         Parser {
//             data: (data),
//             index: (0),
//         }
//     }

//     fn decode(self) -> Bencode {
//         let result = match self.peek() {
//             Some(b'd') => self.parse_dict(),
//             Some(b'l') => self.parse_list(),
//             Some(b'1') | Some(b'2') | Some(b'3') => self.parse_str(),
//             Some(b'i') => self.parse_int(),
//             _ => {
//                 panic!("Invalid token");
//             }
//         };

//         result
//     }

//     fn peek(self) -> Option<u8> {
//         if self.index >= self.data.len() {
//             return None;
//         }
//         Some(self.data[self.index])
//     }

//     fn consume(self) {
//         self.index += 1;
//     }

//     fn read_byte(self) -> Option<u8> {
//         let byte = self.peek();
//         self.consume();
//         return byte;
//     }

//     fn parse_dict(self) -> io::Result<Bencode> {
//         let mut dict: BTreeMap<bytes, Bencode> = std::collections::BTreeMap::new();
//         self.consume();
//         let key = self.decode();
//         let value = self.decode();
//         dict[key] = value;
//     }

//     fn parse_list(self) -> io::Result<Bencode> {}

//     fn parse_str(self) -> Result<String, io::Error> {
//         let length = self.parse_string_length()? as usize;
//         if length + self.index >= self.data.len() {
//             return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid token"));
//         }

//         Ok(String::from(self.data[self.index..(self.index + length)]))
//     }

//     fn parse_int(self) -> Result<Bencode, io::Error> {
//         let mut s = String::new();
//         loop {
//             let b = self
//                 .read_byte()
//                 .ok_or(io::Error::new(io::ErrorKind::InvalidData, "invalid token"))?;

//             if b == b'e' {
//                 break;
//             }
//             let ok = if s == "" {
//                 b == b'-' || b'0' <= b && b <= b'9'
//             } else if s == "-" {
//                 b'1' <= b && b <= b'9'
//             } else if b == b'0' {
//                 false
//             } else {
//                 b'0' <= b && b <= b'9'
//             };
//             if !ok {
//                 return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid token"));
//             }
//             s.push(b as char);
//         }

//         if s == "" || s == "-" {
//             return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid token"));
//         }
//         s.parse::<i64>()
//             .map(Bencode::Int)
//             .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid token"))
//     }

//     fn parse_string_length(self) -> Result<i64, io::Error> {
//         let mut s: String = String::new();
//         loop {
//             let b = self
//                 .read_byte()
//                 .ok_or(io::Error::new(io::ErrorKind::InvalidData, "invalid token"))?;
//             if b == b':' {
//                 break;
//             }
//             if b'0' > b || b > b'9' || s == "0" {
//                 return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid token"));
//             }
//             s.push(char::from(b));
//         }
//         s.parse::<i64>()
//             .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid token"))
//     }
// }

// impl Bencode {}

/*
 * BitTorrent bencode encoder/decoder (Rust)
 *
 * Copyright (c) 2021 Project Nayuki. (MIT License)
 * https://www.nayuki.io/page/bittorrent-bencode-format-tools
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 * - The above copyright notice and this permission notice shall be included in
 *   all copies or substantial portions of the Software.
 * - The Software is provided "as is", without warranty of any kind, express or
 *   implied, including but not limited to the warranties of merchantability,
 *   fitness for a particular purpose and noninfringement. In no event shall the
 *   authors or copyright holders be liable for any claim, damages or other
 *   liability, whether in an action of contract, tort or otherwise, arising from,
 *   out of or in connection with the Software or the use or other dealings in the
 *   Software.
 */

// use std::io;

// #[derive(Clone, Eq, PartialEq, Debug)]
// pub enum Bencode {
//     Int(i64),

//     Bytes(Vec<u8>),

//     List(Vec<Self>),

//     Dict(std::collections::BTreeMap<Vec<u8>, Self>),
// }

// impl Bencode {
//     /*---- Serializer ----*/
//     pub fn serialize(&self, out: &mut dyn io::Write) -> io::Result<()> {
//         match *self {
//             Self::Int(num) => {
//                 let temp: String = format!("i{}e", num);
//                 out.write_all(temp.as_bytes())
//             }
//             Self::Bytes(ref bytes) => Self::serialize_bytes(bytes, out),
//             Self::List(ref list) => {
//                 out.write_all(b"l")?;
//                 for item in list {
//                     item.serialize(out)?;
//                 }
//                 out.write_all(b"e")
//             }
//             Self::Dict(ref dict) => {
//                 out.write_all(b"d")?;
//                 for (key, item) in dict {
//                     Self::serialize_bytes(key, out)?;
//                     item.serialize(out)?;
//                 }
//                 out.write_all(b"e")
//             }
//         }
//     }

//     fn serialize_bytes(bytes: &[u8], out: &mut dyn io::Write) -> io::Result<()> {
//         let temp: String = format!("{}:", bytes.len());
//         out.write_all(temp.as_bytes())?;
//         out.write_all(bytes)
//     }

//     /*---- Parser ----*/
//     pub fn parse(input: &mut dyn io::Read) -> io::Result<Self> {
//         Parser { input }.parse()
//     }
// }

// struct Parser<'a> {
//     input: &'a mut dyn io::Read,
// }

// impl<'a> Parser<'a> {
//     pub fn parse(&mut self) -> io::Result<Bencode> {
//         let mut b: u8 = self.read_byte()?;
//         let result: Bencode = self.parse_value(b)?;
//         if self.input.read(std::slice::from_mut(&mut b))? > 0 {
//             return Self::err_invalid_data("Unexpected extra data");
//         }
//         Ok(result)
//     }

//     fn parse_value(&mut self, head: u8) -> io::Result<Bencode> {
//         match head {
//             b'i' => self.parse_integer(),
//             b'l' => self.parse_list(),
//             b'd' => self.parse_dictionary(),
//             b'0'..=b'9' => Ok(Bencode::Bytes(self.parse_byte_string(head)?)),
//             _ => Self::err_invalid_data("Unexpected value type"),
//         }
//     }

//     fn parse_integer(&mut self) -> io::Result<Bencode> {
//         let mut s = String::new();
//         loop {
//             let b: u8 = self.read_byte()?;
//             if b == b'e' {
//                 break;
//             }

//             let ok = if s == "" {
//                 b == b'-' || b'0' <= b && b <= b'9'
//             } else if s == "-" {
//                 b'1' <= b && b <= b'9'
//             } else if s == "0" {
//                 false
//             } else {
//                 // s starts with [123456789] or -[123456789]
//                 b'0' <= b && b <= b'9'
//             };

//             if !ok {
//                 return Self::err_invalid_data("Unexpected integer character");
//             }
//             s.push(char::from(b));
//         }
//         if s == "" || s == "-" {
//             return Self::err_invalid_data("Invalid integer syntax");
//         }
//         s.parse::<i64>()
//             .map(Bencode::Int)
//             .map_err(|_| Self::invalid_data("Integer overflow"))
//     }

//     fn parse_byte_string(&mut self, head: u8) -> io::Result<Vec<u8>> {
//         let length: usize = self.parse_natural_number(head)?;
//         let mut result = vec![0u8; length];
//         self.input.read_exact(&mut result)?;
//         Ok(result)
//     }

//     fn parse_natural_number(&mut self, head: u8) -> io::Result<usize> {
//         let mut s = String::new();
//         let mut b: u8 = head;
//         loop {
//             if b < b'0' || b > b'9' || s == "0" {
//                 return Self::err_invalid_data("Unexpected integer character");
//             }
//             s.push(char::from(b));
//             b = self.read_byte()?;
//             if b == b':' {
//                 break;
//             }
//         }
//         s.parse::<usize>()
//             .map_err(|_| Self::invalid_data("Integer overflow"))
//     }

//     fn parse_list(&mut self) -> io::Result<Bencode> {
//         let mut result = Vec::<Bencode>::new();
//         loop {
//             match self.read_byte()? {
//                 b'e' => break,
//                 b => result.push(self.parse_value(b)?),
//             }
//         }
//         Ok(Bencode::List(result))
//     }

//     fn parse_dictionary(&mut self) -> io::Result<Bencode> {
//         let mut result = std::collections::BTreeMap::<Vec<u8>, Bencode>::new();
//         loop {
//             let key = match self.read_byte()? {
//                 b'e' => break,
//                 b => self.parse_byte_string(b)?,
//             };
//             let prevkey: Option<&Vec<u8>> = result.keys().next_back();
//             if prevkey.map_or(false, |k| key <= *k) {
//                 return Self::err_invalid_data("Misordered dictionary key");
//             }
//             let b: u8 = self.read_byte()?;
//             let val: Bencode = self.parse_value(b)?;
//             result.insert(key, val);
//         }
//         Ok(Bencode::Dict(result))
//     }

//     pub fn read_byte(&mut self) -> io::Result<u8> {
//         let mut result = 0u8;
//         self.input.read_exact(std::slice::from_mut(&mut result))?;
//         Ok(result)
//     }

//     fn err_invalid_data<T>(msg: &str) -> io::Result<T> {
//         Err(Self::invalid_data(msg))
//     }

//     fn invalid_data(msg: &str) -> io::Error {
//         io::Error::new(io::ErrorKind::InvalidData, msg)
//     }
// }

type bytes = Vec<u8>;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Bencode {
    Dict(std::collections::BTreeMap<Vec<u8>, Self>),
    List(Vec<Self>),
    Int(i64),
    Str(bytes),
}

struct Parser {
    scanner: Scanner,
}

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
        if let Some(b) = self.scanner.pop() {
            match b {
                b'i' => self.parse_integer(),
                b'l' => self.parse_dict(),
                b'd' => self.parse_dict(),
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
            if let Some(b) = self.scanner.pop() {
                if b == b'e' {
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
            if let Some(b) = self.scanner.pop() {
                if b == b'e' {
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

    pub fn peek(&self) -> Option<&u8> {
        self.characters.get(self.cursor)
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
}
