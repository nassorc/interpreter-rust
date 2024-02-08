use std::{
    collections::HashMap,
    rc::Rc
};
use crate::{
    ast::{
        Boolean, Expression, Identifier, InfixExpression, Integer, LetStatement, Node, PrefixExpression, Program, Statement
    }, lexer, token
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PrecedenceType {
    LOWEST = 0,
    LESSGREATER,
    ADD,
    PRODUCT,
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

        // advance tokens so that curToken contains the first token from the lexer
        parser.next_token();
        parser.next_token();

        return parser;
    }

    // This function gets the next token from the lexer and updates cur_token
    // and peek_token.
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    // This function consumes lexer and produces a Node::Program representing
    // the AST of the source file.
    pub fn parse_program(&mut self) -> Node {
        let mut program = Program { statements: vec![] };

        while !self.cur_token_is(token::EOF) {
            let stmt = self.parse_statement();
            if !stmt.is_nil() {
                program.statements.push(stmt);
            }
        } 

        return Node::Program(program);
    }

    // This function transforms a token to represent a node of the AST,
    // and returns the Node
    // The parser parses three types of statement variants
    // 1. LetStatements         - represents a let statement
    // 2. ReturnStatements      - represnts a return statement
    // 3. Expression Statements - represents an any valid expression
    fn parse_statement(&mut self) -> Node {
        if (self.cur_token_is(token::LET)) {
            return self.parse_let_statement().map_or(
                Node::Nil,
                |v| Node::LetStatement(v)
            );
        } 
        else {
            return self.parse_expression_statement();
        }
        Node::Nil
    }

    fn parse_expression_statement(&mut self) -> Node {
        let exprs = self.parse_expression(PrecedenceType::LOWEST);
        self.next_token();
        return exprs;
    }

    fn parse_expression(&mut self, precedence: PrecedenceType) -> Node {
        let mut left = self.parse_prefix_operation();

        while precedence < self.peek_precedence() {
            self.next_token(); // advance token to the infix operator
            let right = self.call_infix_parser(&left);
            if (right.is_nil()) {
                return left;
            }
            left = right;
        }

        return left;
    }

    fn parse_prefix_operation(&mut self) -> Node {
        match self.cur_token.token_type {
            token::INT => self.parse_integer().map_or(
                Node::Nil, 
                |v| Node::Int(v)
            ),
            token::TRUE | token::FALSE => self.parse_boolean().map_or(
                Node::Nil,
                |v| Node::Boolean(v)
            ),
            token::IDENTIFIER => self.parse_identifier().map_or(
                Node::Nil, 
                |v| Node::Ident(v)
            ),
            token::MINUS | token::BANG => self.parse_prefix_expression().map_or(
                Node::Nil, 
                |v| Node::Prefix(v)
            ),
            _ => Node::Nil
        }
    }

    fn call_infix_parser(&mut self, left: &Node) -> Node {
        match self.cur_token.token_type {
            token::PLUS | token::ASTERISK => self.parse_infix_expression(&left).map_or(
                Node::Nil, 
                |v| Node::Infix(v)
            ),
            _ => Node::Nil
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        self.next_token();  // advance token to identifier
        let name = Identifier(self.cur_token.literal.clone());

        if !self.expect_peek(token::ASSIGN) { 
            return None;
        }

        self.next_token();

        let value = self.parse_expression(PrecedenceType::LOWEST);

        if !self.expect_peek(token::SEMICOLON) {
            return None;
        }

        Some(LetStatement {
            name,
            value: Rc::new(value)
        })
    }


    fn parse_integer(&self) -> Result<Integer, String> {
        let value = self
            .cur_token.literal
            .trim()
            .parse::<i32>()
            .expect("Cannot parse self.curk_token.literal as i32");
        return Ok(Integer(value));
    }

    fn parse_boolean(&self) -> Result<Boolean, String> {
        Ok(Boolean(self.cur_token_is(token::TRUE)))
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
            right: Rc::new(right)
        })
    }

    fn parse_infix_expression(&mut self, left: &Node) -> Result<InfixExpression, String> {
        let cur_tk = self.cur_token.clone();
        let precedence = self.cur_precedence(); // save current precedence before advancing lexer

        self.next_token();

        let right = self.parse_expression(precedence);

        Ok(InfixExpression {
            left: Rc::new(left.clone()),
            op: cur_tk.literal.clone(),
            right: Rc::new(right),
        })
    }

    fn expect_peek(&mut self, tk: token::TokenType) -> bool {
        if (self.peek_token_is(tk)) {
            self.next_token();
            return true;
        }
        // if self.cur_token is not expected tk, push error
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
            token::PLUS | token::MINUS => PrecedenceType::ADD,
            token::ASTERISK | token::SLASH => PrecedenceType::PRODUCT,
            _ => PrecedenceType::LOWEST
        }
    }

    fn new_error(&mut self, message: &str) {
        self.errors.push(message.to_string());
    }
}
