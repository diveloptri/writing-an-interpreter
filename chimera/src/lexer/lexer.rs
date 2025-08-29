use crate::token::token::*;

#[derive(Default, Debug, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: i32,
    pub read_position: i32,
    pub character: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut new_lexer = Self {input, ..Default::default()};
        new_lexer.read_char();
        new_lexer
    }

    fn read_char(&mut self) {
        if self.read_position as usize >= self.input.len() {
            self.character = 0;
        } else {
            let n_char = self.read_position as usize;
            self.character = self.input
                .chars()
                .nth(n_char)
                .unwrap() as u8;
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut token = Token { ..Default::default() };

        self.skip_whitespace();

        match self.character {
            b'=' =>  if self.peek_char() == b'=' {
                        let curr_char = self.character as char;
                        self.read_char();
                        let literal = format!("{}{}", curr_char, self.character as char);
                        token = Token{token_type: EQ, literal};
                    } else {
                        token = new_token(ASSIGN, self.character)
                    },
            b'+' =>  token = new_token(PLUS, self.character),
            b'-' =>  token = new_token(MINUS, self.character),
            b'!' =>  if self.peek_char() == b'=' {
                        let curr_char = self.character as char;
                        self.read_char();
                        let literal = format!("{}{}", curr_char, self.character as char);
                        token = Token{token_type: NQ, literal};
                    } else {
                        token = new_token(BANG, self.character)
                    },
            b'*' =>  token = new_token(ASTERISK, self.character),
            b'/' =>  token = new_token(SLASH, self.character),
            b'<' =>  token = new_token(LT, self.character),
            b'>' =>  token = new_token(GT, self.character),
            b',' =>  token = new_token(COMMA, self.character),
            b';' =>  token = new_token(SEMICOLON, self.character),
            b'(' =>  token = new_token(LPAREN, self.character),
            b')' =>  token = new_token(RPAREN, self.character),
            b'{' =>  token = new_token(LBRACE, self.character),
            b'}' =>  token = new_token(RBRACE, self.character),
            b'[' =>  token = new_token(LBRACKET, self.character),
            b']' =>  token = new_token(RBRACKET, self.character),
            b'"' => {
                let literal = self.read_string();
                token = Token{token_type: STRING, literal: literal}
            },
            b':' => {
                token = new_token(COLON, self.character)
            },
            _ => {
                if self.character.is_ascii_alphabetic(){
                    token.literal = self.read_identifier();
                    token.token_type = lookup_ident(&token.literal);
                    return token
                } else if self.character.is_ascii_digit() {
                    token.literal = self.read_number();
                    token.token_type = INT;
                    return token
                } else if self.character.is_ascii_control() {
                    token.literal = "".to_string();
                    token.token_type = EOF;
                } else {
                    token = new_token(ILLEGAL, self.character)
                }
            }  
        }

        self.read_char();
        return token
    }

    fn read_identifier(&mut self) -> String {
        let position= self.position;
        
        while self.character.is_ascii_alphabetic() {
            self.read_char();
        }

        return self.input.get(
            position as usize
            ..
            self.position as usize
        ).unwrap().to_string()
    }

    fn read_number(&mut self) -> String {
        let position= self.position;
        
        while self.character.is_ascii_digit() {
            self.read_char();
        }

        return self.input.get(
            position as usize
            ..
            self.position as usize
        ).unwrap().to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.character.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position as usize >= self.input.len() {
            return 0
        } else {
            return self.input
                .chars()
                .nth(self.read_position as usize)
                .unwrap() as u8
        }
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.character == b'"' || self.character == 0 {
                break;
            }
        }
        self.input.get(
            position as usize
            ..
            self.position as usize
        ).unwrap().to_string()
    }
}

pub fn new_token(token_type: TokenType, char: u8) -> Token {
    let character = char as char;
    return Token { token_type, literal: character.to_string(), }
}
