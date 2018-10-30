
#[derive(Debug, Clone, PartialEq)]
pub enum Token{
    ILLEGAL,
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

    // キーワード
    FUNCTION,
    LET,

}