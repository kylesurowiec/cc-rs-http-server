use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::Result;

use crate::json::{Token, TokenType};

pub struct Tokenizer<'a> {
    filename: &'a str,
    content: String,
}

impl<'a> Tokenizer<'_> {
    pub fn new(filename: &str) -> Result<Tokenizer> {
        let file = File::open(&filename)?;
        let mut content = String::new();
        let mut buf = BufReader::new(&file);
        buf.read_to_string(&mut content)?;

        Ok(Tokenizer { filename, content })
    }

    pub fn get_token(&self) -> Token {
        Token {
            value: "".to_string(),
            token_type: TokenType::ArrayClose,
        }
    }

    // ignore whitespace?
    pub fn get_next_token_as_str(&self) -> char {
        let c = ' ';

        loop {
            if c != ' ' || c != '\n' {
                break;
            }
            return c;
        }

        c
    }

    pub fn go_back(&self) -> Token {
        Token {
            value: "".to_string(),
            token_type: TokenType::ArrayClose,
        }
    }

    pub fn has_more(&self) -> bool {
        false
    }
}
