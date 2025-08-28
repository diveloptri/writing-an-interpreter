#[cfg(test)]
mod tests {
    use crate::ast::ast::{ExpressionType, Node, StatementType};
    use crate::lexer::lexer;
    use crate::parser::parser::{self, Parser};

    #[derive(Debug, Clone, PartialEq)]
    enum TestValue {
        Integer(i64),
        Boolean(bool),
        String(String),
    }

    #[test]
    fn test_let_statements(){
        struct LetStatementTest {
            input: &'static str,
            expected_identifier: &'static str,
            expected_value: TestValue,
        }

        let let_statement_test = [
            LetStatementTest{input: "let x = 5;", expected_identifier: "x", expected_value: TestValue::Integer(5)},
            LetStatementTest{input: "let y = 10;", expected_identifier: "y", expected_value: TestValue::Integer(10)},
            LetStatementTest{input: "let foobar = y;", expected_identifier: "foobar", expected_value: TestValue::String("y".to_string())},
        ];

        for test in let_statement_test.iter(){
            let lexer = lexer::Lexer::new(test.input.to_string());
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser);

            assert_eq!(
                program.statements.len(),
                1,
                "program does not contain {} statements. got = {}",
                1,
                program.statements.len()
            );

            if !test_let_statement(&program.statements[0],test.expected_identifier) {
                panic!();
            }

            let let_stmt = match &program.statements[0] {
                StatementType::Let(stmt) => stmt,
                _ => panic!("expr_stmt is not ast::LetStatement"),
            };

            if !test_literal_expression(let_stmt.value.as_ref().unwrap(), &test.expected_value){
                panic!();
            }
        }
    }

    #[test]
    fn test_return_statements() {
        struct ReturnStatementTest{
            input: &'static str,
            expected_value: TestValue 
        }

        let return_tests = [
            ReturnStatementTest{input: "return 5;", expected_value: TestValue::Integer(5)},
            ReturnStatementTest{input: "return 10;", expected_value: TestValue::Integer(10)},
            ReturnStatementTest{input: "return foobar;", expected_value: TestValue::String("foobar".to_string())},
        ];

        for test in return_tests.iter() {
            let lexer = lexer::Lexer::new(test.input.to_string());
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser);

            assert_eq!(
                program.statements.len(),
                1,
                "program does not contain {} statements. got = {}",
                1,
                program.statements.len()
            );

            let return_stmt = match &program.statements[0] {
                StatementType::Return(stmt) => stmt,
                _ => panic!("program.statements[0] is not ast::ReturnStatement"),
            };

            assert_eq!(
                return_stmt.token_literal(),
                "return",
                "return_stmt.token_literal is not 'return'. got = {}",
                return_stmt.token_literal()
            );

            let Some(return_value) = &return_stmt.return_value else {
                panic!("ReturnStatement has no return_value");
            };

            if !test_literal_expression(return_value, &test.expected_value) {
                panic!();
            }
        } 

    }

    #[test]
    fn test_identifier_expression(){
        let input = String::from("foobar;");

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got = {}",
            program.statements.len()
        );

        let expr_stmt = match &program.statements[0] {
            StatementType::Expression(stmt) => stmt,
            _ => panic!("program.statements[0] is not ast::ExpressionStatement"),
        };

        let Some(expression) = &expr_stmt.expression else {
            panic!("ExpressionStatement has no expression");
        };

        let identifier = match expression {
            ExpressionType::Identifier(ident) => ident,
            _ => panic!("expression is not Identifier"),
        };

        assert_eq!(
            "foobar",
            identifier.value,
            "identifier.value is not {}. got = {}",
            "foobar",
            identifier.value
        );

        assert_eq!(
            "foobar",
            identifier.token_literal(),
            "identifier.token_literal is not {}. got = {}",
            "foobar",
            identifier.token_literal()
        );

    }

    #[test]
    fn test_integer_literal_expression() {
        let input = String::from("5;");

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got = {}",
            program.statements.len()
        );

        let expr_stmt = match &program.statements[0] {
            StatementType::Expression(stmt) => stmt,
            _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
        };

        let Some(expression) = &expr_stmt.expression else {
            panic!("ExpressionStatement has no expression");
        };

        let int_literal= match expression {
            ExpressionType::IntegerLiteral(int_lit) => int_lit,
            _ => panic!("expr_stmt is not ast::IntegerLiteral. got = {:?}", expression), 
        };
        
        assert_eq!(
            int_literal.value,
            5,
            "int_literal.value is not {}. got = {}",
            5,
            int_literal.value
        );

        assert_eq!(
            int_literal.token_literal(),
            "5",
            "int_literal.value is not {}. got = {}",
            "5",
            int_literal.token_literal()
        );
    }

    #[test]
    fn test_boolean_expression() {
        struct BooleanTest {
            input: &'static str,
            expected_boolean: bool
        }

        let boolean_tests = [
            BooleanTest{input: "true;", expected_boolean: true},
            BooleanTest{input: "false;", expected_boolean: false},
        ];

        for test in boolean_tests.iter() {
            let lexer = lexer::Lexer::new(test.input.to_string());
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser);

            assert_eq!(
                program.statements.len(),
                1,
                "program does not contain {} statements. got = {}",
                1,
                program.statements.len()
            );

            let expr_stmt = match &program.statements[0] {
                StatementType::Expression(stmt) => stmt,
                _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
            };

            let Some(expression) = &expr_stmt.expression else {
                panic!("ExpressionStatement has no expression");
            };

            let boolean= match expression {
                ExpressionType::Boolean(boolean) => boolean,
                _ => panic!("expr_stmt is not ast::Boolean. got = {:?}", expression), 
            };

            assert_eq!(
                boolean.value,
                test.expected_boolean,
                "boolean.value is not {}. got = {}",
                test.expected_boolean,
                boolean.value
            );
        }
    }

    #[test]
    fn test_parsing_prefix_expression() {
        struct PrefixTest {
            input: &'static str,
            operator: &'static str,
            value: TestValue,
        }

        let prefix_tests = [
            PrefixTest{input: "!5", operator: "!", value: TestValue::Integer(5)},
            PrefixTest{input: "-15", operator: "-", value: TestValue::Integer(15)},
            PrefixTest{input: "!true;", operator: "!", value: TestValue::Boolean(true)},
            PrefixTest{input: "!false;", operator: "!", value: TestValue::Boolean(false)},
        ];

        for test in prefix_tests.iter(){
            let lexer = lexer::Lexer::new(test.input.to_string());
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser);

            assert_eq!(
                program.statements.len(),
                1,
                "program does not contain {} statements. got = {}",
                1,
                program.statements.len()
            );

            let expr_stmt = match &program.statements[0] {
                StatementType::Expression(stmt) => stmt,
                _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
            };


            let Some(expression) = &expr_stmt.expression else {
                panic!("ExpressionStatement has no expression");
            };

            let prefix_expression = match expression {
                ExpressionType::PrefixExpression(prefix_expr) => prefix_expr,
                _ => panic!("expression is not ast::PrefixExpression. got = {:?}", expression),
            };
            
            assert_eq!(
                prefix_expression.operator,
                test.operator,
                "prefix_expression.operator is not {}. got = {}",
                test.operator,
                prefix_expression.operator
            );
            
            if !test_literal_expression(&prefix_expression.right, &test.value) {
                panic!();
            }
        }

    }

    #[test]
    fn test_parsing_infix_expressions() {
        struct InfixTest {
            input: &'static str,
            left_value: TestValue,
            operator: &'static str,
            right_value: TestValue,
        }

        let infix_tests = [
            InfixTest{input: "5 + 5", left_value: TestValue::Integer(5), operator: "+", right_value: TestValue::Integer(5)},
            InfixTest{input: "5 - 5", left_value: TestValue::Integer(5), operator: "-", right_value: TestValue::Integer(5)},
            InfixTest{input: "5 * 5", left_value: TestValue::Integer(5), operator: "*", right_value: TestValue::Integer(5)},
            InfixTest{input: "5 / 5", left_value: TestValue::Integer(5), operator: "/", right_value: TestValue::Integer(5)},
            InfixTest{input: "5 > 5", left_value: TestValue::Integer(5), operator: ">", right_value: TestValue::Integer(5)},
            InfixTest{input: "5 < 5", left_value: TestValue::Integer(5), operator: "<", right_value: TestValue::Integer(5)},
            InfixTest{input: "5 == 5", left_value: TestValue::Integer(5), operator: "==", right_value: TestValue::Integer(5)},
            InfixTest{input: "5 != 5", left_value: TestValue::Integer(5), operator: "!=", right_value: TestValue::Integer(5)},
            InfixTest{input: "true == true", left_value: TestValue::Boolean(true), operator: "==", right_value: TestValue::Boolean(true)},
            InfixTest{input: "true != false", left_value: TestValue::Boolean(true), operator:  "!=", right_value: TestValue::Boolean(false)},
            InfixTest{input: "false == false", left_value: TestValue::Boolean(false), operator: "==", right_value: TestValue::Boolean(false)},
        ];

        for test in infix_tests.iter(){
            let lexer = lexer::Lexer::new(test.input.to_string());
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser);

            assert_eq!(
                program.statements.len(),
                1,
                "program does not contain {} statement. got = {}",
                1,
                program.statements.len()
            );

            let expr_stmt = match &program.statements[0] {
                StatementType::Expression(stmt) => stmt,
                _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
            };

            if !test_infix_expression(&*expr_stmt.expression.as_ref().unwrap(), test.left_value.clone(), test.operator, test.right_value.clone()) {
                panic!();
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        struct PrecedenceTest {
            input: &'static str,
            expected: &'static str,
        }

        let precedence_tests = [
            PrecedenceTest{input: "-a * b", expected: "((-a) * b)"},
            PrecedenceTest{input: "!-a", expected: "(!(-a))"},
            PrecedenceTest{input: "a + b + c", expected: "((a + b) + c)"},
            PrecedenceTest{input: "a + b - c", expected: "((a + b) - c)"},
            PrecedenceTest{input: "a * b * c", expected: "((a * b) * c)"},
            PrecedenceTest{input: "a * b / c", expected: "((a * b) / c)"},
            PrecedenceTest{input: "a + b / c", expected: "(a + (b / c))"},
            PrecedenceTest{input: "a + b * c + d / e - f", expected: "(((a + (b * c)) + (d / e)) - f)"},
            PrecedenceTest{input: "3 + 4; -5 * 5", expected: "(3 + 4)((-5) * 5)"},
            PrecedenceTest{input: "5 > 4 == 3 < 4", expected: "((5 > 4) == (3 < 4))"},
            PrecedenceTest{input: "5 < 4 != 3 > 4", expected: "((5 < 4) != (3 > 4))"},
            PrecedenceTest{input: "3 + 4 * 5 == 3 * 1 + 4 * 5", expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"},
            PrecedenceTest{input: "1 + (2 + 3) + 4", expected: "((1 + (2 + 3)) + 4)"},
            PrecedenceTest{input: "(5 + 5) * 2", expected: "((5 + 5) * 2)"},
            PrecedenceTest{input: "2 / (5 + 5)", expected: "(2 / (5 + 5))"},
            PrecedenceTest{input: "-(5 + 5)", expected: "(-(5 + 5))"},
            PrecedenceTest{input: "true", expected: "true"},
            PrecedenceTest{input: "false", expected: "false"},
            PrecedenceTest{input: "3 > 5 == false", expected: "((3 > 5) == false)"},
            PrecedenceTest{input: "!(true == true)", expected: "(!(true == true))"},
            PrecedenceTest{input: "a + add(b * c) + d", expected: "((a + add((b * c))) + d)"},
            PrecedenceTest{input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", expected: "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"},
            PrecedenceTest{input: "add(a + b + c * d / f + g)", expected: "add((((a + b) + ((c * d) / f)) + g))"},
        ];

        for test in precedence_tests.iter() {
            let lexer = lexer::Lexer::new(test.input.to_string());
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser);

            let parsed_string = program.string();
            
            assert_eq!(
                parsed_string,
                test.expected,
                "expected = {}, got = {}",
                test.expected,
                parsed_string
            )
        }
    }

    #[test]
    fn test_if_expression() {
        let input = String::from("if (x < y) { x }");

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program does not contain {} statements. got = {}",
            1,
            program.statements.len()
        );

        let stmt = match &program.statements[0] {
            StatementType::Expression(stmt) => stmt,
            _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
        };

        let if_expr = match &stmt.expression {
            Some(ExpressionType::IfExpression(if_expr)) => if_expr,
            None | Some(_) => panic!("stmt.expression is not ast::IfExpression. got = {:?}", stmt.expression),
        };

        if !test_infix_expression(if_expr.condition.as_ref(), TestValue::String("x".to_string()), "<", TestValue::String("y".to_string())) {
            panic!();
        }

        if if_expr.consequence.statements.len() != 1 {
            eprintln!("consequence is not 1 statement. gor = {:?}", if_expr.consequence.statements.len());
        }

        let expr_stmt = match &if_expr.consequence.statements[0] {
            StatementType::Expression(stmt) => stmt,
            _ => panic!("statements[0] is not ast::ExpressionStatement. got = {:?}", if_expr.consequence.statements[0]),
        };

        assert_eq!(
            test_identifier(&*expr_stmt.expression.as_ref().unwrap(), "x"),
            true
        );

        if if_expr.alternative.is_some() {
            panic!("if_expr.alternative.statements was not None. got = {:?}", if_expr.alternative);
        }

    }

    #[test]
    fn test_function_literal_parsing() {
        let input = String::from("fn(x, y) { x + y; }");

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program does not contain {} statements. got = {}",
            1,
            program.statements.len()
        );

        let stmt = match &program.statements[0] {
            StatementType::Expression(stmt) => stmt,
            _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
        };

        let function_literal = match &stmt.expression {
            Some(ExpressionType::FunctionLiteral(function_literal)) => function_literal,
            _ => panic!("stmt.expression is not ast::FunctionLiteral. got = {:?}", stmt.expression),
        };

        assert_eq!(
            function_literal.parameters.len(),
            2,
            "function literal parameters are wrong. want 2, got = {}",
            function_literal.parameters.len()
        );

        let expected_params = ["x", "y"];
        for (param, expected) in function_literal.parameters.iter().zip(&expected_params) {
            assert!(
                test_literal_expression(&ExpressionType::Identifier(param.clone()), &TestValue::String(expected.to_string()))
            );
        }

        assert_eq!(
            function_literal.body.statements.len(),
            1,
            "function_literal.body.statements has not 1 statements. got = {}",
            function_literal.body.statements.len()
        );

        let body_stmt = match &function_literal.body.statements[0] {
            StatementType::Expression(expr) => expr,
            _ => panic!("function_literal.body.statements is not ast::ExpressionStatement. got = {:?}", function_literal.body.statements[0]),
        };

        assert_eq!(
            test_infix_expression(&*body_stmt.expression.as_ref().unwrap(), TestValue::String("x".to_string()), "+", TestValue::String("y".to_string())),
            true
        );
    }

    #[test]
    fn test_function_parameter_parsing() {
        struct ParameterTest {
            input: &'static str,
            expected_parameter: Vec<String>
        }

        let parameter_test = [
            ParameterTest{input: "fn() {};", expected_parameter: Vec::new()},
            ParameterTest{input: "fn(x) {};", expected_parameter: vec!["x".to_string()]},
            ParameterTest{input: "fn(x, y, z) {};", expected_parameter: vec!["x".to_string(), "y".to_string(), "z".to_string()]},
        ];

        for test in parameter_test.iter(){
            let lexer = lexer::Lexer::new(test.input.to_string());
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser);

            let stmt = match &program.statements[0] {
                StatementType::Expression(stmt) => stmt,
                _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
            };

            let function_literal = match &stmt.expression {
                Some(ExpressionType::FunctionLiteral(function_literal)) => function_literal,
                _ => panic!("stmt.expression is not ast::FunctionLiteral. got = {:?}", stmt.expression),
            };

            assert_eq!(
                function_literal.parameters.len(),
                test.expected_parameter.len(),
                "length parameters wrong. want = {}, got = {}",
                test.expected_parameter.len(),
                function_literal.parameters.len()
            );

            for (param, expected) in function_literal.parameters
                .iter()
                .zip(test.expected_parameter.clone()){
                    assert!(
                        test_literal_expression(&ExpressionType::Identifier(param.clone()), &TestValue::String(expected.to_string())),
                        "Parameter mismatch: expected {}, got = {:?}", expected, param
                    );
            }
        } 
    }

    #[test]
    fn test_call_expression_parsing() {
        let input = String::from("add(1, 2 * 3, 4 + 5);");

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program does not contain {} statements. got = {}",
            1,
            program.statements.len()
        );

        let stmt = match &program.statements[0] {
            StatementType::Expression(stmt) => stmt,
            _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
        };

        let call_expr = match &stmt.expression {
            Some(ExpressionType::CallExpression(call_expr)) => call_expr,
            _ => panic!("stmt.expression is not ast::CallExpression. got = {:?}", stmt.expression),
        };

        assert_eq!(
            test_identifier(&*call_expr.function.as_ref(), "add"),
            true
        );

        assert_eq!(
            call_expr.arguments.len(),
            3,
            "wrong length of arguments. got = {}",
            call_expr.arguments.len()
        );

        assert!(
            test_literal_expression(&call_expr.arguments[0], &TestValue::Integer(1)),
            "Argument mismatch: expected {}, got = {:?}", 1, call_expr.arguments[0]
        );

        assert_eq!(
            test_infix_expression(&call_expr.arguments[1], TestValue::Integer(2), "*", TestValue::Integer(3)),
            true
        );

        assert_eq!(
            test_infix_expression(&call_expr.arguments[2], TestValue::Integer(4), "+", TestValue::Integer(5)),
            true
        );
    }


    #[test]
    fn test_call_expression_parameter_parsing() {
        struct CallExpressionParameterTest {
            input: &'static str,
            expected_ident: &'static str,
            expected_args: Vec<String>
        }

        let call_expression_parameter_test = [
            CallExpressionParameterTest{input: "add();", expected_ident: "add", expected_args: Vec::new()},
            CallExpressionParameterTest{input: "add(1);", expected_ident: "add", expected_args: vec!["1".to_string()]},
            CallExpressionParameterTest{input: "add(1, 2 * 3, 4 + 5);", expected_ident: "add", expected_args: vec!["1".to_string(), "(2 * 3)".to_string(), "(4 + 5)".to_string()]},
        ];

        for test in call_expression_parameter_test.iter(){
            let lexer = lexer::Lexer::new(test.input.to_string());
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser);

            let stmt = match &program.statements[0] {
                StatementType::Expression(stmt) => stmt,
                _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
            };

            let call_expr = match &stmt.expression {
                Some(ExpressionType::CallExpression(call_expr)) => call_expr,
                _ => panic!("stmt.expression is not ast::CallExpression. got = {:?}", stmt.expression),
            };

            assert_eq!(
                test_identifier(&*call_expr.function.as_ref(), test.expected_ident),
                true
            );

            assert_eq!(
                call_expr.arguments.len(),
                test.expected_args.len(),
                "wrong number of argument. want = {}, got = {}",
                test.expected_args.len(),
                call_expr.arguments.len()
            );

            for (expected, args) in test.expected_args
                .iter()
                .zip(&call_expr.arguments){
                    assert_eq!(
                        &args.string(),
                        expected,
                        "Wrong argument. want = {}, got = {}",
                        expected, args.string()
                    )
            }
        } 
    }

    #[test]
    fn test_string_literal_expression() {
        let input = String::from(r#""hello world";"#);

        let lexer = lexer::Lexer::new(input.to_string());
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        let stmt = match &program.statements[0] {
            StatementType::Expression(stmt) => stmt,
            _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
        };

        let string_literal = match &stmt.expression {
            Some(ExpressionType::StringLiteral(string_lit)) => string_lit,
            _ => panic!("stmt.expression is not ast::StringLiteral. got = {:?}", stmt.expression) 
        };

        assert_eq!(
            string_literal.value,
            "hello world",
            "string_literal.value is not = {}. got = {}",
            "hello world",
            string_literal.value
        )

    }

    #[test]
    fn test_parsing_array_literal() {
        let input = String::from("[1, 2 * 2, 3 + 3]");

        let lexer = lexer::Lexer::new(input.to_string());
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        let stmt = match &program.statements[0] {
            StatementType::Expression(stmt) => stmt,
            _ => panic!("program.statements[0] is not ast::ExpressionStatement"), 
        };

        let array_literal = match &stmt.expression {
            Some(ExpressionType::ArrayLiteral(array_lit)) => array_lit,
            _ => panic!("stmt.expression is not ast::ArrayLiteral. got = {:?}", stmt.expression) 
        };

        assert_eq!(
            array_literal.elements.len(),
            3,
            "array.elements not 3. got = {}",
            array_literal.elements.len()
        );

        assert!(
            test_integer_literal(&array_literal.elements[0], 1)
        );

        assert!(
            test_infix_expression(&array_literal.elements[1], TestValue::Integer(2), "*", TestValue::Integer(2))
        );

        assert!(
            test_infix_expression(&array_literal.elements[2], TestValue::Integer(3), "+", TestValue::Integer(3))
        );

    }

    // Helper
    fn test_identifier(expr: &ExpressionType, value: &str) -> bool {
        match expr {
            ExpressionType::Identifier(ident) => ident.value == value,
            _ => {
                eprintln!("expr not Identifier. got = {:?}", expr);
                false
            }
        }
    }

    fn test_let_statement(stmt: &StatementType, name: &str) -> bool {
        match stmt {
            StatementType::Let(let_stmt) => {
                if let_stmt.token_literal() != "let" {
                    dbg!("stmt.token_literal not 'let'. got = {}", let_stmt.token_literal());
                    return false
                }

                if let_stmt.name.value != name {
                    eprintln!("let_stmt.name.value is not ‘{}‘. got = {}", name, let_stmt.name.value);
                    return false
                }

                if let_stmt.name.token_literal() != name {
                    eprintln!("let_stmt.name.token_literal() is not ‘{}‘, got = {}", name, let_stmt.name.token_literal());
                    return false
                }
                    return true
            },
            _ => {
                eprintln!("stmt is not LetStatement");
                return false
            }
        }

    }

    fn test_infix_expression(expr: &ExpressionType, left: TestValue, operator: &str, right: TestValue) -> bool {
        let infix_expr = match expr {
            ExpressionType::InfixExpression(infix_expr) => infix_expr,
            _ => {
                eprintln!("expr is not ast::InfixExpression. got = {:?}", expr);
                return false
            }
        };

        if !test_literal_expression(&*infix_expr.left, &left) {
            return false
        }

        if infix_expr.operator != operator {
            eprintln!("infix_expr.operator is not {}. got = {}", operator, infix_expr.operator);
            return false
        }

        if !test_literal_expression(&*infix_expr.right, &right) {
            return false
        }

        return true
    }
    
    fn test_literal_expression(expr: &ExpressionType, expected: &TestValue) -> bool {
        match expected {
            TestValue::Integer(val) => return test_integer_literal(expr, *val),
            TestValue::Boolean(val) => return test_boolean_literal(expr, *val),
            TestValue::String(val) => return test_identifier(expr, val)
        }
    }

    fn test_integer_literal(expr: &ExpressionType, value: i64) -> bool {
        let int_literal = match expr {
            ExpressionType::IntegerLiteral(int_literal) => int_literal,
            _ => {
                eprintln!("expr is not ast::IntegerLiteral. got = {:?}", expr);
                return false
            }
        };

        if int_literal.value != value {
            eprintln!("int_literal is not {}. got = {}", value, int_literal.value);
            return false
        };

        if int_literal.token_literal() != value.to_string() {
            eprintln!("int_literal.token_literal() is not {}. got = {}", value, int_literal.token_literal());
            return  false
        }

        return true
    }

    fn test_boolean_literal(expr: &ExpressionType, value: bool) -> bool {
        let bool_val = match expr {
            ExpressionType::Boolean(bool_val) => bool_val,
            _ => {
                eprintln!("expr is not ast::Boolean. got = {:?}", expr);
                return false
            }
        };

        if bool_val.value != value {
            eprintln!("bool_val.value is not {}. got = {}", value, bool_val.value);
            return false
        };

        if bool_val.token_literal() != format!("{}", value) {
            eprintln!("bool_val.token_literal is not {}. got = {}", value, bool_val.token_literal());
            return false
        }

        return true
    }


    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.len() == 0 {
            return;
        }

        println!("parser has {} errors", errors.len());

        errors.iter().for_each(|msg| println!("parser error: {}", msg));
        panic!();
    }

}