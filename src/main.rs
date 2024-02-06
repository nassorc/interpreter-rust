#![allow(unused)]

mod token;
mod lexer;
mod parser;
mod ast;

use std::collections::HashMap;

use token::Token;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let mut lexer = Lexer::new("hello");
    let mut parser = Parser::new(lexer);

    let prog = parser.parse_program();
    println!("len: {}", prog.statements.len());

    if parser.errors.len() > 0 {
        for err in parser.errors {
            println!("{}", err);
        }
    }

    for node in prog.statements {
        dbg!(&node);
    }

}