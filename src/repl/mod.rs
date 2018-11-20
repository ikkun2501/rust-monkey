use cfg_if::cfg_if;
use lexer;
use std;
use std::io::Write;
use token::Token;
use wasm_bindgen::prelude::*;

cfg_if! {
    // TODO これはなに？
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub struct Repl {}

#[wasm_bindgen]
impl Repl {
    // インスタンスの生成
    pub fn new() -> Repl { return Repl {}; }


    pub fn input(&mut self, s: &str) -> String {
        let mut lexer: lexer::Lexer = lexer::Lexer::new(s);

        let mut output: Vec<Token> = vec![];
        loop {
            let token = lexer.next_token();
            output.push(token.clone());
            if token == Token::Eof || token == Token::Illegal {
                break;
            }
        }
        return output.iter().map(|x| format!("{:?}\r\n", x)).collect();
    }
}

//pub fn start() {
//    loop {
//        print!(">> ");
//        std::io::stdout().flush();
//        let mut s = String::new();
//        std::io::stdin().read_line(&mut s).ok();
//
//        let mut lexer: lexer::Lexer = lexer::Lexer::new(s.as_ref());
//
//        loop {
//            let token = lexer.next_token();
//            println!("{:?}", token);
//            if token == Token::Eof || token == Token::Illegal {
//                break;
//            }
//        }
//    }
//}
