
use crate::{evaluator::evaluator::new_error, object::object::Object};

pub fn builtins(name: &str, args: &[Object]) -> Object {
    match name {
        "len" => len(args),
        _ => new_error(format!("unknown builtin: {}", name)),
    }
}

fn len(args: &[Object]) -> Object {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got = {}, want = 1", args.len()))
    }

    match &args[0] {
        Object::String(str) => Object::Integer(str.len() as i64),
        err => new_error(format!("argument to `len` not supported, got = {}", err.object_type()))
    }
}