#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ILLEGAL(String),
    EOF,
    // 識別子
    IDENT(String),
    // 整数
    INT(u64),

    // 演算子
    ASSIGN,
    PLUS,
    MINUS ,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOT_EQ,

    // デリミタ
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // キーワード(識別子)
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,

}

// 識別子の判別
pub fn lookup_ident(literal: &str) -> Token {
    return match literal {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        "true" => Token::TRUE,
        "false" => Token::FALSE,
        "if" => Token::IF,
        "else" => Token::ELSE,
        "return" => Token::RETURN,
        _ => Token::IDENT(literal.to_string()),
    };
}