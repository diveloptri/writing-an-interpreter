#[cfg(test)]
mod tests {
    use crate::ast::ast::{Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Node, PrefixExpression, ReturnStatement, Statement};
    use crate::lexer::lexer;
    use crate::parser::parser::{self, Parser};

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.len() == 0 {
            return;
        }

        println!("parser has {} errors", errors.len());

        errors.iter().for_each(|msg| println!("parser error: {}", msg));
        panic!();
    }


    #[test]
    fn test_let_statements(){
        let input = String::from(
            r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
            "#
        );

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();
        
        check_parser_errors(&parser);

        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements. got = {}", program.statements.len());
        }

        let tests = vec![
            "x",
            "y",
            "foobar"
        ];

        for (i, expected_name) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            if !test_let_statement(&**stmt, expected_name) {
                panic!();
            } 

        }

    }

    fn test_let_statement(stmt: &dyn Statement, name: &str) -> bool {
        let Some(let_stmt) = stmt.as_any().downcast_ref::<LetStatement>() else {
            eprintln!("stmt is not LetStatement");
            return false
        };

        eprintln!("{}", let_stmt.token_literal());
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
    }
    



    #[test]
    fn test_return_statements() {
        let input = String::from(
            r#"
            return 5;
            return 10;
            return 999233;
            "#);

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements. got = {}", program.statements.len());
        }

        for stmt in program.statements {
            let Some(return_stmt) = stmt.as_any().downcast_ref::<ReturnStatement>() else {
                eprintln!("stmt is not ast::ReturnStatement");
                continue;
            };

            assert_eq!(
                return_stmt.token_literal(),
                "return",
                "return.stmt.token_literal is not 'return', got = {}",
                return_stmt.token_literal()
            );
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

        let Some(expr_stmt) = program.statements[0].as_any().downcast_ref::<ExpressionStatement>() else {
            panic!("program.statements[0] is not ast::ExpressionStatement");
        };

        let Some(expression) = &expr_stmt.expression else {
            panic!("ExpressionStatement has no expression");
        };

        let Some(identifier) = expression.as_any().downcast_ref::<Identifier>() else {
            panic!("expression is not Identifier");
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

        let Some(expr_stmt) = program.statements[0].as_any().downcast_ref::<ExpressionStatement>() else {
            panic!("program.statements[0] is not ast::ExpressionStatement");
        };

        let Some(expression) = &expr_stmt.expression else {
            panic!("ExpressionStatement has no expression");
        };

        let Some(int_literal) = expression.as_any().downcast_ref::<IntegerLiteral>() else {
            panic!("expr_stmt is not ast::IntegerLiteral. got = {:?}", expression);
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
    fn test_parsing_prefix_expression() {
        struct PrefixTest {
            input: &'static str,
            operator: &'static str,
            integer_value: i64,
        }

        let prefix_tests: Vec<PrefixTest> = vec![
            PrefixTest{input: "!5", operator: "!", integer_value: 5},
            PrefixTest{input: "-15", operator: "-", integer_value: 15},
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

            let Some(expr_stmt) = program.statements[0].as_any().downcast_ref::<ExpressionStatement>() else {
                panic!("program.statements[0] is not ast::ExpressionStatement");
            };

            let Some(expression) = &expr_stmt.expression else {
                panic!("ExpressionStatement has no expression");
            };

            let Some(prefix_expression) = expression.as_any().downcast_ref::<PrefixExpression>() else {
                panic!("expression is not ast::PrefixExpression. got = {:?}", expression);
            };

            assert_eq!(
                prefix_expression.operator,
                test.operator,
                "prefix_expression.operator is not {}. got = {}",
                test.operator,
                prefix_expression.operator
            );
            
            if !test_integer_literal(&*prefix_expression.right, test.integer_value) {
                panic!("prefix_expression.right is not {}. got = {:?}", test.integer_value, prefix_expression.right)
            }

        }

    }

    fn test_integer_literal(expr: &dyn Expression, value: i64) -> bool {
        let Some(int_literal) = expr.as_any().downcast_ref::<IntegerLiteral>() else {
            eprintln!("expr is not ast::IntegerLiteral. got = {:?}", expr);
            return false
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

}