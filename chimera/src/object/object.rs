pub type ObjectType = &'static str;

pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";

pub trait Object {
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

#[derive(Debug)]
pub struct Integer {
    pub value: i64
}

impl Object for Integer {
    fn obj_type(&self) -> ObjectType {
        return INTEGER_OBJ
    }

    fn inspect(&self) -> String {
        format!{"{}", self.value}
    }
}

pub struct Boolean {
    pub value: bool
}

impl Object for Boolean{
    fn obj_type(&self) -> ObjectType {
        return BOOLEAN_OBJ
    }

    fn inspect(&self) -> String {
        format!{"{}", self.value}
    }
}