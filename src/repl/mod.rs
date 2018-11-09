use lexer;
use std;
use std::io::Write;
use token::Token;

pub fn start() {
    loop {
        print!(">> ");
        std::io::stdout().flush();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();

        let mut lexer: lexer::Lexer = lexer::Lexer::new(s.as_ref());

        loop {
            let mut token = lexer.next_token();
            println!("{:?}", token);
            if token == Token::Eof || token == Token::Illegal {
                break;
            }
        }
    }
}
