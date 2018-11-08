use token::lookup_ident;
use token::Token;

pub struct Lexer<'a> {
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
            b'+' => Token::Plus,
            b'=' => if self.peek_char() == b'=' {
                self.read_char();
                Token::Eq
            } else {
                Token::Assign
            },
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'!' => if self.peek_char() == b'=' {
                self.read_char();
                Token::NotEq
            } else {
                Token::Bang
            },
            b'-' => Token::Minus,
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            0 => Token::Eof,
            _ => if is_letter(self.ch) {
                return lookup_ident(self.read_identifier());
            } else if is_digit(self.ch) {
                return Token::Int(self.read_number());
            } else {
                return Token::Illegal(self.ch.to_string());
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

    // 数値の読み込み
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
    fn peek_char(&mut self) -> u8 {
        let peek_char = self.input.as_bytes()[self.read_position];
        println!("peek_char:{}", char::from(peek_char));
        return peek_char;
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
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::GreaterThan,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::Bool(true),
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::Bool(false),
            Token::Semicolon,
            Token::Rbrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }
}