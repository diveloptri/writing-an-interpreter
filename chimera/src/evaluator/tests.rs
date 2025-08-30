#[cfg(test)]
mod tests {
    use crate::ast::ast::Node;
    use crate::lexer::lexer;
    use crate::object::object::{self, HashKey, Object};
    use crate::object::environment::Environment;
    use crate::parser::parser;
    use crate::evaluator::evaluator;

    fn test_eval(input: String) -> object::Object {
        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();
        let mut environment = Environment::new();

        evaluator::eval(&program, &mut environment)
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

        let eval_integer_expression_tests = [
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

        let eval_bool_expression_tests = [
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

        let bang_operator_tests = [
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
    
    #[derive(Debug, Clone, PartialEq)]
    enum TestValue {
        Array(Vec<i64>),
        Integer(i64),
        Error(String),
        Null,
    }

    #[test]
    fn test_if_else_expression() {
        struct IfElseExpressionTest {
            input: &'static str,
            expected: TestValue
        }

        let if_else_expression_tests = [
            IfElseExpressionTest{input: "if (true) { 10 }", expected: TestValue::Integer(10)},
            IfElseExpressionTest{input: "if (false) { 10 }", expected: TestValue::Null},
            IfElseExpressionTest{input: "if (1) { 10 }", expected: TestValue::Integer(10)},
            IfElseExpressionTest{input: "if (1 < 2) { 10 }", expected: TestValue::Integer(10)},
            IfElseExpressionTest{input: "if (1 > 2) { 10 }", expected: TestValue::Null},
            IfElseExpressionTest{input: "if (1 > 2) { 10 } else { 20 }", expected: TestValue::Integer(20)},
            IfElseExpressionTest{input: "if (1 < 2) { 10 } else { 20 }", expected: TestValue::Integer(10)},
        ];

        for test in if_else_expression_tests.iter() {
            let evaluated = test_eval(test.input.to_string());
            match test.expected {
                TestValue::Integer(val) => {
                    assert!(
                        test_integer_object(evaluated, val)
                    );
                },
                TestValue::Null => {
                    assert!(
                        test_null_object(evaluated)
                    );
                },
                _ => (),
            }
        }
    }

    fn test_null_object(object: object::Object) -> bool {
        match object {
            object::Object::Null => true,
            _ => {
                eprintln!("object is not NULL. got = {:?}", object);
                false
            },
        }
    }

    #[test]
    fn test_return_statement() {
        struct ReturnStatementTest {
            input: &'static str,
            expected: i64
        }

        let return_statement_tests = [
            ReturnStatementTest{input: "return 10;", expected: 10},
            ReturnStatementTest{input: "return 10; 9;", expected: 10},
            ReturnStatementTest{input: "return 2 * 5; 9;", expected: 10},
            ReturnStatementTest{input: "9; return 2 * 5; 9;", expected: 10},
            ReturnStatementTest{input: "if (10 > 1) { return 10; }", expected: 10},
            ReturnStatementTest{
            input: r#"
                if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                return 1;
                }
            
            "#,
            expected: 10},
            ReturnStatementTest{
            input: r#"
                let f = fn(x) {
                    return x;
                    x + 10;
                };
                f(10);
            "#,
            expected: 10},
            ReturnStatementTest{
            input: r#"
                let f = fn(x) {
                    let result = x + 10;
                    return result
                    return 10;
                };
                f(10);
            "#,
            expected: 20},
        ];

        for test in return_statement_tests.iter() {
            let evaluated = test_eval(test.input.to_string());
            assert!(
                test_integer_object(evaluated, test.expected)
            );
        }
    }
     
    #[test]
    fn test_error_handling() {
        struct ErrorTest{
            input: &'static str,
            expected_message: &'static str,
        }

        let error_handling_tests = [
            ErrorTest{input: "5 + true;", expected_message: "type mismatch: INTEGER + BOOLEAN"},
            ErrorTest{input: "5 + true; 5;", expected_message: "type mismatch: INTEGER + BOOLEAN"},
            ErrorTest{input: "-true;", expected_message: "unknown operator: -BOOLEAN"},
            ErrorTest{input: "true + false", expected_message: "unknown operator: BOOLEAN + BOOLEAN"},
            ErrorTest{input: "5; true + false, 5", expected_message: "unknown operator: BOOLEAN + BOOLEAN"},
            ErrorTest{input: "if (10 > 1) { true + false; }", expected_message: "unknown operator: BOOLEAN + BOOLEAN"},
            ErrorTest{input: "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }", expected_message: "unknown operator: BOOLEAN + BOOLEAN"},
            ErrorTest{input: "foobar", expected_message: "identifier not found: foobar"},
            ErrorTest{input: r#""Hello" - "World""#, expected_message: "unknown operator: STRING - STRING"},
            ErrorTest{input: r#"{"name": "Monkey"}[fn(x) { x }];"#, expected_message: "unusable as hash key: FUNCTION"},
        ];

        for test in error_handling_tests.iter() {
            let evaluated = test_eval(test.input.to_string());
            match evaluated {
                object::Object::Error(val) => {
                    assert_eq!(val, test.expected_message);
                },
                _ => eprintln!("no error object returned. got = {:?}", evaluated),
            }
        }
    }

    #[test]
    fn test_let_statements() {
        struct LetStatementsTest {
            input: &'static str,
            expected: i64,
        }

        let let_statements_tests = [
            LetStatementsTest{input: "let a = 5; a;", expected: 5},
            LetStatementsTest{input: "let a = 5 * 5; a;", expected: 25},
            LetStatementsTest{input: "let a = 5; let b = a; b;", expected: 5},
            LetStatementsTest{input: "let a = 5; let b = a; let c = a + b + 5; c;", expected: 15},
        ];

        for test in let_statements_tests.iter() {
            assert!(
                test_integer_object(
                    test_eval(test.input.to_string()),
                    test.expected
                )
            );
        }
    }

    #[test]
    fn test_function_object() {
        let input = String::from("fn(x) { x + 2 };");

        let evaluated = test_eval(input);

        match evaluated {
            Object::Function(parameters, body, _) => {
                assert_eq!(
                    parameters.len(),
                    1,
                    "function has wrong parameters. Parameters = {:?}",
                    parameters
                );

                assert_eq!(
                    parameters[0].string(),
                    "x",
                    "parameter is not 'x'. got = {}",
                    parameters[0].string()
                );

                let expected_body = String::from("(x + 2)");
                assert_eq!(
                    body.string(),
                    expected_body,
                    "body is not = {}. got = {}",
                    expected_body,
                    body.string()
                );
            },
            Object::Null | _ => panic!("object is not a Function. got Object::Null"),
        }
    }

    #[test]
    fn test_function_application() {
        struct FunctionApplicationTest {
            input: &'static str,
            expected: i64,
        }

        let function_application_tests= [
            FunctionApplicationTest{input: "let identity = fn(x) { x; }; identity(5);", expected: 5},
            FunctionApplicationTest{input: "let identity = fn(x) { return x; }; identity(5);", expected: 5},
            FunctionApplicationTest{input: "let double = fn(x) { x * 2; }; double(5);", expected: 10},
            FunctionApplicationTest{input: "let add = fn(x, y) { x + y; }; add(5, 5);", expected: 10},
            FunctionApplicationTest{input: "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", expected: 20},
            FunctionApplicationTest{input: "fn(x) { x; };(5);", expected: 5},
        ];

        for test in function_application_tests.iter() {
            assert!(
                test_integer_object(
                    test_eval(test.input.to_string()),
                    test.expected
                )
            );
        }
    }

    #[test]
    fn test_closure() {
        let input = String::from(r#"
            let newAdder = fn(x) {
                fn(y) { x + y };
            };

            let addTwo = newAdder(2);
            addTwo(2);
        "#);

        assert!(
            test_integer_object(test_eval(input), 4)
        )
    }

    #[test]
    fn test_string_literal() {
        let input = String::from(r#""Hello World!""#);
        let evaluated = test_eval(input);

        match evaluated {
            Object::String(str) => {
                assert_eq!(
                    str,
                    "Hello World!",
                    "String has wrong value. got = {}",
                    str
                )
            },
            _ => panic!("object is not String. got = {:?}", evaluated)
        }
    }

    #[test]
    fn test_string_concatenation() {
        let input = String::from(r#""Hello" + " " + "World!""#);
        let evaluated = test_eval(input);

        match evaluated {
            Object::String(str) => {
                assert_eq!(
                    str,
                    "Hello World!",
                    "String has wrong value. got = {}",
                    str
                )
            },
            _ => panic!("object is not String. got = {:?}", evaluated)
        }

    }

    #[test]
    fn test_builtin_functions() {
        struct BuiltinFunctionsTest{
            input: &'static str,
            expected: TestValue,
        }

        let builtin_functions_test= [
            BuiltinFunctionsTest{input: r#"len("")"#, expected: TestValue::Integer(0)},
            BuiltinFunctionsTest{input: r#"len("four")"#, expected: TestValue::Integer(4)},
            BuiltinFunctionsTest{input: r#"len("hello world")"#, expected: TestValue::Integer(11)},
            BuiltinFunctionsTest{input: r#"len(1)"#, expected: TestValue::Error(String::from("argument to `len` not supported, got = INTEGER"))},
            BuiltinFunctionsTest{input: r#"len("one", "two")"#, expected: TestValue::Error(String::from("wrong number of arguments. got = 2, want = 1"))},
            BuiltinFunctionsTest{input: "first([1, 2, 3])", expected: TestValue::Integer(1)},
            BuiltinFunctionsTest{input: "first([])", expected: TestValue::Null},
            BuiltinFunctionsTest{input: "first(1)", expected: TestValue::Error(String::from("argument to `first` must be ARRAY, got = INTEGER"))},
            BuiltinFunctionsTest{input: "last([1, 2, 3])", expected: TestValue::Integer(3)},
            BuiltinFunctionsTest{input: "last([])", expected: TestValue::Null},
            BuiltinFunctionsTest{input: "last(1)", expected: TestValue::Error(String::from("argument to `last` must be ARRAY, got = INTEGER"))},
            BuiltinFunctionsTest{input: "rest([1, 2, 3])", expected: TestValue::Array(vec![2, 3])},
            BuiltinFunctionsTest{input: "rest([])", expected: TestValue::Null},
            BuiltinFunctionsTest{input: "push([], 1)", expected: TestValue::Array(vec![1])},
            BuiltinFunctionsTest{input: "push(1, 1)", expected: TestValue::Error(String::from("argument to `push` must be ARRAY, got = INTEGER"))},
        ];

        for test in builtin_functions_test.iter() {
            let evaluated = test_eval(test.input.to_string());

            match &test.expected {
                TestValue::Integer(int) => {
                    assert!(test_integer_object(evaluated, *int))
                },
                TestValue::Error(expected_msg) => {
                    match evaluated {
                        Object::Error(actual_msg) => 
                            assert_eq!(
                                &actual_msg,
                                expected_msg,
                            ),
                        _ => panic!("Expected error, got = {:?}", evaluated)
                    }
                },
                TestValue::Array(expected_elements) => {
                    match evaluated {
                        Object::Array(actual_elements) => {
                            assert_eq!(
                                actual_elements.len(),
                                expected_elements.len(),
                                "wrong number of elements. want = {}, got = {}",
                                expected_elements.len(),
                                actual_elements.len()
                            );

                            for (idx, expected_element) in expected_elements.iter().enumerate() {
                                assert!(test_integer_object(actual_elements[idx].clone(), *expected_element))
                            }

                        },
                        _ => eprintln!("Expected Array. got = {:?}", evaluated)
                    }
                }
                TestValue::Null => assert!(test_null_object(evaluated)),
            }
        }
    }

    #[test]
    fn test_array_literals() {
        let input = String::from("[1, 2 * 2, 3 + 3]");
        let evaluated = test_eval(input);

        match evaluated {
            Object::Array(vec_obj) => {
                assert_eq!(
                    vec_obj.len(),
                    3,
                    "array has wrong num of element. got = {}",
                    vec_obj.len()
                );

                assert!(test_integer_object(vec_obj[0].clone(), 1));
                assert!(test_integer_object(vec_obj[1].clone(), 4));
                assert!(test_integer_object(vec_obj[2].clone(), 6));
            },
            _ => panic!("object is not Array. got = {:?}", evaluated)
        }
    }

    #[test]
    fn test_array_index_expression() {
        struct ArrayIndexExpressionTest{
            input: &'static str,
            expected: TestValue,
        }

        let array_index_expression_tests= [
            ArrayIndexExpressionTest{input: "[1, 2, 3][0]", expected: TestValue::Integer(1)},
            ArrayIndexExpressionTest{input: "[1, 2, 3][1]", expected: TestValue::Integer(2)},
            ArrayIndexExpressionTest{input: "[1, 2, 3][2]", expected: TestValue::Integer(3)},
            ArrayIndexExpressionTest{input: "let i = 0; [1][i]", expected: TestValue::Integer(1)},
            ArrayIndexExpressionTest{input: "[1, 2, 3][1 + 1]", expected: TestValue::Integer(3)},
            ArrayIndexExpressionTest{input: "let myArray = [1, 2, 3]; myArray[2]", expected: TestValue::Integer(3)},
            ArrayIndexExpressionTest{input: "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2]", expected: TestValue::Integer(6)},
            ArrayIndexExpressionTest{input: "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i];", expected: TestValue::Integer(2)},
            ArrayIndexExpressionTest{input: "[1, 2, 3][3];", expected: TestValue::Null},
            ArrayIndexExpressionTest{input: "[1, 2, 3][-1];", expected: TestValue::Null},
        ];

        for test in array_index_expression_tests.iter() {
            let evaluated = test_eval(test.input.to_string());
            match test.expected {
                TestValue::Integer(int) => {
                    assert!(test_integer_object(evaluated, int))
                },
                _ => assert!(test_null_object(evaluated))
            };
        }
    }
    
    #[test]
    fn test_hash_literals() {
        let input = String::from(r#"
            let two = "two";
            {
                "one": 10 - 9,
                two: 1 + 1,
                "thr" + "ee": 6 / 2,
                4: 4,
                true: 5,
                false: 6
            }
        "#);
        let evaluated = test_eval(input);

        match evaluated {
            Object::Hash(hash_pairs) => {
                assert_eq!(
                    hash_pairs.len(),
                    6,
                    "Hash has wrong number of pairs. got = {}",
                    hash_pairs.len()
                );
                
                for (hash_key, hash_val) in &hash_pairs {
                    match hash_key {
                        HashKey::String(str) => {
                            match str.as_str() {
                                "one" => {
                                    assert!(test_integer_object(hash_val.clone(), 1));
                                },
                                "two" => {
                                    assert!(test_integer_object(hash_val.clone(), 2));
                                },
                                "three" => {
                                    assert!(test_integer_object(hash_val.clone(), 3));
                                },
                                _ => panic!("Unexptected string key: {}", str),
                            }
                        },
                        HashKey::Integer(int) => {
                            match int {
                                4 => {
                                    assert!(test_integer_object(hash_val.clone(), 4));
                                },
                                _ => panic!("Unexptected integer key: {}", int),
                                }
                        }
                        HashKey::Boolean(bool_key) => {
                            match bool_key {
                                true => {
                                    assert!(test_integer_object(hash_val.clone(), 5));
                                },
                                false => {
                                    assert!(test_integer_object(hash_val.clone(), 6));
                                },
                            }
                        }
                    }
                }
            },
            _ => panic!("eval didn't return Hash. got = {:?}", evaluated)
        }
    }

    #[test]
    fn test_hash_index_expression() {
        struct HashIndexExpressionTest{
            input: &'static str,
            expected: TestValue,
        }

        let hash_index_expression_tests= [
            HashIndexExpressionTest{input: r#"{"foo": 5}["foo"]"#, expected: TestValue::Integer(5)},
            HashIndexExpressionTest{input: r#"{"foo": 5}["bar"]"#, expected: TestValue::Null},
            HashIndexExpressionTest{input: r#"let key = "foo"; {"foo": 5}[key]"#, expected: TestValue::Integer(5)},
            HashIndexExpressionTest{input: r#"{}["foo"]"#, expected: TestValue::Null},
            HashIndexExpressionTest{input: r#"{true: 5}[true]"#, expected: TestValue::Integer(5)},
            HashIndexExpressionTest{input: r#"{false: 5}[false]"#, expected: TestValue::Integer(5)}
        ];

        for test in hash_index_expression_tests.iter() {
            let evaluated = test_eval(test.input.to_string());
            match test.expected {
                TestValue::Integer(int) => {
                    assert!(test_integer_object(evaluated, int))
                },
                _ => assert!(test_null_object(evaluated))
            };
        }
    }
}