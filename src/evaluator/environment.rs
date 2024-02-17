use super::object::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Environment {
    pub store: HashMap<String, Rc<RefCell<Object>>>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// Creates an empty `Enviroment`.
    ///
    /// The outer environment will be initially set to `None`.`
    pub fn new() -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: None,
        }))
    }

    /// Creates an empty `Environment` that extends and existing environment.
    pub fn new_extended(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }))
    }

    /// Returns a refernce to an `Object`` corresponding to a key.
    /// If key is not defined in the current environment, it will
    /// then recursively call `self.outer.get`, and attempt  to
    /// retrieve the value from its outer environment.
    pub fn get(&self, k: String) -> Option<Rc<RefCell<Object>>> {
        self.store.get(&k).map_or_else(
            || match &self.outer.as_ref() {
                Some(v) => Some(Rc::clone(*v).as_ref().borrow().get(k.clone()).unwrap()),
                None => None,
            },
            |v| Some(Rc::clone(v)),
        )
    }

    pub fn insert(&mut self, k: String, v: Object) -> Option<Rc<RefCell<Object>>> {
        self.store.insert(k, Rc::new(RefCell::new(v)))
    }
}
