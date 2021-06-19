use regex::Regex;
use rug::Float;

use crate::operation::Operation;
use crate::token::Token;
use crate::validator::validate;

pub enum State {
    Empty,
    AlphaNumeric,
    WhiteSpace,
}

pub struct Context {
    out: Vec<Token>,
    state: State,
    value: String,
    pos: usize,
}

impl Context {
    pub fn new() -> Context {
        return Context { out: Vec::new(), state: State::Empty, value: String::new(), pos: 0 };
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        return &self.out;
    }

    pub fn collect_token(&mut self) -> Result<(), Token> {
        let token = match self.state {
            State::AlphaNumeric => if self.is_number() {
                self.to_numeric_token()
            } else {
                let operation = Operation::from_string(&self.value);
                if operation.is_err() {
                    return Result::Err(Token::Unknown { pos: self.pos, val: self.value.clone() });
                }
                Token::Operation { pos: self.pos, val: operation.unwrap() }
            },
            State::WhiteSpace => self.to_whitespace_token(),
            State::Empty => return Result::Ok(())
        };

        self.add_token(token)?;

        Result::Ok(())
    }

    pub fn add_negation_token(&mut self, pos: usize) -> Result<(), Token> {
        self.out.push(Token::VirtualZero { pos });
        self.out.push(Token::Operation { pos, val: Operation::Subtraction });

        Result::Ok(())
    }

    fn add_token(&mut self, token: Token) -> Result<(), Token> {
        if !validate(&self.take_last(), &token) {
            return Result::Err(token);
        }
        self.out.push(token);

        return Result::Ok(());
    }

    pub fn add_symbolic_token(&mut self, val: char, pos: usize) -> Result<(), Token> {
        let token = match val {
            '(' => Token::Open { pos },
            ')' => Token::Close { pos },
            _ => {
                let operation = Operation::from_string(&val.to_string());
                if operation.is_err() {
                    return Result::Err(Token::Unknown { pos: self.pos, val: self.value.clone() });
                }
                Token::Operation { pos, val: operation.unwrap() }
            }
        };
        self.add_token(token)?;
        self.state = State::Empty;

        return Result::Ok(());
    }

    fn take_last(&self) -> Token {
        if self.out.is_empty() {
            return Token::WhiteSpace { pos: 0, val: "".to_string() };
        }
        for x in self.out.clone().iter().rev() {
            if !matches!(x, Token::WhiteSpace {..}) {
                return x.clone();
            }
        }

        return Token::WhiteSpace { pos: 0, val: "".to_string() };
    }

    pub fn collect_whitespace(&mut self) -> Result<(), Token> {
        self.add_token(self.to_whitespace_token())?;

        return Result::Ok(());
    }

    pub fn state(&self) -> &State {
        return &self.state;
    }

    pub fn init_whitespace(&mut self, pos: usize) {
        self.state = State::WhiteSpace;
        self.value = String::from(' ');
        self.pos = pos;
    }

    pub fn init_alfanum(&mut self, val: char, pos: usize) {
        self.state = State::AlphaNumeric;
        self.value = String::from(val);
        self.pos = pos;
    }

    pub fn add_symbol(&mut self, symbol: char) {
        self.value.push(symbol)
    }

    pub fn is_partial_exp(&self) -> bool {
        return PARTIAL_EXP.is_match(&self.value);
    }

    fn to_whitespace_token(&self) -> Token {
        return Token::WhiteSpace { pos: self.pos, val: self.value.clone() };
    }

    fn to_numeric_token(&self) -> Token {
        let valid = Float::parse(&self.value);

        return Token::Number { pos: self.pos, val: Float::with_val(64, valid.unwrap()) };
    }

    pub fn is_empty(&self) -> bool {
        return self.value.is_empty();
    }

    fn is_number(&self) -> bool {
        return if NUMBER.is_match(&self.value) || EXP.is_match(&self.value) {
            true
        } else {
            false
        };
    }

    pub fn is_suitable_for_negation(&self, char: char) -> bool {
        if char != '-' {
            return false;
        }

        if self.out.is_empty() {
            return true;
        }

        return match self.out.last().unwrap() {
            Token::Open { .. } => true,
            _ => false
        };
    }
}

lazy_static! {
    static ref PARTIAL_EXP: Regex = Regex::new(r"^-?(\d+|\d+\.\d+)[eE]$").unwrap();
    static ref NUMBER: Regex = Regex::new(r"^-?\d+\.?\d*$").unwrap();
    static ref EXP: Regex = Regex::new(r"^-?(\d+|\d+\.\d+)[eE][-+]?\d+$").unwrap();
}
