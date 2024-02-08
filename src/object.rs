use std::{ collections::HashMap, rc::Rc };

trait ObjectVariant {
    fn inspect(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(IntegerObject),
    Boolean(BooleanObject),
    Identifier(IdentiferObject),
}

#[derive(Debug, Clone)]
pub struct IntegerObject {
    pub value: i32
}

impl ObjectVariant for IntegerObject {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct BooleanObject {
    pub value: bool
}

impl ObjectVariant for BooleanObject {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct IdentiferObject {
    pub value: String
}

impl ObjectVariant for IdentiferObject {
    fn inspect(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug)]
pub struct Environment {
  pub store: HashMap<String, Rc<Object>>  
}

impl Environment {
  pub fn new() -> Environment {
    Environment { store: HashMap::new() }
  }
}

const TRUE: BooleanObject = BooleanObject{value: true};
const FALSE: BooleanObject = BooleanObject{value: false};