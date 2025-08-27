#[cfg(test)]
mod tests {
    use crate::token::token::*;

    #[test]
    fn test_next_token(){
        let input = String::from(
            r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };
            
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }   

            10 == 10;
            10 != 9;
            "foobar"
            "foo bar"
            [1, 2];
            "#
        );
        let mut lexer = crate::lexer::lexer::Lexer::new(input);
        let test_token_vec = create_test_token_vec();
        for token in test_token_vec {
            let nxt_token = lexer.next_token();
            assert_eq!(
                token.token_type,
                nxt_token.token_type,
                "{:?} test - token_type wrong, expected = {}, got = {}",
                token, nxt_token.token_type, token.token_type
            );

            assert_eq!(
                token.literal,
                nxt_token.literal,
                "{:?} test - literal wrong, expected = {}, got = {}",
                token, nxt_token.literal, token.literal
            );
        }
    }

    fn create_test_token_vec() -> Vec<Token> {
        let mut test_token_vec: Vec<Token> = vec![];
        test_token_vec.push(Token{ token_type: LET, literal: String::from("let")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("five")});
        test_token_vec.push(Token{ token_type: ASSIGN, literal: String::from("=")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("5")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});

        test_token_vec.push(Token{ token_type: LET, literal: String::from("let")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("ten")});
        test_token_vec.push(Token{ token_type: ASSIGN, literal: String::from("=")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("10")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});

        test_token_vec.push(Token{ token_type: LET, literal: String::from("let")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("add")});
        test_token_vec.push(Token{ token_type: ASSIGN, literal: String::from("=")});
        test_token_vec.push(Token{ token_type: FUNCTION, literal: String::from("fn")});
        test_token_vec.push(Token{ token_type: LPAREN, literal: String::from("(")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("x")});
        test_token_vec.push(Token{ token_type: COMMA, literal: String::from(",")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("y")});
        test_token_vec.push(Token{ token_type: RPAREN, literal: String::from(")")});
        test_token_vec.push(Token{ token_type: LBRACE, literal: String::from("{")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("x")});
        test_token_vec.push(Token{ token_type: PLUS, literal: String::from("+")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("y")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});
        test_token_vec.push(Token{ token_type: RBRACE, literal: String::from("}")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});

        test_token_vec.push(Token{ token_type: LET, literal: String::from("let")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("result")});
        test_token_vec.push(Token{ token_type: ASSIGN, literal: String::from("=")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("add")});
        test_token_vec.push(Token{ token_type: LPAREN, literal: String::from("(")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("five")});
        test_token_vec.push(Token{ token_type: COMMA, literal: String::from(",")});
        test_token_vec.push(Token{ token_type: IDENT, literal: String::from("ten")});
        test_token_vec.push(Token{ token_type: RPAREN, literal: String::from(")")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});

        test_token_vec.push(Token{ token_type: BANG, literal: String::from("!")});
        test_token_vec.push(Token{ token_type: MINUS, literal: String::from("-")});
        test_token_vec.push(Token{ token_type: SLASH, literal: String::from("/")});
        test_token_vec.push(Token{ token_type: ASTERISK, literal: String::from("*")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("5")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});

        test_token_vec.push(Token{ token_type: INT, literal: String::from("5")});
        test_token_vec.push(Token{ token_type: LT, literal: String::from("<")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("10")});
        test_token_vec.push(Token{ token_type: GT, literal: String::from(">")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("5")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});

        test_token_vec.push(Token{ token_type: IF, literal: String::from("if")});
        test_token_vec.push(Token{ token_type: LPAREN, literal: String::from("(")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("5")});
        test_token_vec.push(Token{ token_type: LT, literal: String::from("<")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("10")});
        test_token_vec.push(Token{ token_type: RPAREN, literal: String::from(")")});
        test_token_vec.push(Token{ token_type: LBRACE, literal: String::from("{")});
        test_token_vec.push(Token{ token_type: RETURN, literal: String::from("return")});
        test_token_vec.push(Token{ token_type: TRUE, literal: String::from("true")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});
        test_token_vec.push(Token{ token_type: RBRACE, literal: String::from("}")});
        test_token_vec.push(Token{ token_type: ELSE, literal: String::from("else")});
        test_token_vec.push(Token{ token_type: LBRACE, literal: String::from("{")});
        test_token_vec.push(Token{ token_type: RETURN, literal: String::from("return")});
        test_token_vec.push(Token{ token_type: FALSE, literal: String::from("false")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});
        test_token_vec.push(Token{ token_type: RBRACE, literal: String::from("}")});

        test_token_vec.push(Token{ token_type: INT, literal: String::from("10")});
        test_token_vec.push(Token{ token_type: EQ, literal: String::from("==")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("10")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("10")});
        test_token_vec.push(Token{ token_type: NQ, literal: String::from("!=")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("9")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});

        test_token_vec.push(Token{ token_type: STRING, literal: String::from("foobar")});
        test_token_vec.push(Token{ token_type: STRING, literal: String::from("foo bar")});

        test_token_vec.push(Token{ token_type: LBRACKET, literal: String::from("[")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("1")});
        test_token_vec.push(Token{ token_type: COMMA, literal: String::from(",")});
        test_token_vec.push(Token{ token_type: INT, literal: String::from("2")});
        test_token_vec.push(Token{ token_type: RBRACKET, literal: String::from("]")});
        test_token_vec.push(Token{ token_type: SEMICOLON, literal: String::from(";")});

        test_token_vec.push(Token{ token_type: EOF, literal: String::from("")});

        test_token_vec
    }
}