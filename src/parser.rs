use std::collections::HashMap;
use std::num::ParseIntError;

use crate::ast::InfixExpression;
use crate::ast::PrefixExpression;
use crate::token;
use crate::lexer;
use crate::ast::{
    Statement, 
    Expression, 
    Program, 
    LetStatement, 
    Identifier, 
    Integer
};
use crate::token::TokenType;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PrecedenceType {
    LOWEST = 0,
    LESSGREATER,
    ADD,
    MULTIPLY,
    CALL
}

pub struct Parser {
    pub l: lexer::Lexer,
    pub errors: Vec<String>,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn new(l: lexer::Lexer) -> Parser {
        let mut parser = Parser { 
            l,
            errors: vec![],
            cur_token: token::Token{ token_type: token::EOF, literal: "\0".to_string() },
            peek_token: token::Token{ token_type: token::EOF, literal: "\0".to_string() },
        };
        // advance tokens so that curToken is the first token from the lexer
        parser.next_token();
        parser.next_token();

        return parser;
    }

    fn dummy(&self) {
        println!("THIS IS A DUMMY FN");
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

    fn parse_statement(&mut self) -> Statement {
        if (self.cur_token_is(token::LET)) {
            return self.parse_let_statement();
        } else {
            return self.parse_expression_statement();
        }
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let exprs = self.parse_expression(PrecedenceType::LOWEST);
        self.next_token();
        return Statement::ExpressionStatement(exprs);
    }

    fn parse_expression(&mut self, precedence: PrecedenceType) -> Expression {
        let mut left = self.call_prefix_parser();

        while precedence < self.peek_precedence() {
            self.next_token(); // current now is the infix opreator
            let right = self.call_infix_parser(&left);

            if (right == Expression::Nil) {
                return left;
            }

            left = right;
        }

        return left;
    }

    fn parse_let_statement(&mut self) -> Statement {
        self.next_token();  // advance token to identifier
        let name = Identifier(self.cur_token.literal.clone());

        if !self.expect_peek(token::ASSIGN) { 
            return Statement::ExpressionStatement(Expression::Nil);
        }

        self.next_token();

        let value = self.parse_expression(PrecedenceType::LOWEST);

        if !self.expect_peek(token::SEMICOLON) {
            return Statement::Nil;
        }

        Statement::LetStatement(LetStatement {
            name,
            value
        })
    }

    fn call_prefix_parser(&mut self) -> Expression {
        match self.cur_token.token_type {

            token::INT => 
                self.parse_integer()
                .map_or(Expression::Nil, |v| Expression::Int(v)),
            
            token::IDENTIFIER => 
                self.parse_identifier()
                .map_or(Expression::Nil, |v| Expression::Ident(v)),
            
            token::BANG | token::MINUS => 
                self.parse_prefix_expression()
                .map_or(Expression::Nil, |v| Expression::Prefix(v)),

            _ => Expression::Nil
        }
    }

    fn call_infix_parser(&mut self, left: &Expression) -> Expression {
        match self.cur_token.token_type {
            token::PLUS => { 
                dbg!("before entring infix plug function");
                dbg!(&self.cur_token);
                let r = self.parse_infix_expression(&left)
                    .map_or(Expression::Nil, |v| Expression::Infix(v));
                return r;
            },
            _ => Expression::Nil,
        }
    }

    fn parse_integer(&self) -> Result<Integer, String> {
        let value = self
            .cur_token.literal
            .trim()
            .parse::<i32>()
            .expect("Cannot parse self.curk_token.literal as i32");
        return Ok(Integer(value));
    }

    fn parse_identifier(&self) -> Result<Identifier, String> {
        Ok(Identifier(self.cur_token.literal.to_owned()))
    }

    fn parse_prefix_expression(&mut self) -> Result<PrefixExpression, String> {
        let cur_tk = self.cur_token.clone();
        self.next_token();
        let right = self.parse_expression(PrecedenceType::LOWEST);

        Ok(PrefixExpression {
            op: cur_tk.literal.clone(),
            right: Box::new(right)
        })
    }

    fn parse_infix_expression(&mut self, left: &Expression) -> Result<InfixExpression, String> {
        let cur_tk = self.cur_token.clone();

        let precedence = self.cur_precedence();

        self.next_token();
        dbg!("before right expression: cur token");
        dbg!(&self.cur_token);
        let right = self.parse_expression(precedence);

        Ok(InfixExpression {
            left: Box::new(left.clone()),
            op: cur_tk.literal.clone(),
            right: Box::new(right),
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

    fn peek_precedence(&self) -> PrecedenceType {
        self.get_token_precedence(self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> PrecedenceType {
        self.get_token_precedence(self.cur_token.token_type)
    }

    fn get_token_precedence(&self, tt: token::TokenType) -> PrecedenceType {
        match tt {
            token::PLUS => PrecedenceType::ADD,
            _ => PrecedenceType::LOWEST
        }
    }

    fn new_error(&mut self, message: &str) {
        self.errors.push(message.to_string());
    }
}
