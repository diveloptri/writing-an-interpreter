use std::{any::Any, fmt::Debug};

use crate::token::token::Token;

pub trait Node: Any{
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
} 

pub trait Statement: Node + Any + Debug {
    fn statement_node(&self);
}

pub trait Expression: Node + Any + Debug {
    fn expression_node(&self);
}

#[derive(Debug, Clone)]
pub enum StatementType {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Node for StatementType {
    fn token_literal(&self) -> String {
        match self {
            StatementType::Let(stmt) => stmt.token_literal(),
            StatementType::Return(stmt) => stmt.token_literal(),
            StatementType::Expression(stmt) => stmt.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            StatementType::Let(stmt) => stmt.string(),
            StatementType::Return(stmt) => stmt.string(),
            StatementType::Expression(stmt) => stmt.string(),
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    IntegerLiteral(IntegerLiteral),
    Boolean(Boolean),
    PrefixExpression(PrefixExpressionWrapped),
    InfixExpression(InfixExpressionWrapped),
    IfExpression(IfExpressionWrapped),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpressionWrapped),
    BlockStatement(BlockStatement),
}

impl Node for ExpressionType {
    fn token_literal(&self) -> String {
        match self {
            ExpressionType::Identifier(expr) => expr.token_literal(),
            ExpressionType::StringLiteral(expr) => expr.token_literal(),
            ExpressionType::IntegerLiteral(expr) => expr.token_literal(),
            ExpressionType::Boolean(expr) => expr.token_literal(),
            ExpressionType::PrefixExpression(expr) => expr.token_literal(),
            ExpressionType::InfixExpression(expr) => expr.token_literal(),
            ExpressionType::IfExpression(expr) => expr.token_literal(),
            ExpressionType::FunctionLiteral(expr) => expr.token_literal(),
            ExpressionType::CallExpression(expr) => expr.token_literal(),
            ExpressionType::BlockStatement(expr) => expr.token_literal(),
        }
    }
    
    fn string(&self) -> String {
        match self {
            ExpressionType::Identifier(expr) => expr.string(),
            ExpressionType::StringLiteral(expr) => expr.string(),
            ExpressionType::IntegerLiteral(expr) => expr.string(),
            ExpressionType::Boolean(expr) => expr.string(),
            ExpressionType::PrefixExpression(expr) => expr.string(),
            ExpressionType::InfixExpression(expr) => expr.string(),
            ExpressionType::IfExpression(expr) => expr.string(),
            ExpressionType::FunctionLiteral(expr) => expr.string(),
            ExpressionType::CallExpression(expr) => expr.string(),
            ExpressionType::BlockStatement(expr) => expr.string(),
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}

pub struct Program {
    pub statements: Vec<StatementType>,
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

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<ExpressionType>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
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

    fn as_any(&self) -> &dyn Any { self }
}

impl Expression for LetStatement{
    fn expression_node(&self) {}
}

#[derive(Debug, PartialEq, Clone)]
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

    fn as_any(&self) -> &dyn Any { self }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<ExpressionType>,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
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

    fn as_any(&self) -> &dyn Any { self }
}

impl Expression for ReturnStatement{
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<ExpressionType>,
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
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

    fn as_any(&self) -> &dyn Any { self }
}

impl Expression for ExpressionStatement {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct PrefixExpressionWrapped {
    pub token: Token,
    pub operator: String,
    pub right: Box<ExpressionType>

}

impl Node for PrefixExpressionWrapped {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }

    fn as_any(&self) -> &dyn Any { self }
}

impl Expression for PrefixExpressionWrapped {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct InfixExpressionWrapped {
    pub token: Token,
    pub left: Box<ExpressionType>,
    pub operator: String,
    pub right: Box<ExpressionType>
}

impl Node for InfixExpressionWrapped {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("({} {} {})", self.left.string(), self.operator, self.right.string())
    }

    fn as_any(&self) -> &dyn Any { self }
}

impl Expression for InfixExpressionWrapped {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
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

    fn as_any(&self) -> &dyn Any { self }
}

impl Expression for Boolean {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<StatementType>
}

impl Expression for BlockStatement {
    fn expression_node(&self) {}
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

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct IfExpressionWrapped {
    pub token: Token,
    pub condition: Box<ExpressionType>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Expression for IfExpressionWrapped {
    fn expression_node(&self) {}
}

impl Node for IfExpressionWrapped {
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

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let parameter = self.parameters.iter().map(|param| param.string()).collect::<Vec<String>>().join(", ");

        format!("{}({}){}", self.token_literal(), parameter, self.body.string())
    }

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct CallExpressionWrapped {
    pub token: Token,
    pub function: Box<ExpressionType>,
    pub arguments: Vec<ExpressionType>
}

impl Expression for CallExpressionWrapped {
    fn expression_node(&self) {}
}

impl Node for CallExpressionWrapped {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let args = self.arguments.iter().map(|arg| arg.string()).collect::<Vec<String>>().join(", ");

        format!("{}({})", self.function.string(), args) 
    }

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl Expression for StringLiteral {
    fn expression_node(&self) {}
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any { self }
}