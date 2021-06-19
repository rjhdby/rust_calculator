#[macro_use]
extern crate lazy_static;
extern crate rug;

use std::io::{stdout, Write};
use std::process::exit;

use clap::{AppSettings, Clap};
use strum::IntoEnumIterator;

use crate::ast::build_ast;
use crate::operation::Operation;
use crate::token::tokenize;

mod operation;
mod token;
mod validator;
mod ast;
mod context;

#[derive(Clap)]
#[clap(version = "0.1", author = "Andrey G. <rjhdbylive@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, about = "Calculate expression. \nE.g \"-10+sin(23)/(2e10 -1.3)\"", value_name = "expr")]
    calculate: Option<String>,
    #[clap(short, long, about = "Start interactive shell")]
    interactive: bool,
    #[clap(short, long, about = "Supported operations list")]
    list: bool,

}

fn main() {
    let opts: Opts = Opts::parse();

    if opts.calculate.is_some() {
        calculate(&opts.calculate.unwrap())
    } else if opts.list {
        print_operators()
    } else if opts.interactive {
        print_interactive_help();
        loop {
            let mut buffer = String::new();
            print!("> ");
            stdout().flush().unwrap();
            std::io::stdin().read_line(&mut buffer).unwrap();
            buffer = String::from(buffer.trim());
            match buffer.as_str() {
                "" => print_interactive_help(),
                "exit" => exit(0),
                "list" => print_operators(),
                _ => calculate(&buffer)
            }
        }
    } else {
        println!("Use flag --help for usage information")
    }
}

fn print_operators() {
    println!("Supported operations");
    Operation::iter().for_each(
        |it| println!("{}", it.pretty("x".to_string(), Option::Some("y".to_string())))
    )
}

fn print_interactive_help() {
    println!("Type 'exit' for exit and 'list' for supported operations list.")
}

fn calculate(buffer: &str) {
    let tokens = tokenize(&buffer);
    if tokens.is_err() {
        let err = &tokens.err().unwrap();
        println!("{}", buffer);
        println!(" {:>1$}", "^", err.get_pos());
        println!("[syntax error] {}", err);
        return;
    }
    let ast = build_ast(&tokens.ok().unwrap());
    if ast.is_err() {
        let err = &ast.err().unwrap();
        println!("[syntax error] {}", err);
        return;
    }
    let ast_result = ast.unwrap();
    let result = ast_result.calculate();
    if result.is_err() {
        println!("[logic error] {}", &result.err().unwrap());
        return;
    }
    println!("{}", result.unwrap().to_string());
}