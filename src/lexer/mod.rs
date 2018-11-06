use token::Token;

struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer { input, position: 0, read_position: 0, ch: 0 };
        lexer.read_char();
        return lexer;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            b'+' => Token::PLUS,
            b'=' => Token::ASSIGN,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            0 => Token::EOF,
            _ => if is_letter(self.ch) {
                return Token::IDENT(self.read_identifier());
            } else {
                return Token::ILLEGAL(self.ch.to_string());
            }
        };

        self.read_char();
        return token;
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        let s = self.input[position..self.position].to_string();
        println!("read_identifier:{}", s);
        return s;
    }
}

fn is_letter(ch: u8) -> bool {
    println!("is_letter arg:ch:{}, ch -> string:{}", ch, char::from(ch));
    println!("a(byte):{}", b'a');
    println!("z(byte):{}", b'z');

    return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_';
}

#[cfg(test)]
mod tests {
    use lexer::Lexer;
    use token::Token;

    #[test]
    fn test_next_token() {
//        let input = r#""
//         let five = 5;
//         let ten = 10;
//         let add = fn( x, y) { x + y; };
//         let result = add( five, ten);
//         ""#;
        let input = r#"aiueo"#;
        let tests: Vec<Token> = vec![
//            Token::LET,
Token::IDENT(String::from("aiueo")),
//            Token::ASSIGN,
//            Token::INT(5),
//            Token::SEMICOLON,
Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }
}