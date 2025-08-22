use std::collections::HashMap;
use crate::lexer::lexer::Lexer;
use crate::ast::ast::{self, BlockStatement, CallExpressionWrapped, ExpressionStatement, ExpressionType, FunctionLiteral, Identifier, IfExpressionWrapped, InfixExpressionWrapped, IntegerLiteral, LetStatement, PrefixExpressionWrapped, Program, ReturnStatement, StatementType};
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

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while !self.cur_token_is(token::EOF) {
            statements.extend(self.parse_statement());
            self.next_token();
        }
        Program { statements }
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<ExpressionType> {
        let mut left_exp = match self.cur_token.token_type {
            token::IDENT => self.parse_identifier(),
            token::INT => self.parse_integer_literal(),
            token::BANG | token::MINUS => self.parse_prefix_expression(),
            token::TRUE | token::FALSE => self.parse_boolean(),
            token::LPAREN => self.parse_grouped_expression(),
            token::IF => self.parse_if_expression(),
            token::FUNCTION => self.parse_function_literal(),
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
                token::LPAREN => {
                    self.next_token();
                    left_exp = self.parse_call_expression(left_exp)?;
                },
                _ => return Some(left_exp)
            }
        }
        Some(left_exp)
    }

    fn parse_prefix_expression(&mut self) -> Option<ExpressionType> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::PREFIX)?;

        let expression = PrefixExpressionWrapped {
            token: token,
            operator: operator,
            right: Box::new(right),
        };

        Some(ExpressionType::PrefixExpression(expression))
    }

    fn parse_infix_expression(&mut self, left: ExpressionType) -> Option<ExpressionType> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();

        self.next_token();

        let right = self.parse_expression(precedence)?;

        let expression = InfixExpressionWrapped {
            token: token,
            operator: operator,
            left: Box::new(left),
            right: Box::new(right),
        };

        Some(ExpressionType::InfixExpression(expression))
    }

    fn unsupported_prefix_token_error(&mut self, token_type: TokenType) {
        let msg = format!("no prefix parse function for {} found", token_type);
        self.errors.push(msg);
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

    pub fn parse_identifier(&mut self) -> Option<ExpressionType> {
        Some(ExpressionType::Identifier(ast::Identifier{
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone()
        }))
    }

    pub fn parse_boolean(&mut self) -> Option<ExpressionType> {
        Some(ExpressionType::Boolean(ast::Boolean{
            token: self.cur_token.clone(),
            value: self.cur_token_is(token::TRUE),
        }))
    }

    pub fn parse_grouped_expression(&mut self) -> Option<ExpressionType> {
        self.next_token();

        let expr = self.parse_expression(Precedence::LOWEST)?;

        if !self.expect_peek(token::RPAREN) {
            return None
        }

        Some(expr)
    }

    pub fn parse_if_expression(&mut self) -> Option<ExpressionType> {
        let cur_token = self.cur_token.clone();

        if !self.expect_peek(token::LPAREN) {
            self.errors.push("Expected '(' after 'if' keyword".to_string());
            return None
        }

        self.next_token();

        let condition = self.parse_expression(Precedence::LOWEST)?;

        if !self.expect_peek(token::RPAREN) {
            self.errors.push("Expected ')' after if condition".to_string());
            return None
        }

        if !self.expect_peek(token::LBRACE) {
            self.errors.push("Expected '{' after if condition".to_string());
            return None
        }

        let consequence = self.parse_block_statement();
        let alternative = if self.peek_token_is(token::ELSE) {
            self.next_token();
            self.parse_else_block()
        } else {
            None
        };

        Some(ExpressionType::IfExpression(IfExpressionWrapped{
            token: cur_token,
            condition: Box::new(condition),
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
            self.errors.push("Expected '{' after 'else' keyword".to_string());
            return None
        }
        Some(self.parse_block_statement())
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

    pub fn parse_integer_literal(&mut self) -> Option<ExpressionType> {
        let int_from_literal = self.cur_token.literal.parse::<i64>()
            .map_err(|_| self.errors.push(format!("Invalid integer literal: '{}'", self.cur_token.literal)))
            .ok()?;

        Some(ExpressionType::IntegerLiteral(
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

    pub fn parse_function_literal(&mut self) -> Option<ExpressionType> {
        let cur_token = self.cur_token.clone();

        if !self.expect_peek(token::LPAREN) {
            self.errors.push("Expected '(' after 'fn' keyword".to_string());
            return None
        };

        let parameters = self.parse_function_parameters()?;

        if !self.expect_peek(token::LBRACE) {
            self.errors.push("Expected '{' after function parameters".to_string());
            return None
        };

        let body = self.parse_block_statement();

        Some(ExpressionType::FunctionLiteral(
            FunctionLiteral{
                token: cur_token,
                parameters: parameters,
                body: body
        }))
    }

    pub fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(token::RPAREN) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        identifiers.push(self.create_identifier_from_current_token());

        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();
            identifiers.push(self.create_identifier_from_current_token());
        }

        if !self.expect_peek(token::RPAREN) {
            self.errors.push("Expected ')' after function parameters".to_string());
            return None
        }

        Some(identifiers)
    }

    fn create_identifier_from_current_token(&self) -> Identifier {
        Identifier { token: self.cur_token.clone(), value: self.cur_token.literal.clone() }
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

    pub fn parse_statement(&mut self) -> Option<StatementType> {
        match self.cur_token.token_type {
            token::LET => {
                let stmt = self.parse_let_statement()?;
                Some(StatementType::Let(stmt))
            },
            token::RETURN => {
                let stmt = self.parse_return_statement();
                Some(StatementType::Return(stmt))
            },
            _ => {
                let stmt = self.parse_expression_statement();
                Some(StatementType::Expression(stmt))
            }
        }
    }

    pub fn parse_call_expression(&mut self, function: ExpressionType) -> Option<ExpressionType> {
        let expr_arguments = self.parse_call_arguments();

        Some(ExpressionType::CallExpression(CallExpressionWrapped{
            token: self.cur_token.clone(), function: Box::new(function), arguments: expr_arguments
        }))
    }

    pub fn parse_call_arguments(&mut self) -> Vec<ExpressionType> {
        if self.peek_token_is(token::RPAREN) {
            self.next_token();
            return Vec::new()
        }

        self.next_token();
        let mut arguments= Vec::new();
        
        if let Some(expr) = self.parse_expression(Precedence::LOWEST) {
            arguments.push(expr);
        }

        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();

            if let Some(expr) = self.parse_expression(Precedence::LOWEST) {
                arguments.push(expr);
            }
        }

        if !self.expect_peek(token::RPAREN) {
            self.errors.push("Expected ')' after call arguments".to_string());
            return Vec::new()
        }

        arguments
    }

}