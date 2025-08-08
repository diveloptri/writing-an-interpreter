use crate::{ast::ast, object::object::{self, Object}};


pub const TRUE: Object = object::Object::Boolean(true);
pub const FALSE: Object = object::Object::Boolean(false);
pub const NULL: Object = object::Object::Null;

enum NodeType<'a> {
    Program(&'a ast::Program),
    ExpressionStatement(&'a ast::ExpressionStatement),
    IntegerLiteral(&'a ast::IntegerLiteral),
    Boolean(&'a ast::Boolean),
    Unknown,
}

fn classify_node(node: &dyn ast::Node) -> NodeType<'_> {
    if let Some(program) = node.as_any().downcast_ref::<ast::Program>() {
        NodeType::Program(program)
    } else if let Some(expr_stmt) = node.as_any().downcast_ref::<ast::ExpressionStatement>() {
        NodeType::ExpressionStatement(expr_stmt)
    } else if let Some(int_literal) = node.as_any().downcast_ref::<ast::IntegerLiteral>() {
        NodeType::IntegerLiteral(int_literal)
    } else if let Some(boolean) = node.as_any().downcast_ref::<ast::Boolean>() {
        NodeType::Boolean(boolean)
    } else {
        NodeType::Unknown
    }
}

pub fn eval(node: &dyn ast::Node) -> object::Object {
    match classify_node(node) {
        NodeType::Program(program) => eval_statements(&program.statements),
        NodeType::ExpressionStatement(expr_stmt) => {
            if let Some(expr) = &expr_stmt.expression {
                eval(&**expr)
            } else {
                object::Object::Null
            }
        },
        NodeType::IntegerLiteral(int_literal) => object::Object::Integer(int_literal.value),
        NodeType::Boolean(boolean) => native_bool_to_boolean_object(boolean.value),
        NodeType::Unknown => object::Object::Null
    }
}

fn eval_statements(stmts: &[Box<dyn ast::Statement>]) -> object::Object {
    stmts.iter()
        .map(|stmt| eval(&**stmt))
        .last()
        .unwrap_or(Object::Null)
}

fn native_bool_to_boolean_object(input: bool) -> object::Object {
    if input {
        return TRUE
    }
    FALSE
}