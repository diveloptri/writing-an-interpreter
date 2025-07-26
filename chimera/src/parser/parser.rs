use std::collections::HashMap;

use crate::lexer::lexer::Lexer;
use crate::ast::ast::{self, Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, Program, ReturnStatement, Statement};
use crate::token::token::{self, Token, TokenType};


    
enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL
}

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,

    cur_token: Token,
    peek_token: Token,

    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer: lexer,
            errors: Vec::new(),
            cur_token: Token::default(),
            peek_token: Token::default(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        match self.cur_token.token_type {
            token::IDENT => Some(self.parse_identifier()),
            token::INT => self.parse_integer_literal(),
            token::BANG => self.parse_prefix_expression_with_operator(),
            token::MINUS => self.parse_prefix_expression_with_operator(),
            _ => {
                self.no_prefix_parse_fn_error(self.cur_token.token_type);
                None
            }
        }
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        match self.cur_token.token_type {
            token::PLUS | token::MINUS | token::ASTERISK | token::SLASH => {
                self.parse_infix_expression_with_operator(left)
            },
            token::EQ | token::NQ => {
                self.parse_infix_expression_with_operator(left)
            },
            _ => None
        }
    }

    fn parse_prefix_expression_with_operator(&mut self) -> Option<Box<dyn Expression>> {
        let operator = self.cur_token.literal.clone();
        let token = self.cur_token.clone();

        self.next_token();

        let right = self.parse_expression()?;

        Some(Box::new(
            PrefixExpression{
                token,
                operator,
                right
            }
        ))
    }

    fn parse_infix_expression_with_operator(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let operator = self.cur_token.literal.clone();
        let token = self.cur_token.clone();
        
        self.next_token();

        let right = self.parse_expression()?;

        Some(Box::new(
            InfixExpression{
                token,
                left,
                operator,
                right
            }
        ))
    }

    fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
        let msg = format!("no prefix parse function for {} found", token_type);
        self.errors.push(msg);
    }

    pub fn register_prefix(&mut self, token_type: TokenType, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    pub fn register_infix(&mut self, token_type: TokenType, func: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, func);
    }

    pub fn parse_identifier(&self) -> Box<dyn Expression> {
        Box::new(ast::Identifier{
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone()
        })
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!("expected next token to be {}, got {} instead", token_type, self.peek_token.token_type);
        self.errors.push(msg);
    }

    pub fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    pub fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            return false
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        if !self.expect_peek(token::IDENT) {
            return None
        }

        let name = Identifier{
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone()
        };

        if !self.expect_peek(token::ASSIGN) {
            return None
        }

        let stmt = LetStatement {
            token: self.cur_token.clone(),
            name: name, 
            value: None,
        };

        while !self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        } 

        Some(stmt)
    }

    pub fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        let int_from_literal = self.cur_token.literal.parse::<i64>()
            .map_err(|n| self.errors.push(format!("could not parse {} as integer", n)))
            .ok()?;

        Some(Box::new(
            IntegerLiteral{
                token: self.cur_token.clone(),
                value: int_from_literal
            }
        ))
    }

    pub fn parse_return_statement(&mut self) -> ReturnStatement {
        let stmt = ReturnStatement{
            token: self.cur_token.clone(),
            return_value: None,
        };

        self.next_token();

        while !self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        }

        stmt
    }

    pub fn parse_expression(&mut self) -> Option<Box<dyn Expression>> {
        match self.prefix_parse_fns.get(self.cur_token.token_type) {
            Some(prefix) => prefix(self),
            None => None,
        }
    }

    pub fn parse_expression_statement(&mut self) -> ExpressionStatement {
        let stmt = ExpressionStatement{
            token: self.cur_token.clone(),
            expression: self.parse_expression()
        };

        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }

        stmt
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            token::LET => {
                let stmt = self.parse_let_statement()?;
                Some(Box::new(stmt))
            },
            token::RETURN => {
                let stmt = self.parse_return_statement();
                Some(Box::new(stmt))
            },
            _ => {
                let stmt = self.parse_expression_statement();
                Some(Box::new(stmt))
            }
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while !self.cur_token_is(token::EOF) {
            statements.extend(self.parse_statement());
            self.next_token();
        }
        Program { statements }
    }
}