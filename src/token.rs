use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOTEQ,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::EOF => write!(f, "EOF"),
            TokenType::IDENT => write!(f, "IDENT"),
            TokenType::INT => write!(f, "INT"),
            TokenType::ASSIGN => write!(f, "ASSIGN"),
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::BANG => write!(f, "BANG"),
            TokenType::ASTERISK => write!(f, "ASTERISK"),
            TokenType::SLASH => write!(f, "SLASH"),
            TokenType::LT => write!(f, "LT"),
            TokenType::GT => write!(f, "GT"),
            TokenType::EQ => write!(f, "EQ"),
            TokenType::NOTEQ => write!(f, "NOTEQ"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::SEMICOLON => write!(f, "SEMICOLON"),
            TokenType::LPAREN => write!(f, "LPAREN"),
            TokenType::RPAREN => write!(f, "RPAREN"),
            TokenType::LBRACE => write!(f, "LBRACE"),
            TokenType::RBRACE => write!(f, "RBRACE"),
            TokenType::FUNCTION => write!(f, "FUNCTION"),
            TokenType::LET => write!(f, "LET"),
            TokenType::TRUE => write!(f, "TRUE"),
            TokenType::FALSE => write!(f, "FALSE"),
            TokenType::IF => write!(f, "IF"),
            TokenType::ELSE => write!(f, "ELSE"),
            TokenType::RETURN => write!(f, "RETURN"),
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
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        }
    }
}
