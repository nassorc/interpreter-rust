pub mod ast;

use std::{collections::HashMap, rc::Rc};

use crate::{lexer, lexer::token};
use ast::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PrecedenceType {
    LOWEST = 0,
    EQUALS,
    LESSGREATER,
    ADD,
    PRODUCT,
    PREFIX,
    CALL,
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
            cur_token: token::Token {
                token_type: token::EOF,
                literal: "\0".to_string(),
            },
            peek_token: token::Token {
                token_type: token::EOF,
                literal: "\0".to_string(),
            },
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
            token::LET => self
                .parse_let_statement()
                .map_or(Node::Nil, |v| Node::LetStatement(v)),
            token::RETURN => self
                .parse_return_statement()
                .map_or(Node::Nil, |v| Node::ReturnStatement(v)),
            _ => self.parse_expression_statement(),
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
            token::INT => self.parse_integer().map_or(Node::Nil, |v| Node::Int(v)),
            token::TRUE | token::FALSE => {
                self.parse_boolean().map_or(Node::Nil, |v| Node::Boolean(v))
            }
            token::IDENTIFIER => self
                .parse_identifier()
                .map_or(Node::Nil, |v| Node::Ident(v)),
            token::FUNCTION => self
                .parse_function_literal()
                .map_or(Node::Nil, |v| Node::Function(v)),
            token::MINUS | token::BANG => self
                .parse_prefix_expression()
                .map_or(Node::Nil, |v| Node::Prefix(v)),
            token::IF => self
                .parse_if_expression()
                .map_or(Node::Nil, |v| Node::IfExpression(v)),
            token::LPAREN => {
                self.next_token();
                let group = self.parse_expression(PrecedenceType::LOWEST);
                if !self.expect_peek(token::RPAREN) {
                    return Node::Nil;
                }

                return group;
            }
            _ => Node::Nil,
        }
    }

    fn parse_if_expression(&mut self) -> Result<IfExpression, String> {
        if !self.expect_peek(token::LPAREN) {
            return Err(String::from("Missing ("));
        }

        self.next_token();
        let condition = self.parse_expression(PrecedenceType::LOWEST);

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

        let mut alternative: Vec<Node> = vec![];

        if self.peek_token_is(token::ELSE) {
            self.next_token();
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

        Ok(IfExpression {
            condition: Rc::new(condition),
            consequence,
            alternative,
        })
    }

    fn call_infix_parser(&mut self, left: &Node) -> Node {
        match self.cur_token.token_type {
            token::PLUS
            | token::MINUS
            | token::ASTERISK
            | token::SLASH
            | token::EQ
            | token::NOTEQ
            | token::LT
            | token::GT => self
                .parse_infix_expression(&left)
                .map_or(Node::Nil, |v| Node::Infix(v)),
            token::LPAREN => self
                .parse_call_expression(&left)
                .map_or(Node::Nil, |v| Node::CallExpression(v)),
            _ => Node::Nil,
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        self.next_token(); // advance token to identifier
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
            value: Rc::new(value),
        })
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        self.next_token();
        let return_value = self.parse_expression(PrecedenceType::LOWEST);
        Some(ReturnStatement {
            value: Rc::new(return_value),
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

            if !self.expect_peek(token::RPAREN) {
                return Err(String::from("Missing )"));
            }
        }

        Ok(CallExpression {
            arguments,
            function: Rc::new(left.clone()),
        })
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

        Some(FunctionLiteral {
            parameters: parameters,
            body: Rc::new(Node::BlockStatement(BlockStatement { statements: stmts })),
        })
    }

    fn parse_integer(&self) -> Result<Integer, String> {
        let value = self
            .cur_token
            .literal
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
        let right = self.parse_expression(PrecedenceType::PREFIX);

        Ok(PrefixExpression {
            op: cur_tk.literal.clone(),
            right: Rc::new(right),
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
        self.errors.push(format!(
            "Expected peek_token to be {}, got {}",
            tk, self.peek_token.token_type
        ));
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
            token::LPAREN => PrecedenceType::CALL,
            token::ASTERISK | token::SLASH => PrecedenceType::PRODUCT,
            token::PLUS | token::MINUS => PrecedenceType::ADD,
            token::LT | token::GT => PrecedenceType::LESSGREATER,
            token::EQ | token::NOTEQ => PrecedenceType::EQUALS,
            _ => PrecedenceType::LOWEST,
        }
    }

    fn new_error(&mut self, message: &str) {
        self.errors.push(message.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::{lexer::Lexer, *};
    use crate::utils::setup;

    #[test]
    fn test_parsing_integers() {
        let input = "
            5;
            10;
        ";
        let input_size = utils::count_statements(input);
        let (parser, prog) = setup(&input);

        let Node::Program(prog) = prog else {
            assert!(false, "prog is not Node::Program");
            return;
        };

        utils::check_parser_errors(&parser);
        utils::assert_program_len(&prog, input_size);

        let tests = vec![5, 10];

        for (idx, test) in tests.iter().enumerate() {
            let actual = prog.statements.get(idx).unwrap();
            utils::assert_integer_type(actual, *test);
        }
    }

    #[test]
    fn test_parsing_booleans() {
        let input = "
            true;
            false;
        ";
        let input_size = utils::count_statements(input);
        let (parser, prog) = setup(&input);

        let Node::Program(prog) = prog else {
            assert!(false, "prog is not Node::Program");
            return;
        };

        utils::check_parser_errors(&parser);
        utils::assert_program_len(&prog, input_size);

        let tests = vec![true, false];

        for (idx, test) in tests.iter().enumerate() {
            let actual = prog.statements.get(idx).unwrap();
            match actual {
                Node::Boolean(actual) => {
                    assert_eq!(&actual.0, test);
                }
                _ => {
                    assert!(false, "expected Node::Boolean, got=.");
                }
            }
        }
    }

    #[test]
    fn test_parsing_identifiers() {
        let input = "
            a;
            myVar;
        ";
        let input_size = utils::count_statements(input);
        let (parser, prog) = setup(&input);

        let Node::Program(prog) = prog else {
            assert!(false, "prog is not Node::Program");
            return;
        };

        utils::check_parser_errors(&parser);
        utils::assert_program_len(&prog, input_size);

        let tests = vec!["a", "myVar"];

        for (idx, test) in tests.iter().enumerate() {
            let actual = prog.statements.get(idx).unwrap();
            match actual {
                Node::Ident(actual) => {
                    assert_eq!(&actual.0, test);
                }
                _ => {
                    assert!(false, "expected Node::Ident, got=.");
                }
            }
        }
    }

    #[test]
    fn test_parsing_prefix_expression() {
        let input = "
            -5;
            !10;
        ";
        let input_size = utils::count_statements(input);
        let (parser, prog) = setup(&input);

        let Node::Program(prog) = prog else {
            assert!(false, "prog is not Node::Program");
            return;
        };

        utils::check_parser_errors(&parser);
        utils::assert_program_len(&prog, input_size);

        let tests = vec![("-", 5), ("!", 10)];

        for (idx, test) in tests.iter().enumerate() {
            let actual = prog.statements.get(idx).unwrap();
            match actual {
                Node::Prefix(actual) => {
                    assert_eq!(&actual.op, test.0);
                    utils::assert_integer_type(&actual.right.as_ref(), test.1);
                }
                _ => {
                    assert!(false, "expected Node::Ident, got=.");
                }
            }
        }
    }

    #[test]
    fn test_parsing_integer_infix_expressions() {
        let input = "
            10 + 5;
            10 - 5;
            10 * 5;
            10 / 5;
            10 < 5;
            10 > 5;
            10 == 5;
            10 == 10;
            10 != 5;
            10 != 10;
        ";
        let input_size = utils::count_statements(input);
        let (parser, prog) = setup(&input);

        let Node::Program(prog) = prog else {
            assert!(false, "prog is not Node::Program");
            return;
        };

        utils::check_parser_errors(&parser);
        utils::assert_program_len(&prog, input_size);

        let tests = vec![
            (10, "+", 5),
            (10, "-", 5),
            (10, "*", 5),
            (10, "/", 5),
            (10, "<", 5),
            (10, ">", 5),
            (10, "==", 5),
            (10, "==", 10),
            (10, "!=", 5),
            (10, "!=", 10),
        ];

        for (idx, test) in tests.iter().enumerate() {
            let actual = prog.statements.get(idx).unwrap();
            match actual {
                Node::Infix(actual) => {
                    assert_eq!(&actual.op, test.1);

                    utils::assert_integer_type(&actual.left.as_ref(), test.0);
                    utils::assert_integer_type(&actual.right.as_ref(), test.2);
                }
                _ => {
                    assert!(false, "expected Node::Ident, got=.");
                }
            }
        }
    }

    #[test]
    fn test_parsing_boolean_infix_expressions() {
        let input = "
            true == true;
            true != false;
        ";
        let input_size = utils::count_statements(input);
        let (parser, prog) = setup(&input);

        let Node::Program(prog) = prog else {
            assert!(false, "prog is not Node::Program");
            return;
        };

        utils::check_parser_errors(&parser);
        utils::assert_program_len(&prog, input_size);

        let tests = vec![(true, "==", true), (true, "!=", false)];

        for (idx, test) in tests.iter().enumerate() {
            let actual = prog.statements.get(idx).unwrap();
            match actual {
                Node::Infix(actual) => {
                    assert_eq!(&actual.op, test.1);

                    utils::assert_boolean_type(&actual.left.as_ref(), test.0);
                    utils::assert_boolean_type(&actual.right.as_ref(), test.2);
                }
                _ => {
                    assert!(false, "expected Node::Ident, got=.");
                }
            }
        }
    }

    #[test]
    fn test_parsing_let_statements() {
        let input = "
        let a = 10;
        ";
        let input_size = utils::count_statements(input);
        let (parser, prog) = setup(&input);

        let Node::Program(prog) = prog else {
            assert!(false, "prog is not Node::Program");
            return;
        };

        utils::check_parser_errors(&parser);
        utils::assert_program_len(&prog, input_size);

        let tests = vec![("a", 10)];

        for (idx, test) in tests.iter().enumerate() {
            let actual = prog.statements.get(idx).unwrap();
            match actual {
                Node::LetStatement(actual) => {
                    assert_eq!(&actual.name.0, test.0);
                    utils::assert_integer_type(actual.value.as_ref(), test.1);
                }
                _ => {
                    assert!(false, "expected Node::Ident, got=.");
                }
            }
        }
    }

    #[test]
    fn test_parsing_return_statements() {
        let input = "
        return 10;
        ";
        let input_size = utils::count_statements(input);
        let (parser, prog) = setup(&input);

        let Node::Program(prog) = prog else {
            assert!(false, "prog is not Node::Program");
            return;
        };

        utils::check_parser_errors(&parser);
        utils::assert_program_len(&prog, input_size);

        let tests = vec![10];

        for (idx, test) in tests.iter().enumerate() {
            let actual = prog.statements.get(idx).unwrap();
            match actual {
                Node::ReturnStatement(actual) => {
                    utils::assert_integer_type(actual.value.as_ref(), *test);
                }
                _ => {
                    assert!(false, "expected Node::Return, got=.");
                }
            }
        }
    }

    #[test]
    fn test_parsing_function_literal() {
        struct FunctionTest<'a> {
            params: Vec<&'a str>,
            body_len: usize,
            test: &'a str,
        };

        let tests = vec![
            (
                "fn () {}",
                FunctionTest {
                    params: vec![],
                    body_len: 0,
                    test: "fn () {  }",
                },
            ),
            (
                "fn(a) { 10; }",
                FunctionTest {
                    params: vec!["a"],
                    body_len: 1,
                    test: "fn (a) { 10 }",
                },
            ),
            (
                "fn(x, y) { let myVar = 0; return x + y + myVar; }",
                FunctionTest {
                    params: vec!["x", "y"],
                    body_len: 2,
                    test: "fn (x, y) { let myVar = 0; return ((x + y) + myVar); }",
                },
            ),
        ];

        for test in tests {
            let input = test.0;
            let test = test.1;
            let (parser, prog) = setup(&input);

            assert_eq!(prog.to_string(), test.test);

            let Node::Program(prog) = prog else {
                assert!(false, "prog is not Node::Program");
                return;
            };

            utils::check_parser_errors(&parser);

            let Node::Function(f) = prog.statements.get(0).unwrap() else {
                assert!(prog.statements.get(0).is_none(), "Expected prog.statements.len()=1, got=0");
                return;
            };

            assert_eq!(test.params.len(), f.parameters.len());

            let Node::BlockStatement(body) = f.body.as_ref() else {
                assert!(prog.statements.get(0).is_none(), "Expected prog.statements[0] to be Node::BlockStatement, got=.");
                return;
            };

            assert_eq!(test.body_len, body.statements.len());
        }
    }

    #[test]
    fn test_call_expression() {
        let tests = vec![
            ("a();", "a()"),
            ("myFunc(a,b,c);", "myFunc(a, b, c)"),
            (
                "fn(x, y) { return x + y; }(10 + 2, -1);",
                "fn (x, y) { return (x + y); }((10 + 2), (-1))",
            ),
        ];
        for test in tests {
            let input = test.0;
            let test = test.1;
            let (parser, prog) = setup(&input);

            assert_eq!(prog.to_string(), test);

            let Node::Program(prog) = prog else {
                assert!(false, "prog is not Node::Program");
                return;
            };

            utils::check_parser_errors(&parser);
        }
    }

    #[test]
    fn test_precedence_correctness() {
        assert!(true);
        // TESTS FROM BOOK
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a * (b + c)", "(a * (b + c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];

        for test in tests {
            let input = test.0;
            let test = test.1;
            let (parser, prog) = setup(&input);

            assert_eq!(prog.to_string(), test);

            let Node::Program(prog) = prog else {
                assert!(false, "prog is not Node::Program");
                return;
            };

            utils::check_parser_errors(&parser);
        }
    }

    mod utils {
        use super::*;

        pub(super) fn assert_program_len(prog: &Program, expected: usize) {
            let actual_size = prog.statements.len();
            assert_eq!(
                actual_size, expected,
                "prog.statements does not contain 2 elements, got={}",
                actual_size
            );
        }

        pub(super) fn check_parser_errors(parser: &Parser) {
            if parser.errors.len() == 0 {
                return;
            }
            let mut error_msg = String::new();
            error_msg.push_str(format!("Parser has ${} errors\n", parser.errors.len()).as_str());

            for err in &parser.errors {
                error_msg.push_str(format!("parser error: {}", err).as_str());
            }

            assert!(false, "{}", error_msg);
        }

        pub(super) fn count_statements(stmts: &str) -> usize {
            stmts.split(";").count() - 1
        }

        pub(super) fn assert_integer_type(node: &Node, expected_value: i32) {
            match node {
                Node::Int(value) => {
                    assert_eq!(value.0, expected_value);
                }
                _ => {
                    assert!(false, "expected Node::Int, got=Node::.");
                }
            }
        }

        pub(super) fn assert_boolean_type(node: &Node, expected_value: bool) {
            match node {
                Node::Boolean(value) => {
                    assert_eq!(value.0, expected_value);
                }
                _ => {
                    assert!(false, "expected Node::Boolean, got=Node::.");
                }
            }
        }
    }
}
