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

#[derive(Debug, Clone)]
pub enum Object {
    Function(Vec<ast::Identifier>, ast::BlockStatement, Rc<RefCell<Environment>>),
    Error(String),
    ReturnValue(Box<Object>),
    Integer(i64),
    Boolean(bool),
    Null,
}
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(a), Object::Integer(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Null, Object::Null) => true,
            (Object::Error(a), Object::Error(b)) => a == b,
            (Object::Function(..), Object::Function(..)) => false,
            _ => false,
        }
    }
}
impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Function(..) => FUNCTION_OBJ,
            Object::Error(_) => ERROR_OBJ,
            Object::ReturnValue(_) => RETURN_VALUE_OBJ,
            Object::Integer(_) => INTEGER_OBJ,
            Object::Boolean(_) => BOOLEAN,
            Object::Null => NULL,
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Function(parameters, body, _)=> { 
                format!("fn({:?}) {{\n {} \n}}",
                parameters.iter().map(|val| format!("{},", val.string())), &*body.string())
            },
            Object::Error(val) => { format!("ERROR: {}", val)},
            Object::ReturnValue(val) => val.inspect(),
            Object::Integer(val) => val.to_string(),
            Object::Boolean(val) => val.to_string(),
            Object::Null => "null".to_string(),
        }
    }
}