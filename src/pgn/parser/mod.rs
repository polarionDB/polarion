#![allow(dead_code)]

use phf::phf_map;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum PgnToken {
    MetadataTag(String, String),
    MoveNumber(String),
    SanMove(String),
    Annoation(String),
    VariationStart,
    VariationEnd,
    Result(char),
}

static IS_ANNOTATION: phf::Map<char, bool> = phf_map! {
    '!' => true,
    '?' => true,
    '$' => true
};

pub fn parse_pgn(str: String) -> Vec<PgnToken> {
    let str = str.as_bytes();
    let mut vec = vec![];
    
    let mut i = usize::MAX;
    while i < str.len() || i == usize::MAX {
        i = i.wrapping_add(1);

        if i >= str.len() {break;}

        if (str[i] as char).is_whitespace() {
            continue;
        }

        if str[i] == b'[' {
            i += 1;

            let mut key = String::with_capacity(64);
            let mut val = String::with_capacity(128);

            while !(str[i] as char).is_whitespace() && i < str.len() {
                key.push(str[i] as char); i += 1;
            }

            i += 2;

            while str[i] != b'"' && i < str.len() {
                val.push(str[i] as char); i += 1;
            }

            i += 2;
            vec.push(PgnToken::MetadataTag(key, val));

            continue;
        }

        if (str[i] as char).is_numeric() {
            if str[i + 1] == b'-' {
                if str[i] == b'1' {vec.push(PgnToken::Result('w'));}
                if str[i] == b'0' {vec.push(PgnToken::Result('b'));}

                i += 3;

                continue;
            } if str[i + 1] == b'/' {
                vec.push(PgnToken::Result('d')); i += 6;

                continue;
            }

            let mut val = String::with_capacity(8);
            while !(str[i] as char).is_whitespace() && i < str.len() {
                val.push(str[i] as char); i += 1;
            }

            vec.push(PgnToken::MoveNumber(val));

            continue;
        }

        if str[i].is_ascii_alphabetic() {
            let mut val = String::with_capacity(8);
            while !(str[i] as char).is_whitespace() && str[i] != b')' && i < str.len() {
                val.push(str[i] as char); i += 1;
            }

            if str[i] == b')' {i -= 1;}

            vec.push(PgnToken::SanMove(val));

            continue;
        }

        if str[i] == b'*' {
            vec.push(PgnToken::Result('u'));
            continue;
        }

        if str[i] == b'(' {
            vec.push(PgnToken::VariationStart);
            continue;
        }

        if str[i] == b')' {
            vec.push(PgnToken::VariationEnd);
            continue;
        }

        if IS_ANNOTATION.contains_key(&(str[i] as char)) {
            let mut val = String::with_capacity(8);
            while str[i] != b' ' && str[i] != b')' && i < str.len() {
                val.push(str[i] as char); i += 1;
            }

            vec.push(PgnToken::Annoation(val));

            continue;
        }
    }

    vec
}