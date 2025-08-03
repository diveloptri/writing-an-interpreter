use std::collections::HashMap;
use crate::lexer::lexer::Lexer;
use crate::ast::ast::{self, BlockStatement, Expression, ExpressionStatement, Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, Program, ReturnStatement, Statement};
use crate::token::token::{self, Token, TokenType};


    
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd)]
pub enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL
}

pub fn get_precedences() -> HashMap<TokenType, Precedence> {
    let mut map = HashMap::new();
    map.insert(token::EQ, Precedence::EQUALS);
    map.insert(token::NQ, Precedence::EQUALS);
    map.insert(token::LT, Precedence::LESSGREATER);
    map.insert(token::GT, Precedence::LESSGREATER);
    map.insert(token::PLUS, Precedence::SUM);
    map.insert(token::MINUS, Precedence::SUM);
    map.insert(token::SLASH, Precedence::PRODUCT);
    map.insert(token::ASTERISK, Precedence::PRODUCT);
    map.insert(token::LPAREN, Precedence::CALL);

    map
}

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,

    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer: lexer,
            errors: Vec::new(),
            cur_token: Token::default(),
            peek_token: Token::default(),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::PREFIX)?;

        let expression = PrefixExpression{
            token: token,
            operator: operator,
            right: right

        };

        Some(Box::new(
            expression
        ))
        
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();

        self.next_token();

        let right = self.parse_expression(precedence)?;

        let expression = InfixExpression{
            token: token,
            operator: operator,
            left: left,
            right: right
        };

        Some(Box::new(
            expression
        ))
    }

    fn unsupported_prefix_token_error(&mut self, token_type: TokenType) {
        let msg = format!("no prefix parse function for {} found", token_type);
        self.errors.push(msg);
    }

    pub fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(ast::Identifier{
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone()
        }))
    }

    pub fn parse_boolean(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(ast::Boolean{
            token: self.cur_token.clone(),
            value: self.cur_token_is(token::TRUE),
        }))
    }

    pub fn parse_grouped_expression(&mut self) -> Option<Box<dyn Expression>> {
        self.next_token();

        let expr = self.parse_expression(Precedence::LOWEST)?;

        if !self.expect_peek(token::RPAREN) {
            return None
        }

        Some(expr)
    }

    pub fn parse_if_expression(&mut self) -> Option<Box<dyn Expression>> {
        let cur_token = self.cur_token.clone();

        if !self.expect_peek(token::LPAREN) {
            return None
        }

        self.next_token();

        let condition = self.parse_expression(Precedence::LOWEST)?;

        if !self.expect_peek(token::RPAREN) {
            return None
        }

        if !self.expect_peek(token::LBRACE) {
            return None
        }

        let consequence = self.parse_block_statement();
        let alternative = if self.peek_token_is(token::ELSE) {
            self.next_token();
            self.parse_else_block()
        } else {
            None
        };

        Some(Box::new(IfExpression{
            token: cur_token,
            condition: condition,
            consequence: consequence,
            alternative: alternative
        }))
    }

    pub fn parse_block_statement(&mut self) -> BlockStatement {
        let token = self.cur_token.clone();
        self.next_token();

        let mut statements = Vec::new();

        while !self.cur_token_is(token::RBRACE) && !self.cur_token_is(token::EOF) {
            
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }

            self.next_token();
        }
        BlockStatement { token, statements }
    }

    pub fn parse_else_block(&mut self) -> Option<BlockStatement> {
        if !self.expect_peek(token::LBRACE) {
            return None
        }
        Some(self.parse_block_statement())
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

    pub fn peek_precedence(&self) -> Precedence {
        let precedence = get_precedences();

        if let Some(precedence) = precedence.get(self.peek_token.token_type) {
            return *precedence
        } else {
            return Precedence::LOWEST
        }
    }

    pub fn cur_precedence(&self) -> Precedence {
        let precedence = get_precedences();

        if let Some(precedence) = precedence.get(self.cur_token.token_type) {
            return *precedence
        } else {
            return Precedence::LOWEST
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let let_token = self.cur_token.clone();

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

        let mut stmt = LetStatement {
            token: let_token,
            name: name, 
            value: None,
        };

        self.next_token();

        stmt.value = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(token::SEMICOLON) {
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
        let mut stmt = ReturnStatement{
            token: self.cur_token.clone(),
            return_value: None,
        };

        self.next_token();

        stmt.return_value = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }

        stmt
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let mut left_exp = match self.cur_token.token_type {
            token::IDENT => self.parse_identifier(),
            token::INT => self.parse_integer_literal(),
            token::BANG | token::MINUS => self.parse_prefix_expression(),
            token::TRUE | token::FALSE => self.parse_boolean(),
            token::LPAREN => self.parse_grouped_expression(),
            token::IF => self.parse_if_expression(),
            _ => {
                self.unsupported_prefix_token_error(&self.cur_token.token_type);
                return None;
            }
        }?;

        while !self.peek_token_is(token::SEMICOLON) && precedence < self.peek_precedence() {
            match self.peek_token.token_type {
                token::PLUS | token::MINUS | token::SLASH | token::ASTERISK | token::EQ
                | token::NQ | token::LT | token::GT => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                },
                _ => return Some(left_exp)
            }
        }
        Some(left_exp)
    }

    pub fn parse_expression_statement(&mut self) -> ExpressionStatement {
        let stmt = ExpressionStatement{
            token: self.cur_token.clone(),
            expression: self.parse_expression(Precedence::LOWEST)
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