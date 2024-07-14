use L::{lexer::Lexer, token::TokenType};

#[test]
fn test_next_token() {
    let input = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
        x + y;
    };
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    if (5 < 10) {
       return true;
    } else {
       return false;
    }
    10 == 10;
    10 != 9;"
        .to_owned();

    let tests: Vec<(String, TokenType)> = vec![
        ("let".to_string(), TokenType::LET),
        ("five".to_string(), TokenType::IDENT),
        ("=".to_string(), TokenType::ASSIGN),
        ("5".to_string(), TokenType::INT),
        (";".to_string(), TokenType::SEMICOLON),
        ("let".to_string(), TokenType::LET),
        ("ten".to_string(), TokenType::IDENT),
        ("=".to_string(), TokenType::ASSIGN),
        ("10".to_string(), TokenType::INT),
        (";".to_string(), TokenType::SEMICOLON),
        ("let".to_string(), TokenType::LET),
        ("add".to_string(), TokenType::IDENT),
        ("=".to_string(), TokenType::ASSIGN),
        ("fn".to_string(), TokenType::FUNCTION),
        ("(".to_string(), TokenType::LPAREN),
        ("x".to_string(), TokenType::IDENT),
        (",".to_string(), TokenType::COMMA),
        ("y".to_string(), TokenType::IDENT),
        (")".to_string(), TokenType::RPAREN),
        ("{".to_string(), TokenType::LBRACE),
        ("x".to_string(), TokenType::IDENT),
        ("+".to_string(), TokenType::PLUS),
        ("y".to_string(), TokenType::IDENT),
        (";".to_string(), TokenType::SEMICOLON),
        ("}".to_string(), TokenType::RBRACE),
        (";".to_string(), TokenType::SEMICOLON),
        ("let".to_string(), TokenType::LET),
        ("result".to_string(), TokenType::IDENT),
        ("=".to_string(), TokenType::ASSIGN),
        ("add".to_string(), TokenType::IDENT),
        ("(".to_string(), TokenType::LPAREN),
        ("five".to_string(), TokenType::IDENT),
        (",".to_string(), TokenType::COMMA),
        ("ten".to_string(), TokenType::IDENT),
        (")".to_string(), TokenType::RPAREN),
        (";".to_string(), TokenType::SEMICOLON),
        ("!".to_string(), TokenType::BANG),
        ("-".to_string(), TokenType::MINUS),
        ("/".to_string(), TokenType::SLASH),
        ("*".to_string(), TokenType::ASTERISK),
        ("5".to_string(), TokenType::INT),
        (";".to_string(), TokenType::SEMICOLON),
        ("5".to_string(), TokenType::INT),
        ("<".to_string(), TokenType::LT),
        ("10".to_string(), TokenType::INT),
        (">".to_string(), TokenType::GT),
        ("5".to_string(), TokenType::INT),
        (";".to_string(), TokenType::SEMICOLON),
        ("if".to_string(), TokenType::IF),
        ("(".to_string(), TokenType::LPAREN),
        ("5".to_string(), TokenType::INT),
        ("<".to_string(), TokenType::LT),
        ("10".to_string(), TokenType::INT),
        (")".to_string(), TokenType::RPAREN),
        ("{".to_string(), TokenType::LBRACE),
        ("return".to_string(), TokenType::RETURN),
        ("true".to_string(), TokenType::TRUE),
        (";".to_string(), TokenType::SEMICOLON),
        ("}".to_string(), TokenType::RBRACE),
        ("else".to_string(), TokenType::ELSE),
        ("{".to_string(), TokenType::LBRACE),
        ("return".to_string(), TokenType::RETURN),
        ("false".to_string(), TokenType::FALSE),
        (";".to_string(), TokenType::SEMICOLON),
        ("}".to_string(), TokenType::RBRACE),
        ("10".to_string(), TokenType::INT),
        ("==".to_string(), TokenType::EQ),
        ("10".to_string(), TokenType::INT),
        (";".to_string(), TokenType::SEMICOLON),
        ("10".to_string(), TokenType::INT),
        ("!=".to_string(), TokenType::NOTEQ),
        ("9".to_string(), TokenType::INT),
        (";".to_string(), TokenType::SEMICOLON),
        ("\0".to_string(), TokenType::EOF),
    ];
    let mut lex = Lexer::new(input);

    for (i, t) in tests.iter().enumerate() {
        let tok = lex.next_token();
        assert!(
            tok.literal == t.0.to_string(),
            "{}",
            format!(
                "Test {} Expected token type: {:?}, found {:?}",
                i, t.0, tok.literal
            )
        );
        assert!(
            tok.ttype == t.1,
            "{}",
            format!(
                "Test {} Expected token type: {:?}, found {:?}",
                i, t.1, tok.ttype
            )
        );
    }
}
