use crate::ast::ast;
use crate::object::{object, object::Object};


pub const TRUE: Object = object::Object::Boolean(true);
pub const FALSE: Object = object::Object::Boolean(false);
pub const NULL: Object = object::Object::Null;

enum NodeType<'a> {
    Program(&'a ast::Program),
    ExpressionStatement(&'a ast::ExpressionStatement),
    IntegerLiteral(&'a ast::IntegerLiteral),
    Boolean(&'a ast::Boolean),
    PrefixExpression(&'a ast::PrefixExpression),
    InfixExpression(&'a ast::InfixExpression),
    BlockStatement(&'a ast::BlockStatement),
    IfExpression(&'a ast::IfExpression),
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
    } else if let Some(prefix_expr) = node.as_any().downcast_ref::<ast::PrefixExpression>() {
        NodeType::PrefixExpression(prefix_expr)
    } else if let Some(infix_expr) = node.as_any().downcast_ref::<ast::InfixExpression>() {
        NodeType::InfixExpression(infix_expr)
    } else if let Some(block_stmt) = node.as_any().downcast_ref::<ast::BlockStatement>() {
        NodeType::BlockStatement(block_stmt)
    } else if let Some(if_expr) = node.as_any().downcast_ref::<ast::IfExpression>() {
        NodeType::IfExpression(if_expr)
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
        NodeType::PrefixExpression(prefix_expr) => {
            let right = eval(&*prefix_expr.right);
            eval_prefix_expression(&prefix_expr.operator, right)
        },
        NodeType::InfixExpression(infix_expr) => {
            let left= eval(&*infix_expr.left);
            let right = eval(&*infix_expr.right);
            eval_infix_expression(&infix_expr.operator, left, right)
        },
        NodeType::BlockStatement(block_stmt) => eval_statements(&block_stmt.statements),
        NodeType::IfExpression(if_expr) => eval_if_expression(if_expr),
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

fn eval_prefix_expression(operator: &str, right: object::Object) -> object::Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => NULL
    }
}

fn eval_bang_operator_expression(right: object::Object) -> object::Object {
    match right {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        _ => FALSE
    }
}

fn eval_minus_prefix_operator_expression(right: object::Object) -> object::Object {
    match right {
        object::Object::Integer(val) => object::Object::Integer(-val),
        _ => Object::Null
    }
}

fn eval_infix_expression(operator: &str, left: object::Object, right: object::Object) -> object::Object {
    match (left, right, operator) {
        (object::Object::Integer(l), object::Object::Integer(r), _) => {
            eval_integer_infix_expression(operator, l, r)
        },
        (l, r, "==") => native_bool_to_boolean_object(l == r),
        (l, r, "!=") => native_bool_to_boolean_object(l != r),
        _ => NULL
    }
}

fn eval_integer_infix_expression(operator: &str, left: i64, right: i64) -> object::Object {
    match operator {
        "+" => object::Object::Integer(left + right),
        "-" => object::Object::Integer(left - right),
        "*" => object::Object::Integer(left * right),
        "/" => object::Object::Integer(left / right),
        "<" => native_bool_to_boolean_object(left < right),
        ">" => native_bool_to_boolean_object(left > right),
        "==" => native_bool_to_boolean_object(left == right),
        "!=" => native_bool_to_boolean_object(left != right),
        _ => Object::Null,
    }
}

fn eval_if_expression(if_expr: &ast::IfExpression) -> object::Object {
    let condition = eval(&*if_expr.condition);

    if is_truthy(condition) {
        return eval(&if_expr.consequence)
    } else if let Some(alternative) = &if_expr.alternative {
        return eval(alternative)
    } else {
        return NULL
    }
}

fn is_truthy(object: object::Object) -> bool {
    match object {
        NULL => false,
        TRUE => true,
        FALSE => false,
        _ => true
    }
}