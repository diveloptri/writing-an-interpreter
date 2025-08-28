use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::ast::{self, Node};
use crate::object::environment::Environment;

pub type ObjectType = &'static str;

pub const FUNCTION_OBJ: &str = "FUNCTION";
pub const ERROR_OBJ: &str = "ERROR";
pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN:  &str = "BOOLEAN";
pub const NULL: &str = "NULL";
pub const STRING_OBJ: &str = "STRING";
pub const BUILTIN_OBJ: &str = "BUILTIN";
pub const ARRAY_OBJ: &str = "ARRAY";

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    String(String),
    Boolean(bool),
    Null,
    Error(String),
    Function(Vec<ast::Identifier>, ast::BlockStatement, Rc<RefCell<Environment>>),
    ReturnValue(Box<Object>),
    Builtin(String),
    Array(Vec<Object>),
}
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(a), Object::Integer(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Null, Object::Null) => true,
            (Object::Error(a), Object::Error(b)) => a == b,
            (Object::Function(..), Object::Function(..)) => false,
            (Object::Builtin(_), Object::Builtin(_)) => false,
            (Object::Array(a), Object::Array(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b)
            },
            _ => false,
        }
    }
}
impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(_) => INTEGER_OBJ,
            Object::String(_) => STRING_OBJ,
            Object::Boolean(_) => BOOLEAN,
            Object::Null => NULL,
            Object::Error(_) => ERROR_OBJ,
            Object::Function(..) => FUNCTION_OBJ,
            Object::ReturnValue(_) => RETURN_VALUE_OBJ,
            Object::Builtin(_) => BUILTIN_OBJ,
            Object::Array(_) => ARRAY_OBJ,
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(val) => val.to_string(),
            Object::String(val) => val.to_string(),
            Object::Boolean(val) => val.to_string(),
            Object::Null => "null".to_string(),
            Object::Error(val) => { format!("ERROR: {}", val)},
            Object::Function(parameters, body, _)=> { 
                format!(
                    "fn({:?}) {{\n {} \n}}",
                    parameters.iter().map(|val| format!("{},", val.string())), &*body.string()
                )
            },
            Object::ReturnValue(val) => val.inspect(),
            Object::Builtin(_) => "builtin function".to_string(),
            Object::Array(elements) => {
                format!(
                    "[{}]",
                    elements.iter().map(|element| element.inspect()).collect::<Vec<String>>().join(", ")
                )
            },
        }
    }
}