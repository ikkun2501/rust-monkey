struct Token {
    token_type: TokenType,
    literal: String,
}

enum TokenType {
    ILLEGAL,
    EOF,
    // 識別子＋リテラル
    IDENT,
    // 整数
    INT,

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