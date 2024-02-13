pub mod ast;

use std::{collections::HashMap, rc::Rc};

use ast::*;
use crate::{
    lexer,  
    lexer::token,
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
            self.next_token();
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
        match self.cur_token.token_type {
            token::LET => self.parse_let_statement().map_or(
                Node::Nil,
                |v| Node::LetStatement(v)
            ),
            token::RETURN => self.parse_return_statement().map_or(
                Node::Nil, 
                |v| Node::ReturnStatement(v)
            ),
            _ => self.parse_expression_statement()
        }
    }

    fn parse_expression_statement(&mut self) -> Node {
        let exprs = self.parse_expression(PrecedenceType::LOWEST);
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
            token::FUNCTION => self.parse_function_literal().map_or(
                Node::Nil, 
                |v| Node::Function(v)
            ),
            token::MINUS | token::BANG => self.parse_prefix_expression().map_or(
                Node::Nil, 
                |v| Node::Prefix(v)
            ),
            token::IF => self.parse_if_expression().map_or(
                Node::Nil, 
                |v| Node::IfExpression(v)
            ),
            _ => Node::Nil
        }
    }

    fn parse_if_expression(&mut self) -> Result<IfExpression, String> {
        // if (<condition>) { <consequence> } else { <alternative> }
        //    C P

        if !self.expect_peek(token::LPAREN) {
            return Err(String::from("Missing ("));
        }

        self.next_token();
        let condition = self.parse_expression(PrecedenceType::LOWEST);

        // dbg!(&condition);
        // dbg!(&self.peek_token);

        if !self.expect_peek(token::RPAREN) {
            return Err(String::from("Missing ("));
        }

        if !self.expect_peek(token::LBRACE) {
            return Err(String::from("Missing ("));
        }

        let mut consequence: Vec<Node> = vec![];

        // body
        while !self.cur_token_is(token::RBRACE) && !self.cur_token_is(token::EOF) {
            let stmt = self.parse_statement();
            if !stmt.is_nil() {
                consequence.push(stmt);
            }
            self.next_token();
        }

        // else { }
        //    c p

        let mut alternative: Vec<Node> = vec![];


        if self.peek_token_is(token::ELSE) {
            self.next_token();
            dbg!(&self.cur_token);
            dbg!(&self.peek_token);
            if !self.expect_peek(token::LBRACE) {
                return Err(String::from("Missing ("));
            }

            // body
            while !self.cur_token_is(token::RBRACE) && !self.cur_token_is(token::EOF) {
                let stmt = self.parse_statement();
                if !stmt.is_nil() {
                    alternative.push(stmt);
                }
                self.next_token();
            }
        }


        // dbg!(&consequence);


        // dbg!("NEXT AFTER CONSEQUENCE");
        // dbg!(&self.cur_token);
        // dbg!(&self.peek_token);
        

        dbg!(Ok(IfExpression {
            condition: Rc::new(condition),
            consequence,
            alternative
        }))


        // Err(String::from("testing"))
    }

    fn call_infix_parser(&mut self, left: &Node) -> Node {
        match self.cur_token.token_type {
            token::PLUS | token::ASTERISK => self.parse_infix_expression(&left).map_or(
                Node::Nil, 
                |v| Node::Infix(v)
            ),
            token::LPAREN => self.parse_call_expression(&left).map_or(
                Node::Nil, 
                |v| Node::CallExpression(v)
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

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        self.next_token();
        let return_value = self.parse_expression(PrecedenceType::LOWEST);
        Some(ReturnStatement{
            value: Rc::new(return_value)
        })
    }

    fn parse_call_expression(&mut self, left: &Node) -> Result<CallExpression, String> {
        self.next_token(); // cur_token is '(' 

        let mut arguments: Vec<Node> = vec![];
        // if function has arguments
        if !self.cur_token_is(token::RPAREN) {
            // parse first agument
            let arg = self.parse_expression(PrecedenceType::LOWEST);
            arguments.push(arg);

            // while more than one argument
            while !self.peek_token_is(token::RPAREN) && !self.peek_token_is(token::EOF) {
                self.next_token();
                self.next_token();

                let arg = self.parse_expression(PrecedenceType::LOWEST);
                arguments.push(arg);
            }
        } 

        if !self.expect_peek(token::RPAREN) {
            return Err(String::from("Missing )"));
        }

        Ok(CallExpression { arguments, function: Rc::new(left.clone())})
    }

    fn parse_function_literal(&mut self) -> Option<FunctionLiteral> {
        if !self.expect_peek(token::LPAREN) {
            return None;
        }

        let mut parameters: Vec<Node> = vec![];

        if !self.peek_token_is(token::RPAREN) {
            self.next_token();
            let ident = self.parse_identifier().unwrap();
            parameters.push(Node::Ident(ident));

            while self.peek_token_is(token::COMMA) {
                self.next_token();
                self.next_token();
                // identifier here
                let ident = self.parse_identifier().unwrap();
                parameters.push(Node::Ident(ident));
            }
        }

        if !self.expect_peek(token::RPAREN) {
            return None;
        }

        if !self.expect_peek(token::LBRACE) {

            return None;
        }

        let mut stmts: Vec<Node> = vec![];

        // body
        while !self.cur_token_is(token::RBRACE) && !self.cur_token_is(token::EOF) {
            let stmt = self.parse_statement();
            if !stmt.is_nil() {
                stmts.push(stmt);
            }
            self.next_token();
        }

        Some(FunctionLiteral{
            parameters: parameters,
            body: Rc::new(Node::BlockStatement(BlockStatement {
                statements: stmts
            }))
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
            token::LPAREN => PrecedenceType::CALL,
            _ => PrecedenceType::LOWEST
        }
    }

    fn new_error(&mut self, message: &str) {
        self.errors.push(message.to_string());
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn it_should_parse_integer_expressions() {
        let input = "10; 5;";
        let tests = vec![10, 5];
        let mut parser = utils::setup_parser(input);
        let prog = match parser.parse_program() {
            Node::Program(prog) => Some(prog),
            _ => None
        }.unwrap();

        assert!(parser.errors.len() == 0, "parsing error");

        for (idx, stmt) in prog.statements.iter().enumerate() {
            match stmt {
                Node::Int(actual) => { 
                    // dbg!(format!("{} - {}", tests[idx], actual.0));
                    assert_eq!(tests[idx], actual.0)
                },
                _ => {
                    assert!(false, "expr not Node::Int");
                }
            }
        }
    }

    // #[test]
    // fn it_should_parse_function_literals() {
    //     let input = "
    //     let f = fn() { 10 };
    //     ";
    //     let mut parser = utils::setup_parser(input);
    //     let prog = match parser.parse_program() {
    //         Node::Program(prog) => Some(prog),
    //         _ => None
    //     }.unwrap();

    //     assert!(parser.errors.len() == 0, "parsing error");

    //     match &prog.statements[0] {
    //         Node::LetStatement(lt) => {
    //             match lt.value.borrow() {
    //                 Node::Function(f) => {
    //                     let f = f;
    //                     assert_eq!(f.parameters.len(), 0);
    //                     assert_eq!(f.body.statements.len(), 1);
    //                 },
    //                 _ => assert!(false, "expr not Node::Function")
    //             }
    //         }
    //         _ => {}
    //     }
    // }

    mod utils {
        use super::*;
        pub(super) fn setup_parser(input: &str) -> Parser {
            let mut lexer = Lexer::new(input);
            Parser::new(lexer)
        }
    }
}