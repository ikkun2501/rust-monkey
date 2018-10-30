use token::Token;

struct Lexer {
    input: &str,
    position: u64,
    read_position: int64,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer { input, position: 0, read_position: 0, ch: 0 };
        lexer.read_char();
    }

    pub fn read_char(&mut self){

    }
}

#[cfg(test)]
mod tests {
    use lexer::Lexer;
    use token::Token;

    #[test]
    fn test_next_token() {
        let input = "+";
        let tests: Vec<Token> = vec![
            Token::ASSIGN,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }
}