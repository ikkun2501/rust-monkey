use token::lookup_ident;
use token::Token;

struct Lexer<'a> {
    // 入力された文字列（ソース）
    input: &'a str,
    // 現在位置
    position: usize,
    // 読み込む位置
    read_position: usize,
    // 現在読み込んだ文字（バイト）
    ch: u8,
}

impl<'a> Lexer<'a> {
    // インスタンスの生成
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer { input, position: 0, read_position: 0, ch: 0 };
        lexer.read_char();
        return lexer;
    }

    // 文字の読み込み
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // TOKENの取得
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            b'+' => Token::PLUS,
            b'=' => Token::ASSIGN,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'!' => Token::BANG,
            b'-' => Token::MINUS,
            b'/' => Token::SLASH,
            b'*' => Token::ASTERISK,
            b'<' => Token::LT,
            b'>' => Token::GT,
            0 => Token::EOF,
            _ => if is_letter(self.ch) {
                return lookup_ident(self.read_identifier());
            } else if is_digit(self.ch) {
                return Token::INT(self.read_number());
            } else {
                return Token::ILLEGAL(self.ch.to_string());
            }
        };

        self.read_char();
        return token;
    }

    // 文字列の読み込み
    pub fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        let s = &self.input[position..self.position];
        println!("read_identifier:{}", s);
        return s;
    }

    // 文字列の読み込み
    pub fn read_number(&mut self) -> u64 {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        return self.input[position..self.position].parse().unwrap();
    }

    // 空白をスキップ
    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
}

// 文字列判定
fn is_letter(ch: u8) -> bool {
    println!("is_letter arg:ch:{}, ch -> string:{}", ch, char::from(ch));
    println!("a(byte):{}", b'a');
    println!("z(byte):{}", b'z');

    return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_';
}

// 数値判定
fn is_digit(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9';
}

#[cfg(test)]
mod tests {
    use lexer::Lexer;
    use token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"
let five = 5;
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
10 != 9;
         "#;
        let tests: Vec<Token> = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(10),
            Token::EQ,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(10),
            Token::NOT_EQ,
            Token::INT(9),
            Token::SEMICOLON,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }
}