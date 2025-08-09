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
            EvalIntegerExpressionTest{input: "-10", expected: -10},
            EvalIntegerExpressionTest{input: "5 + 5 + 5 + 5 - 10", expected: 10},
            EvalIntegerExpressionTest{input: "2 * 2 * 2 * 2 * 2", expected: 32},
            EvalIntegerExpressionTest{input: "-50 + 100 + -50", expected: 0},
            EvalIntegerExpressionTest{input: "5 * 2 + 10", expected: 20},
            EvalIntegerExpressionTest{input: "5 + 2 * 10", expected: 25},
            EvalIntegerExpressionTest{input: "20 + 2 * -10", expected: 0},
            EvalIntegerExpressionTest{input: "50 / 2 * 2 + 10", expected: 60},
            EvalIntegerExpressionTest{input: "2 * (5 + 10)", expected: 30},
            EvalIntegerExpressionTest{input: "3 * 3 * 3 + 10", expected: 37},
            EvalIntegerExpressionTest{input: "3 * (3 * 3) + 10", expected: 37},
            EvalIntegerExpressionTest{input: "(5 + 10 * 2 + 15 / 3) * 2 + -10", expected: 50},
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
            EvalBoolExpressionTest{input: "1 < 2", expected: true},
            EvalBoolExpressionTest{input: "1 > 2", expected: false},
            EvalBoolExpressionTest{input: "1 < 1", expected: false},
            EvalBoolExpressionTest{input: "1 > 1", expected: false},
            EvalBoolExpressionTest{input: "1 == 1", expected: true},
            EvalBoolExpressionTest{input: "1 != 1", expected: false},
            EvalBoolExpressionTest{input: "1 == 2", expected: false},
            EvalBoolExpressionTest{input: "1 != 2", expected: true},
            EvalBoolExpressionTest{input: "true == true", expected: true},
            EvalBoolExpressionTest{input: "false == false", expected: true},
            EvalBoolExpressionTest{input: "true == false", expected: false},
            EvalBoolExpressionTest{input: "true != false", expected: true},
            EvalBoolExpressionTest{input: "false != true", expected: true},
            EvalBoolExpressionTest{input: "(1 < 2) == true", expected: true},
            EvalBoolExpressionTest{input: "(1 < 2) == false", expected: false},
            EvalBoolExpressionTest{input: "(1 > 2) == true", expected: false},
            EvalBoolExpressionTest{input: "(1 > 2) == false", expected: true},
        ];

        for test in eval_bool_expression_tests.iter() {
            let evaluated = test_eval(test.input.to_string());
            assert!(
                test_boolean_object(evaluated, test.expected),
            );
        }
    }

    #[test]
    fn test_bang_operator() {
        struct BangOperatorTest {
            input: &'static str,
            expected: bool,
        }

        let bang_operator_tests: Vec<BangOperatorTest> = vec![
            BangOperatorTest{input: "!true", expected: false},
            BangOperatorTest{input: "!false", expected: true},
            BangOperatorTest{input: "!5", expected: false},
            BangOperatorTest{input: "!!true", expected: true},
            BangOperatorTest{input: "!!false", expected: false},
            BangOperatorTest{input: "!!5", expected: true},
        ];

        for test in bang_operator_tests.iter() {
            let evaluated = test_eval(test.input.to_string());
            assert!(
                test_boolean_object(evaluated, test.expected),
            );
        }
    }
}