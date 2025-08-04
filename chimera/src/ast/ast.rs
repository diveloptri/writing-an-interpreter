use std::{any::Any, fmt::Debug};

use crate::token::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
} 

pub trait Statement: Node + Any + Debug {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node + Any + Debug {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(first) = self.statements.first() {
            first.token_literal()
        } else {
            return String::new()
        }
    }

    fn string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.string())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl Node for LetStatement{
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.string(),
            self.value.as_ref()
                .map(|v| v.string())
                .unwrap_or_else(|| String::new())
        )
    }
}

impl Expression for LetStatement{
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl Node for ReturnStatement{
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!(
            "{} {};",
            self.token_literal(),
            self.return_value.as_ref()
                .map(|v| v.string())
                .unwrap_or_else(|| String::new())
        
        )
    }
}

impl Expression for ReturnStatement{
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        if let Some(val) = &self.expression {
            return val.string()
        } 

        return String::new()
    }
}

impl Expression for ExpressionStatement {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>

}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("({} {} {})", self.left.string(), self.operator, self.right.string())
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String{
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Boolean {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>
}

impl Expression for BlockStatement {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String{
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.string())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[derive(Debug)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Expression for IfExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl Node for IfExpression {
    fn token_literal(&self) -> String{
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut result = format!("if {} {}", self.condition.string(), self.consequence.string());

        if let Some(alternative) = &self.alternative {
            result.push_str(&format!("else {}", alternative.string()));
        }

        result 
    }
}

#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let parameter = self.parameters.iter().map(|param| param.string()).collect::<Vec<String>>().join(", ");

        format!("{}({}){}", self.token_literal(), parameter, self.body.string())
    }
}
