mod ir;
mod ast;
mod lexer;

use lexer::*;
use ast::*;

//use std::io::*;
//use std::process::Command;

fn main() {
    let ast = DangAst::from_tokens(Lexer::from_chars("print!(34 + 35, 10)".chars()).peekable());
    println!("{}", ast)
}
