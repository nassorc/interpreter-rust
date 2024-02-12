use std::{ borrow::Borrow, cell::{Ref, RefCell}, collections::HashMap, rc::Rc };

use crate::ast;

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
    Null
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
    pub value: Rc<Object>
}

impl ObjectVariant for ReturnObject {
    fn inspect(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub struct Environment {
    pub store: HashMap<String, Rc<RefCell<Object>>>,
    pub outer: Option<Rc<RefCell<Environment>>>
}

impl Environment {
    /// Creates an empty `Enviroment`.
    /// 
    /// The outer environment will be initially set to `None`.`
    pub fn new() -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(
            Environment { store: HashMap::new(), outer: None }
        ))
    }
    // Creates an empty `Environment` that extends and existing 
    // outer environment.
    pub fn new_extended(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(
            Environment { 
                store: HashMap::new(), 
                outer: Some(outer) }
        ))
    }
    /// Returns a refernce to an object corresponding to a key.
    /// If key is not defined in the current environment, it will
    /// then recursively attempt to retrieve the value from the outer scope.
    pub fn get(& self, k: String) -> Option<Rc<RefCell<Object>>> {
        // Rc::clone(&self.store.get(&k).unwrap_or_else(|| {
        //     self.outer.as_deref().unwrap().borrow().get(&k).unwrap()
        // }));
        // Some(())
        if self.store.contains_key(&k) {
            return Some(Rc::clone(
                &self.store.get(&k).unwrap()
            ));
        } else {
            return Some(Rc::clone(
                // TODO!: NOT SURE IF CHAINING A CLONE IS CORRECT.
                // TODO!: without the clone, compiler complains that we are 
                // TODO!: MOVISING the value out of the reference.
                &(*self.outer.clone().unwrap()).borrow().get(k).unwrap()
            ));
        }
        None
    }

    pub fn insert(&mut self, k: String, v: Object) -> Option<Rc<RefCell<Object>>> {
        self.store.insert(k, Rc::new(RefCell::new(v)))
    } 
}

pub const TRUE: BooleanObject = BooleanObject{value: true};
pub const FALSE: BooleanObject = BooleanObject{value: false};