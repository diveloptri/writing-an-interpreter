use std::collections::HashMap;
use crate::object::object::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Environment { store: HashMap::new(), outer: None }
    }

    pub fn new_enclosed(outer: Environment) -> Self {
        Environment { store: HashMap::new(), outer: Some(Box::new(outer))}
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        let object = self.store.get(name);
        if object.is_none() {
            if let Some(outer_env) = &self.outer {
                return outer_env.get(name)
            }
        }
        object
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}