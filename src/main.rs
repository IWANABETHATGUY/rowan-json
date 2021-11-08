use std::time::Instant;

use logos::Logos;
use rowan_json::SyntaxKind;
fn main() {
    let string = include_str!("../assets/test.json");
    let mut lex = SyntaxKind::lexer(string);
    let start = Instant::now();
    while let Some(tok) = lex.next() {
        // println!("{:?}: {}", tok, lex.slice());
    }
    println!("{:?}", start.elapsed());
}
