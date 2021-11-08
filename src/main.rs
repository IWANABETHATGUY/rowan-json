use std::time::Instant;

use logos::Logos;
use rowan_json::Token;
fn main() {
    let string = include_str!("../assets/big.json");
    let mut lex = Token::lexer(string);
    let start = Instant::now();
    while let Some(tok) = lex.next() {
    }
    println!("{:?}", start.elapsed());
}
