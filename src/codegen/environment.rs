use std::collections::HashMap;

use crate::codegen::object::*;
use crate::parser::*;
use crate::parser::ast::*;

#[derive(Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        let mut store = HashMap::new();
        store.insert("null".to_string(), Object::Null);
        store.insert("void".to_string(), Object::Null);
        store.insert("printf".to_string(), Object::BuildIn(BuildIn::Printf));
        store.insert("length".to_string(), Object::BuildIn(BuildIn::Length));

        Environment { store: store }
    }

    pub fn get(&self, name: &str) -> Object {
        if let Some(obj) = self.store.get(name) {
            return obj.clone();
        };
        Object::Error(format!("{} is not found. row", name))
    }

    pub fn set(&mut self, name: String, value: Object) -> Object {
        self.store.insert(name, value.clone());
        value
    }
}
