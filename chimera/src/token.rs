use phf::{phf_map, Map};

pub type TokenType = &'static str;

#[derive(Default, Debug)]
pub struct Token {
   pub token_type: TokenType,
   pub literal: String 
}

pub static ILLEGAL: &str = "ILLEGAL";
pub static EOF: &str = "EOF";

// Identifiers & Literals
pub static IDENT: &str = "IDENT";
pub static INT: &str = "INT";

// Operators
pub static ASSIGN: &str = "=";
pub static PLUS: &str = "+";
pub static MINUS: &str = "-";
pub static BANG: &str = "!";
pub static ASTERISK: &str = "*";
pub static SLASH: &str = "/";

pub static LT: &str = "<";
pub static GT: &str = ">";

pub static EQ: &str = "==";
pub static NQ: &str = "!=";

// Delimeters
pub static COMMA: &str = ",";
pub static SEMICOLON: &str = ";";

pub static LPAREN: &str = "(";
pub static RPAREN: &str = ")";
pub static LBRACE: &str = " {";
pub static RBRACE: &str = "}";

// Keywords
pub static FUNCTION: &str = "FUNCTION";
pub static LET: &str = "LET";
pub static TRUE: &str = "TRUE";
pub static FALSE: &str = "FALSE";
pub static IF: &str = "IF";
pub static ELSE: &str = "ELSE";
pub static RETURN: &str = "RETURN";

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