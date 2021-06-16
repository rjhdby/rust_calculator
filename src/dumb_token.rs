use std::any::Any;

use regex::Regex;
use crate::operation::Operation;

#[derive(Clone)]
pub enum DumbToken {
    WhiteSpace { pos: usize, val: String },
    Open { pos: usize },
    Close { pos: usize },
    Number { pos: usize, val: f64 },
    Operation { pos: usize, val: Operation },
}

impl DumbToken {
    pub fn to_string(&self) -> String {
        return match self {
            DumbToken::Open { .. } => String::from('('),
            DumbToken::Close { .. } => String::from(')'),
            DumbToken::WhiteSpace { pos: _pos, val } => val.clone(),
            DumbToken::Number { pos, val } => val.to_string(),
            DumbToken::Operation { pos, val } => val.to_string(),
        };
    }
}

enum State {
    Empty,
    AlphaNumeric,
    WhiteSpace,
}

pub fn dumb_tokenize(input: &str) -> Result<Vec<DumbToken>, String> {
    let mut out: Vec<DumbToken> = Vec::new();
    let mut state: State = State::Empty;
    let mut current = String::new();
    let mut current_start = 0;

    for entry in input.chars().into_iter().enumerate() {
        let pos = entry.0;
        let val = entry.1;

        match state {
            State::Empty => match val {
                ' ' => {
                    current_start = pos;
                    current = String::from(' ');
                    state = State::WhiteSpace;
                }
                c if c.is_alphanumeric() || c == '.' => {
                    current_start = pos;
                    current = String::from(c);
                    state = State::AlphaNumeric;
                }
                it if it == '-' && (out.is_empty() || matches!(out.last().unwrap(), DumbToken::Open{..})) => {
                    out.push(DumbToken::Operation { pos, val: Operation::Negation })
                }
                _ => {
                    let token = make_symbol_token(val, pos);
                    if token.is_err() { return Result::Err(token.err().unwrap()); }
                    out.push(token.unwrap())
                }
            }
            State::AlphaNumeric => match val {
                ' ' => {
                    let token = make_dumb_token(&state, &current, current_start);
                    if token.is_err() { return Result::Err(token.err().unwrap()); }
                    out.push(token.unwrap());
                    current_start = pos;
                    current = String::from(val);
                    state = State::WhiteSpace;
                }
                c if c.is_alphanumeric() || c == '.' => {
                    current.push(val)
                }
                it if (it == '-' || it == '+') && PARTIAL_EXP.is_match(&current) => {
                    current.push(it)
                }
                _ => {
                    let token = make_dumb_token(&state, &current, current_start);
                    if token.is_err() { return Result::Err(token.err().unwrap()); }
                    out.push(token.unwrap());
                    let token = make_symbol_token(val, pos);
                    if token.is_err() { return Result::Err(token.err().unwrap()); }
                    out.push(token.unwrap());
                    state = State::Empty
                }
            }
            State::WhiteSpace => match val {
                ' ' => {
                    current.push(' ')
                }
                c if c.is_alphanumeric() => {
                    out.push(DumbToken::WhiteSpace {pos:current_start, val: current.clone()});
                    current_start = pos;
                    current = String::from(val);
                    state = State::AlphaNumeric;
                }
                _ => {
                    out.push(DumbToken::WhiteSpace {pos:current_start, val: current.clone()});
                    let token = make_symbol_token(val, pos);
                    if token.is_err() { return Result::Err(token.err().unwrap()); }
                    out.push(token.unwrap());
                    state = State::Empty
                }
            }
        }
    }

    if !current.is_empty() {
        let token = make_dumb_token(&state, &current, current_start);
        if token.is_err() { return Result::Err(token.err().unwrap()); }
        out.push(token.unwrap());
    }

    return Result::Ok(out);
}

fn make_dumb_token(state: &State, val: &String, pos: usize) -> Result<DumbToken, String> {
    let result = match state {
        State::AlphaNumeric => match val {
            it if NUMBER.is_match(it) || EXP.is_match(it) => {
                DumbToken::Number { pos, val: it.parse::<f64>().unwrap() }
            }
            _ => {
                let operation = Operation::from_string(val);
                if operation.is_err() { return Result::Err(operation.err().unwrap()); }
                DumbToken::Operation { pos, val: operation.unwrap() }
            }
        },
        State::WhiteSpace => DumbToken::WhiteSpace { pos, val: val.clone() },
        _ => return Result::Err(String::from("Impossible state"))
    };

    return Result::Ok(result);
}

fn make_symbol_token(val: char, pos: usize) -> Result<DumbToken, String> {
    let result = match val {
        '(' => DumbToken::Open { pos },
        ')' => DumbToken::Close { pos },
        _ => {
            let operation = Operation::from_string(&val.to_string());
            if operation.is_err() { return Result::Err(operation.err().unwrap()); }
            DumbToken::Operation { pos, val: operation.unwrap() }
        }
    };
    return Result::Ok(result);
}

lazy_static! {
    static ref PARTIAL_EXP: Regex = Regex::new(r"^(\d+|\d+\.\d+)[eE]$").unwrap();
    static ref EXP: Regex = Regex::new(r"^(\d+|\d+\.\d+)[eE][-+]?\d+$").unwrap();
    static ref NUMBER: Regex = Regex::new(r"^\d+\.?\d*$").unwrap();
}