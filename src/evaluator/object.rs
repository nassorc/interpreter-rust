use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use crate::parser::ast;

trait ObjectVariant {
    fn inspect(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(IntegerObject),
    Boolean(BooleanObject),
    Identifier(IdentiferObject),
    Function(FunctionLiteralObject),
    Return(ReturnObject),
    ObjectRef(Rc<RefCell<Object>>),
    Null,
}

#[derive(Debug, Clone)]
pub struct IntegerObject {
    pub value: i32,
}

impl ObjectVariant for IntegerObject {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct BooleanObject {
    pub value: bool,
}

impl ObjectVariant for BooleanObject {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct IdentiferObject {
    pub value: String,
}

impl ObjectVariant for IdentiferObject {
    fn inspect(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionLiteralObject {
    // fn ( <Identifer[]> ) { <BlockStatement>}
    pub parameters: Vec<ast::Node>,
    pub body: ast::Node,
}

impl ObjectVariant for FunctionLiteralObject {
    fn inspect(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ReturnObject {
    pub value: Rc<Object>,
}

impl ObjectVariant for ReturnObject {
    fn inspect(&self) -> String {
        todo!()
    }
}

pub const TRUE: BooleanObject = BooleanObject { value: true };
pub const FALSE: BooleanObject = BooleanObject { value: false };
