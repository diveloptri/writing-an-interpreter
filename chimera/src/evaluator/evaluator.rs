use crate::ast::ast::{self};
use crate::object::object;
use crate::object::object::Object;


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
    ReturnStatement(&'a ast::ReturnStatement),
    LetStatement(&'a ast::LetStatement),
    Identifier(&'a ast::Identifier),
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
    } else if let Some(return_stmt) = node.as_any().downcast_ref::<ast::ReturnStatement>() {
        NodeType::ReturnStatement(return_stmt)
    } else if let Some(let_stmt) = node.as_any().downcast_ref::<ast::LetStatement>() {
        NodeType::LetStatement(let_stmt)
    } else if let Some(identifier) = node.as_any().downcast_ref::<ast::Identifier>() {
        NodeType::Identifier(identifier)
    } else {
        NodeType::Unknown
    }
}

pub fn eval(node: &dyn ast::Node, env: &mut object::Environment) -> object::Object {
    match classify_node(node) {
        NodeType::Program(program) => eval_program(&program, env),
        NodeType::ExpressionStatement(expr_stmt) => {
            if let Some(expr) = &expr_stmt.expression {
                eval(&**expr, env)
            } else {
                object::Object::Null
            }
        },
        NodeType::IntegerLiteral(int_literal) => object::Object::Integer(int_literal.value),
        NodeType::Boolean(boolean) => native_bool_to_boolean_object(boolean.value),
        NodeType::PrefixExpression(prefix_expr) => {
            let right = eval(&*prefix_expr.right, env);
            if is_error(&right) {
                return right
            }
            eval_prefix_expression(&prefix_expr.operator, right)
        },
        NodeType::InfixExpression(infix_expr) => {
            let left= eval(&*infix_expr.left, env);
            if is_error(&left) {
                return left
            }

            let right = eval(&*infix_expr.right, env);
            if is_error(&right) {
                return right
            }

            eval_infix_expression(&infix_expr.operator, left, right)
        },
        NodeType::BlockStatement(block_stmt) => eval_block_statements(block_stmt, env),
        NodeType::IfExpression(if_expr) => eval_if_expression(if_expr, env),
        NodeType::ReturnStatement(return_stmt) => {
            let return_val = match &return_stmt.return_value {
                Some(expr) => eval(&**expr, env),
                None => NULL
            };

            if is_error(&return_val) {
                return return_val
            }

            object::Object::ReturnValue(Box::new(return_val))
        },
        NodeType::LetStatement(let_stmt) => {
            let node_name_val = let_stmt.name.value.clone();
            let let_val = match &let_stmt.value {
                Some(let_stmt_val) => eval(&**let_stmt_val, env),
                None => NULL
            };

            if is_error(&let_val) {
                return let_val 
            }
            let result = let_val.clone();
            env.set(node_name_val, let_val);
            result
        }
        NodeType::Identifier(identifier) => {
            eval_identifier(identifier, env)
        }
        NodeType::Unknown => object::Object::Null
    }
}

fn eval_program(program: &ast::Program, env: &mut object::Environment) -> object::Object {
    let mut result = Object::Null;
    for stmt in &program.statements {
        result = eval(&**stmt, env);
        
        if is_error(&result) {
            return result
        }

        if let Object::ReturnValue(val) = result {
            return *val
        }
    }
    result
}

fn eval_block_statements(block: &ast::BlockStatement, env: &mut object::Environment) -> object::Object {
    let mut result = Object::Null;
    for stmt in &block.statements {
        result = eval(&**stmt, env);

        if is_error(&result) {
            return result
        }

        if let Object::ReturnValue(_) = result {
            return result;
        }
    }
    result
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
        _ => new_error(format!("unknown operator: {}{}", operator, right.object_type()))
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
        _ => new_error(format!("unknown operator: -{}", right.object_type()))
    }
}

fn eval_infix_expression(operator: &str, left: object::Object, right: object::Object) -> object::Object {
    let left_type = left.object_type();
    let right_type = right.object_type();

    if left_type != right_type {
        return new_error(format!("type mismatch: {} {} {}", left_type, operator, right_type))
    }

    match (left, right, operator) {
        (object::Object::Integer(l), object::Object::Integer(r), _) => {
            eval_integer_infix_expression(operator, l, r)
        },
        (l, r, "==") => native_bool_to_boolean_object(l == r),
        (l, r, "!=") => native_bool_to_boolean_object(l != r),
        _ => new_error(format!("unknown operator: {} {} {}", left_type, operator, right_type))
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
        _ => new_error(format!("unknown operator: {} {} {}", left, operator, right))
    }
}

fn eval_if_expression(if_expr: &ast::IfExpression, env: &mut object::Environment) -> object::Object {
    let condition = eval(&*if_expr.condition, env);
    if is_error(&condition) {
        return condition
    }

    if is_truthy(condition) {
        return eval(&if_expr.consequence, env)
    } else if let Some(alternative) = &if_expr.alternative {
        return eval(alternative, env)
    } else {
        return NULL
    }
}

fn eval_identifier(node: &ast::Identifier, env: &mut object::Environment) -> object::Object {
    match env.get(&node.value) {
        Some(value) => value.clone(),
        None => new_error(format!("identifier not found: {}", node.value))
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

fn new_error(message: String) -> object::Object {
    Object::Error(message)
}

fn is_error(object: &Object) -> bool {
    matches!(object, Object::Error(_))
}