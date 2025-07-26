use phf::{phf_map, Map};

pub type TokenType = &'static str;

#[derive(Debug, Clone, Default)]
pub struct Token {
   pub token_type: TokenType,
   pub literal: String 
}

pub static ILLEGAL: &str = "ILLEGAL";
pub static EOF: &str = "EOF";

// Identifiers & Literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Operators
pub static ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

pub static LT: &str = "<";
pub static GT: &str = ">";

pub const EQ: &str = "==";
pub const NQ: &str = "!=";

// Delimeters
pub static COMMA: &str = ",";
pub static SEMICOLON: &str = ";";

pub static LPAREN: &str = "(";
pub static RPAREN: &str = ")";
pub static LBRACE: &str = " {";
pub static RBRACE: &str = "}";

// Keywords
pub static FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub static TRUE: &str = "TRUE";
pub static FALSE: &str = "FALSE";
pub static IF: &str = "IF";
pub static ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";

static KEYWORDS: Map<&str, TokenType> = phf_map! {
    "fn" => FUNCTION,
    "let" => LET,
    "true" => TRUE,
    "false" => FALSE,
    "if" => IF,
    "else" => ELSE,
    "return" => RETURN,
};

pub fn lookup_ident(identifier: &str ) -> TokenType {
    if KEYWORDS.contains_key(identifier) {
        return KEYWORDS.get(identifier).unwrap()
    };
    IDENT
}