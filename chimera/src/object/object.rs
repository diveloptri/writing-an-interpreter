pub type ObjectType = &'static str;

pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN:  &str = "BOOLEAN";
pub const NULL: &str = "NULL";

#[derive(Debug, PartialEq)]
pub enum Object {
    ReturnValue(Box<Object>),
    Integer(i64),
    Boolean(bool),
    Null,
}

impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::ReturnValue(_) => RETURN_VALUE_OBJ,
            Object::Integer(_) => INTEGER_OBJ,
            Object::Boolean(_) => BOOLEAN,
            Object::Null => NULL,
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::ReturnValue(val) => val.inspect(),
            Object::Integer(val) => val.to_string(),
            Object::Boolean(val) => val.to_string(),
            Object::Null => "null".to_string(),
        }
    }
}