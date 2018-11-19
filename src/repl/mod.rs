use lexer;
use std;
use std::io::Write;
use token::Token;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
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

        loop {
            let mut token = lexer.next_token();
            println!("{:?}", token);
            if token == Token::Eof || token == Token::Illegal {
                break;
            }
        }
        return "test".to_string();
    }
}

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
