#![allow(unused)]

mod token;
mod lexer;

use std::borrow::BorrowMut;
use std::any::Any;
use std::fs;
use std::io;
use lexer::Lexer;
use token::Token;

use crate::token::TokenType;

trait Node {
  fn token_literal(&self) -> &str;
}

trait Statement : Node {
  fn statement_node(&self);
}
trait Expression : Node {
  fn expression_node(&self);
}

struct Program {
  statements: Vec<Box<dyn Any>>
}

impl Program {
  fn token_literal(&self) -> &str {
    if self.statements.len() > 0 {
      return self.statements[0].downcast_ref::<Box<dyn Statement>>().unwrap().token_literal();
    }
    return "";
  }
}

// let a = 10
// type identifier assign value
struct LetStatement {
  token: Token,
  name: Box<Identifier>,
  value: Box<dyn Expression>
} 

struct Identifier {
  token: Token,
  value: String
}

impl Node for Identifier {
  fn token_literal(&self) -> &str {
    return &self.token.literal;
  }
}
impl Expression for Identifier {
  fn expression_node(&self) {
  }
}

impl Node for LetStatement {
  fn token_literal(&self) -> &str {
    return "let";
  }
}
impl Statement for LetStatement {
  fn statement_node(&self) {}
}

struct IntExpression {
  token: Token,
  value: i32
}

impl Node for IntExpression {
  fn token_literal(&self) -> &str {
    return &self.token.literal;
  }
}

impl Expression for IntExpression {
  fn expression_node(&self) {
  }
}

struct Parser<'a> {
  l: &'a mut Lexer,

  current_token: Token,
  peek_token: Token
}

impl<'a> Parser<'a> {
  fn new(l: &'a mut Lexer) -> Self {
    Self {
      l: l,
      current_token: Token {
        token_type: TokenType::EOF,
        literal: "".to_string()
      },
      peek_token: Token {
        token_type: TokenType::EOF,
        literal: "".to_string()
      }
    }
  }
}

fn main() -> Result<(), io::Error> {

    let mut lexer1 = Lexer::new("let a = 10;");

    let mut parser1 = Parser::new(&mut lexer1);

    let expect_let = parser1.l.next_token();

    dbg!(expect_let);
    println!("========");


    let mut program = Program {
      statements: vec![]
    };
    let i1 = IntExpression {
      token: Token {
        token_type: TokenType::INT,
        literal: "5".to_string()
      },
      value: 5
    };
    let mut n1 = Identifier {
        token: Token {
          literal: "myInt".to_string(),
          token_type: TokenType::IDENTIFIER
        },
        value: "myInt".to_string()
      };

    let l1 = Box::new(LetStatement{
      token: Token {
        token_type: TokenType::LET,
        literal: "let".to_string()
      },
      name: Box::new(n1),
      value: Box::new(IntExpression {
        token: Token { token_type: TokenType::INT, literal: "5".to_string() },
        value: 5
      })
    });

    let mut l = Lexer::new("let == fn 10,");

    let c = l.next_token();
    dbg!(&c);
    let c = l.next_token();
    dbg!(&c);
    let c = l.next_token();
    dbg!(&c);
    Ok(())
}

