use crate::ast::{Expression, LetStatement, Statement};

pub trait Object {
    fn inspect(&self) -> String;
}


#[derive(Debug)]
struct Null {}
impl Object for Null {
    fn inspect(&self) -> String {
        "null".to_string()
    }
}

#[derive(Debug)]
struct IntegerObject {
    value: i32,
}
impl Object for IntegerObject {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

const NULL: Null = Null{};


pub fn eval_statements(node: Statement) -> Option<impl Object> {
    match node {
        Statement::LetStatement(lt) => Some(eval_let_statement(lt)),
        // Statement::ExpressionStatement(exprs) => Some(eval_expression(exprs)),
        _ => None
    }
}

pub fn eval_expression(node: Expression) -> impl Object {
    match node {
        Expression::Int(v) => IntegerObject{ value: v.0 },
        _ => IntegerObject{ value: -1 }
    }
}

pub fn eval_let_statement(node: LetStatement) -> impl Object { 
    // eval_expression(node.value)
}