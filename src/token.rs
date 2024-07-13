use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::EOF => write!(f, "EOF"),
            TokenType::IDENT => write!(f, "IDENT"),
            TokenType::INT => write!(f, "INT"),
            TokenType::ASSIGN => write!(f, "ASSIGN"),
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::SEMICOLON => write!(f, "SEMICOLON"),
            TokenType::LPAREN => write!(f, "LPAREN"),
            TokenType::RPAREN => write!(f, "RPAREN"),
            TokenType::LBRACE => write!(f, "LBRACE"),
            TokenType::RBRACE => write!(f, "RBRACE"),
            TokenType::FUNCTION => write!(f, "FUNCTION"),
            TokenType::LET => write!(f, "LET"),
            TokenType::ILLEGAL => write!(f, "ILLEGAL"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
}

impl TokenType {
    pub fn lookup_ident(ident: &String) -> Self {
        match ident.as_str() {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            _ => TokenType::IDENT,
        }
    }
}
