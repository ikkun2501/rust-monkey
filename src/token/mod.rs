// TODO IDENTとLITERALの違いは？
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ILLEGAL(String),
    EOF,
    // 識別子＋リテラル
    IDENT(String),
    // 整数
    INT(u64),

    // 演算子
    ASSIGN,
    PLUS,

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
    LITERAL(String),

}

// 識別子の判別
pub fn lookup_ident(literal: &str) -> Token{
    return match literal {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        _ => Token::IDENT(literal.to_string()),
    };
}