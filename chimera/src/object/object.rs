use std::collections::HashMap;

pub type ObjectType = &'static str;

pub const ERROR_OBJ: &str = "ERROR";
pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN:  &str = "BOOLEAN";
pub const NULL: &str = "NULL";

pub struct Environment {
    store: HashMap<String, Object>
}

impl Environment {
    pub fn new() -> Self {
        Environment { store: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Error(String),
    ReturnValue(Box<Object>),
    Integer(i64),
    Boolean(bool),
    Null,
}

impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Error(_) => ERROR_OBJ,
            Object::ReturnValue(_) => RETURN_VALUE_OBJ,
            Object::Integer(_) => INTEGER_OBJ,
            Object::Boolean(_) => BOOLEAN,
            Object::Null => NULL,
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Error(val) => { format!("ERROR: {}", val)}
            Object::ReturnValue(val) => val.inspect(),
            Object::Integer(val) => val.to_string(),
            Object::Boolean(val) => val.to_string(),
            Object::Null => "null".to_string(),
        }
    }
}