
use crate::{evaluator::evaluator::new_error, object::object::{Object, ARRAY_OBJ}};

pub fn builtins(name: &str, args: &[Object]) -> Object {
    match name {
        "len" => len(args),
        "first" => first(args),
        "last" => last(args),
        "rest" => rest(args),
        "push" => push(args),
        "println" => println(args),
        _ => new_error(format!("unknown builtin: {}", name)),
    }
}

fn len(args: &[Object]) -> Object {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got = {}, want = 1", args.len()))
    }

    match &args[0] {
        Object::String(str) => Object::Integer(str.chars().count() as i64),
        Object::Array(elements) => Object::Integer(elements.len() as i64),
        err => new_error(format!("argument to `len` not supported, got = {}", err.object_type()))
    }
}

fn first(args: &[Object]) -> Object {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got = {}, want = 1", args.len()))
    }

    if args[0].object_type() != ARRAY_OBJ {
        return new_error(format!("argument to `first` must be ARRAY, got = {}", args[0].object_type()))
    };

    match &args[0] {
        Object::Array(elements) => {
            if elements.len() > 0 {
                return elements[0].clone()
            }
            return Object::Null
        },
        _ => Object::Null
    }
}

fn last(args: &[Object]) -> Object {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got = {}, want = 1", args.len()))
    }

    if args[0].object_type() != ARRAY_OBJ {
        return new_error(format!("argument to `last` must be ARRAY, got = {}", args[0].object_type()))
    };

    match &args[0] {
        Object::Array(elements) => {
            let length = elements.len();
            if elements.len() > 0 {
                return elements[length - 1].clone()
            }
            return Object::Null
        },
        _ => Object::Null
    }

}

fn rest(args: &[Object]) -> Object {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got = {}, want = 1", args.len()))
    }

    if args[0].object_type() != ARRAY_OBJ {
        return new_error(format!("argument to `rest` must be ARRAY, got = {}", args[0].object_type()))
    };

    match &args[0] {
        Object::Array(elements) => {
            if elements.is_empty() {
                return Object::Null
            }
            return Object::Array(elements[1..].to_vec())
        },
        _ => Object::Null
    }
}

fn push(args: &[Object]) -> Object {
    if args.len() != 2 {
        return new_error(format!("wrong number of arguments. got = {}, want = 2", args.len()))
    }

    if args[0].object_type() != ARRAY_OBJ {
        return new_error(format!("argument to `push` must be ARRAY, got = {}", args[0].object_type()))
    };

    match &args[0] {
        Object::Array(elements) => {
            let mut new_elements = elements.clone();
            new_elements.push(args[1].clone());
            return Object::Array(new_elements.to_vec())
        },
        _ => Object::Null
    }
}

fn println(args: &[Object]) -> Object {
    args.iter().for_each(|value| println!("{}", value.inspect()));
    Object::Null
}