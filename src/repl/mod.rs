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

        let token = lexer.next_token();
        while token != Token::Eof {
            println!("{:?}", token);
            let token = lexer.next_token();
        }
//        println!("{}", s)
    }
}
