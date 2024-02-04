use std::collections::HashMap;

use crate::token;
use crate::lexer;
use crate::ast::{
  NodeType, 
  ExpressionType, 
  Program, 
  LetStatement, 
  Identifier, 
  Integer
};

enum PrecedenceType {
    LOWEST,
    LESSGREATER,
    ADD,
    MULTIPLY,
    CALL
}

pub struct Parser {
    pub l: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
    pub errors: Vec<String>
}

impl Parser {
    pub fn new(l: lexer::Lexer) -> Parser {
        let mut parser = Parser { 
            l,
            cur_token: token::Token{ token_type: token::EOF, literal: "\0".to_string() },
            peek_token: token::Token{ token_type: token::EOF, literal: "\0".to_string() },
            errors: vec![]
        };
        parser.next_token();
        parser.next_token();
        return parser;
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while !self.cur_token_is(token::EOF) {
            let stmt = self.parse_statement();
            program.statements.push(stmt);
        } 

        return program;
    }

    fn parse_statement(&mut self) -> NodeType {
        if (self.cur_token_is(token::LET)) {
            return self.parse_let_statement();
        } else {
            self.next_token();
            return self.parse_expression_statement();
        }
        self.next_token();
        return NodeType::Nil;
    }

    fn parse_expression_statement(&self) -> NodeType {
        let exprs = self.parse_expression(PrecedenceType::LOWEST);
        return NodeType::ExpressionStatement(exprs);
    }

    fn parse_expression(&self, precedence: PrecedenceType) -> ExpressionType {
        ExpressionType::Int(Integer(9009))
    }

    fn parse_let_statement(&mut self) -> NodeType {
        self.next_token();
        let name = Identifier(self.cur_token.literal.clone());
        if !self.expect_peek(token::ASSIGN) { return NodeType::ExpressionStatement(ExpressionType::Nil); }
        let value = ExpressionType::Int(Integer(1000));
        let value = self.parse_expression(PrecedenceType::LOWEST);

        if !self.expect_peek(token::SEMICOLON) {
          self.new_error(format!("Expectect ;, got {}", self.cur_token.token_type).as_str());
          return NodeType::Nil;
        }

        NodeType::LetStatement(LetStatement {
            name,
            value
        })
    }

    fn expect_peek(&mut self, tk: token::TokenType) -> bool {
        if (self.peek_token_is(tk.clone())) {
            self.next_token();
            return true;
        }
        self.errors.push(format!("Expected peek_token to be {}, got {}", tk, self.peek_token.token_type));
        return false;
    }

    fn cur_token_is(&self, tk: token::TokenType) -> bool {
        self.cur_token.token_type == tk
    }

    fn peek_token_is(&self, tk: token::TokenType) -> bool {
        self.peek_token.token_type == tk
    }

    fn new_error(&mut self, message: &str) {
      self.errors.push(message.to_string());
    }
}
