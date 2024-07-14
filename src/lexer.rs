use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,      // current position in input (current character)
    read_position: usize, // current reading position in input (after current character)
    ch: char,             // current character
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: char::default(),
        };
        lex.read_char();
        lex
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();
        let ttype = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char(); // to read the next =
                    self.read_char(); // to move to the next char
                    return Token {
                        ttype: TokenType::EQ,
                        literal: "==".to_string(),
                    };
                } else {
                    TokenType::ASSIGN
                }
            }
            '+' => TokenType::PLUS,
            '-' => TokenType::MINUS,
            '*' => TokenType::ASTERISK,
            '/' => TokenType::SLASH,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char(); // to read the next =
                    self.read_char(); // to move to the next char
                    return Token {
                        ttype: TokenType::NOTEQ,
                        literal: "!=".to_string(),
                    };
                } else {
                    TokenType::BANG
                }
            }
            '>' => TokenType::GT,
            '<' => TokenType::LT,
            ';' => TokenType::SEMICOLON,
            '{' => TokenType::LBRACE,
            '}' => TokenType::RBRACE,
            '(' => TokenType::LPAREN,
            ')' => TokenType::RPAREN,
            ',' => TokenType::COMMA,
            '\0' => TokenType::EOF,
            b => {
                if b.is_alphabetic() {
                    let literal = self.read_identifier();
                    return Token {
                        ttype: TokenType::lookup_ident(&literal),
                        literal,
                    };
                } else if b.is_digit(10) {
                    let literal = self.read_number();
                    return Token {
                        ttype: TokenType::INT,
                        literal,
                    };
                } else {
                    TokenType::ILLEGAL
                }
            }
        };

        let literal = self.ch.to_string();
        self.read_char(); // to move to the next char
        Token { ttype, literal }
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    pub fn skip_whitespaces(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}
