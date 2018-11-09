#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    // 識別子
    Ident(String),
    // 整数
    Int(u64),

    // 演算子
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Eq,
    NotEq,

    // デリミタ
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // キーワード(識別子)
    Function,
    Let,
    Bool(bool),
    If,
    Else,
    Return,

}

// 識別子の判別
pub fn lookup_ident(literal: &str) -> Token {
    return match literal {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(String::from(literal)),
    };
}