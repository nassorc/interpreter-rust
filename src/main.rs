#![allow(unused)]

mod token;
use std::{arch::x86_64::_MM_FROUND_NO_EXC, io};

use token::Token;

fn main() -> Result<(), io::Error> {
    // let l = Lexer::new("let a = 10;");
    
    // let mut p = Parser::new(l);

    let l = LetStatement{};

    let stms: Vec<Box<dyn Node>> = Vec::new();

    Ok(())
}

trait Node {
    fn to_string(&self) -> String;
    fn to_literal(&self) -> String;
}

trait StatementNode {}
trait ExpressionNode {}

struct LetStatement {}

impl Node for LetStatement {
    fn to_string(&self) -> String {
        "let statement".to_owned()
    }
    fn to_literal(&self) -> String {
        "let statement literal".to_owned()
    }
}

#[derive(Debug)]
struct Lexer {
    input: String,
    ch: char,
    position: usize,
    read_position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut l = Self {
            input: input.to_string(),
            ch: 0 as char,
            position: 0,
            read_position: 0,
        };
        l.read_char();
        return l;
    }

    fn next_token(&mut self) -> token::Token {
        let mut tmp = [0; 4];
        let tk: token::Token;

        self.skip_whitespace();

        match self.ch {
            '=' => {
                tk = self.new_token(token::ASSIGN, self.ch.encode_utf8(&mut tmp))
            },
            '+' => {
                tk = self.new_token(token::PLUS, self.ch.encode_utf8(&mut tmp))
            },
            '-' => {
                tk = self.new_token(token::MINUS, self.ch.encode_utf8(&mut tmp))
            },
            ';' => {
                tk = self.new_token(token::SEMICOLON, self.ch.encode_utf8(&mut tmp))
            },
            '\0' => {
                tk = self.new_token(token::EOF, self.ch.encode_utf8(&mut tmp))
            },
            _ => {
                if self.is_letter() {
                    let identifier = self.read_identifier();
                    let token_type = token::get_identifier(&identifier);
                    return self.new_token(token_type, &identifier);
                } 
                else if self.is_digit() {
                    let number = self.read_number();
                    return self.new_token(token::INT, &number);
                }
                tk = token::Token::new(token::ILLEGAL, (0 as char).encode_utf8(&mut tmp))
            }
        }
        self.read_char();
        return tk;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.is_letter() {
            self.read_char();
        }
        self.input[pos..self.position].to_owned()
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.is_digit() {
            self.read_char();
        }
        self.input[pos..self.position].to_owned()
    }

    fn new_token(&self, token_type: token::TokenType, literal: &str) -> token::Token {
        token::Token::new(token_type, literal)
    }

    fn is_letter(&self) -> bool {
        ('a' <= self.ch && self.ch <= 'z') || ('A' <= self.ch && self.ch <= 'Z') || (self.ch == '_')
    }

    fn is_digit(&self) -> bool {
        ('0' <= self.ch && self.ch <= '9')
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

struct Parser {
    l: Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {

    fn new(l: Lexer) -> Self {
        let mut p = Self { 
            l,
            cur_token: token::Token::new(token::EOF, "\0"),
            peek_token: token::Token::new(token::EOF, "\0"),
        };
        p.next_token();
        p.next_token();
        return p;
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }
}