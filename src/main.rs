mod ir;
mod ast;
mod lexer;

use lexer::*;
use ast::*;
//use std::io::*;
//use std::process::Command;

fn main() {
    for token in Lexer::from_chars("print!(34 + 35)".chars()) {
        println!("{:?}", token);
    }

    let ast = DangAst::from_tokens(Lexer::from_chars("print!(34 + 35)".chars()).peekable());
}
