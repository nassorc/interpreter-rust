#![allow(unused)]

mod token;
mod lexer;
mod parser;
mod ast;
// mod eval;

use std::{borrow::Borrow, collections::HashMap};
use std::rc::Rc;

use token::Token;
use lexer::Lexer;

use crate::ast::{Identifier, Integer, LetStatement, Node};
use parser::Parser;

fn print_node(node: &Node) {
    // dbg!(node);
}

fn main() {
    let mut lexer = Lexer::new("1 + 2 * 3");
    let mut parser = Parser::new(lexer);

    let prog = parser.parse_program();

    dbg!(&prog);


    // let evaluated = eval::eval_statements(prog.statements[0].clone()).unwrap();

    // println!("working");

    let mut statements: Vec<Node> = vec![];
    let l1 = LetStatement {
        name: Identifier(String::from("myVar")),
        value: Rc::new(Node::Int(Integer(1009)))
    };
    let expr1 = Integer(1010);
    statements.push(Node::LetStatement(l1));
    statements.push(Node::Int(expr1));

    // dbg!(&statements);


    for n in statements {
        match &n {
            Node::LetStatement(lt) => {
                print_node(&lt.value);
            },
            Node::Int(_) => {
                print_node(&n);
            }
            _ => {}
        }
    }




    // println!("len: {}", prog.statements.len());

    // if parser.errors.len() > 0 {
    //     for err in parser.errors {
    //         println!("{}", err);
    //     }
    // }

    // dbg!(&prog.statements[0]);
    // eval::eval()
    // for node in prog.statements {
    // }

}