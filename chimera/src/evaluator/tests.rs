#[cfg(test)]
mod tests {
    use crate::lexer::lexer;
    use crate::object::object;
    use crate::parser::parser;
    use crate::evaluator::evaluator;

    fn test_eval(input: String) -> object::Object {
        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        evaluator::eval(&program)
    }

    fn test_integer_object(obj: object::Object, expected: i64) -> bool {
        match obj {
            object::Object::Integer(val) if val == expected => true,
            object::Object::Integer(val) => {
                    eprintln!("object has wrong value, got = {}, want = {}",
                        val, expected
                    );
                    false
                },
                
            fail => {
                eprintln!("object is not Object::Integer. got = {:?}", fail);
                false
            }
        }
    }

    #[test]
    fn test_eval_integer_expression() {
        struct EvalIntegerExpressionTest {
            input: &'static str,
            expected: i64,
        }

        let eval_integer_expression_tests: Vec<EvalIntegerExpressionTest> = vec![
            EvalIntegerExpressionTest{input: "5", expected: 5},
            EvalIntegerExpressionTest{input: "10", expected: 10},
        ];

        for test in eval_integer_expression_tests.iter() {
            let evaluated = test_eval(test.input.to_string());
            assert!(
                test_integer_object(evaluated, test.expected),
            );
        }
    }

    fn test_boolean_object(obj: object::Object, expected: bool) -> bool {
        match obj {
            object::Object::Boolean(val) if val == expected => true,
            object::Object::Boolean(val) => {
                    eprintln!("object has wrong value, got = {}, want = {}",
                        val, expected
                    );
                    false
                },
                
            fail => {
                eprintln!("object is not Object::Boolean. got = {:?}", fail);
                false
            }
        }
    }

    #[test]
    fn test_eval_bool_expression() {
        struct EvalBoolExpressionTest {
            input: &'static str,
            expected: bool,
        }

        let eval_bool_expression_tests: Vec<EvalBoolExpressionTest> = vec![
            EvalBoolExpressionTest{input: "true", expected: true},
            EvalBoolExpressionTest{input: "false", expected: false},
        ];

        for test in eval_bool_expression_tests.iter() {
            let evaluated = test_eval(test.input.to_string());
            assert!(
                test_boolean_object(evaluated, test.expected),
            );
        }
    }
}