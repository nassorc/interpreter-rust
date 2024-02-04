#[derive(Debug, Clone)]
pub enum NodeType {
    LetStatement(LetStatement),
    ExpressionStatement(ExpressionType),
    Nil
}

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Ident(Identifier),
    Int(Integer),
    Nil
}

#[derive(Clone)]
pub struct Program {
    pub statements: Vec<NodeType>
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: ExpressionType
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);


#[derive(Debug, Clone)]
pub struct Integer(pub i32);