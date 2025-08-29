use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::ast::{self, ExpressionType, StatementType};
use crate::evaluator::builtins::builtins;
use crate::object::environment::Environment;
use crate::object::object::{Object, ARRAY_OBJ, INTEGER_OBJ, STRING_OBJ};

pub fn eval(program: &ast::Program, env: &mut Environment) -> Object {
    eval_program(program, env)
}

fn eval_expression_type(expr: &ExpressionType, env: &mut Environment) -> Object {
    match expr {
        ExpressionType::Identifier(identifier) => eval_identifier(identifier, env),
        ExpressionType::IntegerLiteral(int_literal) => Object::Integer(int_literal.value),
        ExpressionType::StringLiteral(string_literal) => Object::String(string_literal.value.clone()),
        ExpressionType::Boolean(boolean) => native_bool_to_boolean_object(boolean.value),
        ExpressionType::PrefixExpression(prefix_expr) => {
            let right = eval_expression_type(&*prefix_expr.right, env);
            if is_error(&right) {
                return right
            }
            eval_prefix_expression(&prefix_expr.operator, right)
        },
        ExpressionType::InfixExpression(infix_expr) => {
            let left= eval_expression_type(&*infix_expr.left, env);
            if is_error(&left) {
                return left
            }

            let right = eval_expression_type(&*infix_expr.right, env);
            if is_error(&right) {
                return right
            }

            eval_infix_expression(&infix_expr.operator, left, right)
        },
        ExpressionType::IfExpression(if_expr) => eval_if_expression(if_expr, env),
        ExpressionType::FunctionLiteral(function_literal) => {
            Object::Function(
                function_literal.parameters.clone(),
                function_literal.body.clone(),
                Rc::new(RefCell::new(env.clone()))
            )
        },
        ExpressionType::CallExpression(call_expr) => {
            let function = eval_expression_type(&*call_expr.function, env);

            if is_error(&function) {
                return function
            };

            let arguments = eval_expression(&call_expr.arguments, env);
            if arguments.len() == 1 && is_error(&arguments[0]) {
                return arguments[0].clone();
            }
            apply_function(function, arguments)
        },
        ExpressionType::BlockStatement(block_stmt) => eval_block_statements(block_stmt, env),
        ExpressionType::ArrayLiteral(arr_lit) => {
            let elements = eval_expression(&arr_lit.elements, env);
            if elements.len() == 1 && is_error(&elements[0]) {
                return elements[0].clone();
            }
            Object::Array(elements)
        },
        ExpressionType::IndexExpression(idx_expr) => {
            let left = eval_expression_type(&idx_expr.left, env);
            if is_error(&left) {
                return left
            }

            let idx = eval_expression_type(&idx_expr.index, env);
            if is_error(&idx){
                return idx
            }

            eval_index_expression(left, idx)
        },
    }
}

fn eval_statement_type(stmt: &StatementType, env: &mut Environment) -> Object {
    match stmt {
        StatementType::Let(let_stmt) => {
            let node_name_val = let_stmt.name.value.clone();
            let let_val = match &let_stmt.value {
                Some(let_stmt_val) => eval_expression_type(let_stmt_val, env),
                None => Object::Null
            };

            if is_error(&let_val) {
                return let_val 
            }
            let result = let_val.clone();
            env.set(node_name_val, let_val);
            result
        },
        StatementType::Return(return_stmt) => {
            let return_val = match &return_stmt.return_value {
                Some(expr) => eval_expression_type(expr, env),
                None => Object::Null
            };

            if is_error(&return_val) {
                return return_val
            }

            Object::ReturnValue(Box::new(return_val))
        },
        StatementType::Expression(expr_stmt) => {
            if let Some(expr) = &expr_stmt.expression {
                eval_expression_type(expr, env)
            } else {
                Object::Null
            }
        }
    }
}

fn eval_program(program: &ast::Program, env: &mut Environment) -> Object {
    let mut result = Object::Null;
    for stmt in &program.statements {
        result = eval_statement_type(stmt, env);
        
        if is_error(&result) {
            return result
        }

        if let Object::ReturnValue(val) = result {
            return *val
        }
    }
    result
}

fn eval_block_statements(block: &ast::BlockStatement, env: &mut Environment) -> Object {
    let mut result = Object::Null;
    for stmt in &block.statements {
        result = eval_statement_type(stmt, env);

        if is_error(&result) {
            return result
        }

        if let Object::ReturnValue(_) = result {
            return result;
        }
    }
    result
}

fn native_bool_to_boolean_object(input: bool) -> Object {
    if input {
        return Object::Boolean(true)
    }
    Object::Boolean(false)
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => new_error(format!("unknown operator: {}{}", operator, right.object_type()))
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean(true) => Object::Boolean(false),
        Object::Boolean(false) => Object::Boolean(true),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false)
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(val) => Object::Integer(-val),
        _ => new_error(format!("unknown operator: -{}", right.object_type()))
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    let left_type = left.object_type();
    let right_type = right.object_type();

    if left_type != right_type {
        return new_error(format!("type mismatch: {} {} {}", left_type, operator, right_type))
    }

    if left_type == STRING_OBJ && right_type == STRING_OBJ {
        return eval_string_infix_expression(operator, left, right)
    }

    match (left, right, operator) {
        (Object::Integer(l), Object::Integer(r), _) => {
            eval_integer_infix_expression(operator, l, r)
        },
        (l, r, "==") => native_bool_to_boolean_object(l == r),
        (l, r, "!=") => native_bool_to_boolean_object(l != r),
        _ => new_error(format!("unknown operator: {} {} {}", left_type, operator, right_type))
    }
}

fn eval_integer_infix_expression(operator: &str, left: i64, right: i64) -> Object {
    match operator {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "<" => native_bool_to_boolean_object(left < right),
        ">" => native_bool_to_boolean_object(left > right),
        "==" => native_bool_to_boolean_object(left == right),
        "!=" => native_bool_to_boolean_object(left != right),
        _ => new_error(format!("unknown operator: {} {} {}", left, operator, right))
    }
}

fn eval_if_expression(if_expr: &ast::IfExpressionWrapped, env: &mut Environment) -> Object {
    let condition = eval_expression_type(&*if_expr.condition, env);
    if is_error(&condition) {
        return condition
    }

    if is_truthy(condition) {
        return eval_block_statements(&if_expr.consequence, env)
    } else if let Some(alternative) = &if_expr.alternative {
        return eval_block_statements(alternative, env)
    } else {
        return Object::Null
    }
}

fn eval_identifier(node: &ast::Identifier, env: &mut Environment) -> Object {
    match env.get(&node.value) {
        Some(value) => value.clone(),
        None => {
            if is_builtin(&node.value) {
                Object::Builtin(node.value.to_string())
            } else {
                new_error(format!("identifier not found: {}", node.value))
            }
        },
    }
}

fn eval_expression(expressions: &[ExpressionType], env: &mut Environment) -> Vec<Object> {
    let mut result = Vec::new();

    for expr in expressions.iter() {
        let evaluated = eval_expression_type(expr, env);

        if is_error(&evaluated) {
            return vec![evaluated]
        }

        result.push(evaluated);
    }
    result
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Null => false,
        Object::Boolean(true) => true,
        Object::Boolean(false) => false,
        _ => true
    }
}

pub fn new_error(message: String) -> Object {
    Object::Error(message)
}

fn is_error(object: &Object) -> bool {
    matches!(object, Object::Error(_))
}

fn is_builtin(name: &str) -> bool {
    matches!(name, "len")
}

fn apply_function(function: Object, args: Vec<Object>) -> Object {
    match function {
        Object::Function(params, body, env_rc ) => {
            let mut extended_env = extend_function_env(&params, args, env_rc);
            let evaluated = eval_block_statements(&body, &mut extended_env);
            unwrap_return_value(evaluated)
        },
        Object::Builtin(str) => builtins(&str, &args),
        _ => new_error(format!("not a function: {}", function.object_type()))
    }
}

fn extend_function_env(params: &[ast::Identifier], args: Vec<Object>, outer_env: Rc<RefCell<Environment>>) -> Environment {
    let mut extended_env = Environment::new_enclosed(outer_env.borrow().clone());

    for (param_idx, param) in params.iter().enumerate() {
        if let Some(arg) = args.get(param_idx) {
            extended_env.set(param.value.clone(), arg.clone());
        }
    }
    extended_env
}

fn unwrap_return_value(object: Object) -> Object {
    match object {
        Object::ReturnValue(return_val) => *return_val,
        _ => object
    }
}

fn eval_string_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    if operator != "+" {
        return new_error(format!("unknown operator: {} {} {}", 
            left.object_type(), operator, right.object_type()
        ))
    }

    match (left, right) {
        (Object::String(l), Object::String(r)) => Object::String(format!("{}{}", l, r)),
        _ => Object::Null
    }
}

fn eval_index_expression(left: Object, index: Object) -> Object {
    if left.object_type() == ARRAY_OBJ && index.object_type() == INTEGER_OBJ {
        return eval_array_index_expression(&left, &index)
    }
    new_error(format!("index operator not supported: {}", left.object_type()))
}

fn eval_array_index_expression(array: &Object, index: &Object) -> Object {
    match (array, index) {
        (Object::Array(elements), Object::Integer(idx)) => {
            if elements.is_empty() {
                return Object::Null
            }

            let max = (elements.len() - 1) as i64;
            if *idx < 0 || *idx > max {
                return Object::Null
            }
            elements[*idx as usize].clone()
        },
        _ => Object::Null
    }

}