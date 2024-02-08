use std::rc::Rc;
#[derive(Debug, Clone)]
pub enum Node {
    Program(Program),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(Expression),
    Ident(Identifier),
    Int(Integer),
    Boolean(Boolean),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Nil,
}

impl Node {
    pub fn is_nil(&self) -> bool {
        match self {
            Node::Nil => true,
            _ => false
        }
    }
}

pub trait ExpressionNode {}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(Expression),
    Nil
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident(Identifier),
    Int(Integer),
    // Prefix(PrefixExpression),
    // Infix(InfixExpression),
    Nil
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Node>
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Rc<Node>
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Expression
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Integer(pub i32);

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean(pub bool);

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub op: String,
    pub right: Rc<Node>
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub op: String,
    pub right: Rc<Node>,
    pub left: Rc<Node>
}