extern crate rust_monkey;

use rust_monkey::repl;

fn main() {
//    repl::start();
    let mut repl: repl::Repl = repl::Repl::new();
    println!("{}", repl.input("let aiueo = 1;"));
}
