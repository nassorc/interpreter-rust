#![allow(unused)]

mod token;
mod lexer;

use std::fs;
use std::io;
use lexer::Lexer;

use crate::token::TokenType;

fn main() -> Result<(), io::Error> {
    let mut l = Lexer::new("let == fn 10,");
    let c = l.next_token();
    dbg!(&c);
    let c = l.next_token();
    dbg!(&c);
    let c = l.next_token();
    dbg!(&c);
    Ok(())
}

