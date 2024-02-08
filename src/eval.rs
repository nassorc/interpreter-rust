use std::{borrow::Borrow, rc::Rc};

use crate::{
    object::*,
    ast::*
};

pub fn eval(node: &Node, env: &mut Environment) -> Option<Object> {
    match node {
        Node::Program(p) => eval_statements(&p.statements, env),
        Node::LetStatement(v) => eval_let_statement(v, env),
        Node::Prefix(v) => eval_prefix(v, env),
        Node::Infix(v) => eval_infix(v, env),
        Node::Int(v) => Some(Object::Integer(IntegerObject{value: v.0})),
        Node::Boolean(v) => Some(Object::Boolean(BooleanObject{value: v.0})),
        Node::Ident(v) => eval_identifier(v, env),
        _ => None
    }
}

fn eval_statements(statements: &[Node], env: &mut Environment) -> Option<Object> {
    // IntegerObject{ value: 100 }
    let mut result = None;
    for stmt in statements {
        result = eval(stmt, env);
    }
    result
}

fn eval_prefix(prefix: &PrefixExpression, env: &mut Environment) -> Option<Object> {
    match prefix.op.as_str() {
        "-" => {
            let right = eval(&prefix.right, env)?;
            if let Object::Integer(mut right) = right {
                right.value *= -1;
                Some(Object::Integer(right))
            } else {
                None
            }
        },
        // "!" => {
        //     let right = eval(&prefix.right, env)?;
        //     match right {
        //         Object::Boolean(b) => {

        //         },
        //         _ => return None
        //     }
        // },
        _ => None
    }
}

fn eval_infix(infix: &InfixExpression, env: &mut Environment) -> Option<Object> {

    let left = eval(&infix.left, env)?;
    let right = eval(&infix.right, env)?;

    match (left, right) {
        (Object::Integer(left), Object::Integer(right)) => eval_integer_infix_opr(&infix.op, left, right),
        _ => None
    }
}

fn eval_integer_infix_opr(op: &str, left: IntegerObject, right: IntegerObject) -> Option<Object>{
    match op {
        "+" => Some(Object::Integer(IntegerObject {
            value: left.value + right.value
        })),
        _ => None
    }
}

fn eval_identifier(ident: &Identifier, env: &mut Environment) -> Option<Object> {
    let value = env.store.get(&ident.0)?;
    let v: &Object = value.borrow();
    let v = v.clone();
    // Rc::try_unwrap(value.clone()).ok()
    Some(v)
    // let value = value.to_owned().clone();
}

fn eval_let_statement(stmt: &LetStatement, env: &mut Environment) -> Option<Object> {
    let value = eval(&stmt.value, env)?;
    env.store.insert(stmt.name.0.clone(), Rc::new(value));
    None
}