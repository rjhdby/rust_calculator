use crate::token::tokenize;
use crate::ast::build_ast;
use std::process::exit;

#[macro_use]
extern crate lazy_static;

mod operation;
mod token;
mod ast;

fn main() {
    let s = "123.5*ln(10)/(3-7)+ 15^2*sin(3) + 2e3 - 2.1E-3 + 1e-2";
    println!("{}", s);
    let tokens = tokenize(s);
    if tokens.is_err() {
        println!("<error> {}", &tokens.err().unwrap());
        exit(1)
    }
    let ast = build_ast(&tokens.unwrap()).unwrap();
    println!("{} = {}", ast, ast.calculate());
}