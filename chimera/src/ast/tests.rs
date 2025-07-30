#[cfg(test)]
mod tests {
    use crate::ast::ast::{Identifier, LetStatement, Node, Program};
    use crate::token::token::{self, Token};


    #[test]
    fn test_string() {
        let program = Program{
            statements: vec![Box::new(
                LetStatement{
                    token: Token { token_type: token::LET, literal: String::from("let") },
                    name: Identifier{
                        token: Token { token_type: token::IDENT, literal: String::from("myVariable") },
                        value: String::from("myVariable"),
                    },
                    value: Some(Box::new(Identifier {
                        token: Token { token_type: token::IDENT, literal: String::from("anotherVariable") },
                        value: String::from("anotherVariable"),
                    })),
                },
            )],
        };

        assert_eq!(
            "let myVariable = anotherVariable;",
            program.string(),
            "program.string() wrong. got={}",
            program.string()
        );
    }
}