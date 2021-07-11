#[macro_use]
extern crate lazy_static;
extern crate rug;

use std::io::{stdout, Write};
use std::process::exit;

use clap::{AppSettings, Clap};
use crate::ast::{ExprCalculator};
use rug::Float;

mod lambdas;
mod ast;

#[derive(Clap)]
#[clap(version = "0.1", author = "Andrey G. <rjhdbylive@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, about = "Calculate expression. \nE.g \"-10+sin(23)/(2e10 -1.3)\"", value_name = "expr")]
    float_calc: Option<String>,
    #[clap(short, long, about = "Calculate boolean expression. \nE.g \"true | false ^ (true&!false)\"", value_name = "expr")]
    bool_calc: Option<String>,
    #[clap(short, long, about = "Start interactive shell")]
    interactive: bool,
    #[clap(short, long, about = "Supported operations list")]
    list: bool,

}

fn main() {
    let opts: Opts = Opts::parse();

    if opts.float_calc.is_some() {
        calculate(&opts.float_calc.unwrap())
    } else if opts.bool_calc.is_some() {
        calculate_bool(&opts.bool_calc.unwrap())
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
                // it if it.starts_with("add") => ,
                _ => calculate(&buffer)
            }
        }
    } else {
        println!("Use flag --help for usage information")
    }
}

fn print_operators() {
    println!("Supported operations");
    let calculator = ExprCalculator::<Float>::float_calculator();
    for op in calculator.operations {
        println!(
            "{:<15} {}",
            op.pretty("x".to_string(), Option::Some("y".to_string())),
            op.description()
        )
    }
}

fn print_interactive_help() {
    println!("Type 'exit' for exit and 'list' for supported operations list.")
}

fn calculate(buffer: &str) {

    let calculator = ExprCalculator::<Float>::float_calculator();
    let result = calculator.calculate(&buffer);

    if result.is_err() {
        let err = &result.err().unwrap();
        println!("{}", buffer);
        println!(" {:>1$}", "^", err.get_pos());
        println!("[syntax error] {}", err);
        return;
    }

    println!("{}", result.ok().unwrap().to_string());
}

fn calculate_bool(buffer: &str) {

    let calculator = ExprCalculator::<bool>::boolean_calculator();
    let result = calculator.calculate(&buffer);

    if result.is_err() {
        let err = &result.err().unwrap();
        println!("{}", buffer);
        println!(" {:>1$}", "^", err.get_pos());
        println!("[syntax error] {}", err);
        return;
    }

    println!("{}", result.ok().unwrap().to_string());
}