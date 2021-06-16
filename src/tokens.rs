use std::fmt;

use crate::functions::Function;

#[derive(Clone)]
pub enum Token {
    Number { value: f64 },
    Function { value: String },
    Add,
    Subtract,
    Multiplication,
    Divide,
    Product,
    OpenBracket,
    CloseBracket,
    Unary { operation: Function, op1: Box<Token> },
    Binary { operation: Box<Token>, op1: Box<Token>, op2: Box<Token> },
    Space,
    Unknown,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Token {
    fn to_string(&self) -> String {
        return match self {
            Token::Add => PLUS.to_string(),
            Token::Number { value } => value.to_string(),
            Token::Function { value } => value.clone(),
            Token::Subtract => MINUS.to_string(),
            Token::Multiplication => MULTIPLICATION.to_string(),
            Token::Divide => DIVIDE.to_string(),
            Token::Product => PRODUCT.to_string(),
            Token::Unary { operation, op1 } => format!("{}({})", operation, &*op1),
            Token::OpenBracket => OPEN.to_string(),
            Token::CloseBracket => CLOSE.to_string(),
            Token::Binary { operation, op1, op2 } => {
                format!("({}{}{})", &*op1, &*operation, &*op2)
            }
            Token::Unknown => EMPTY.to_string(),
            Token::Space => SPACE.to_string()
        };
    }

    pub fn from_string(string: &str) -> Token {
        match string {
            c if is_exp_float(c) => Token::Number { value: c.to_string().parse::<f64>().unwrap() },
            c if c.chars().all(|it| { it.is_alphanumeric() }) => Token::Function { value: c.to_string() },
            PLUS => Token::Add,
            MINUS => Token::Subtract,
            MULTIPLICATION => Token::Multiplication,
            DIVIDE => Token::Divide,
            PRODUCT => Token::Product,
            OPEN => Token::OpenBracket,
            CLOSE => Token::CloseBracket,
            _ => Token::Unknown
        }
    }

    pub fn make_unary(&self, op1: &Token) -> Token {
        match self {
            Token::Function { value } => Token::Unary { operation: Function::from_string(value), op1: Box::new(op1.to_owned()) },
            _ => panic!("Unary operation can only be made from Function.")
        }
    }

    pub fn calculate(&self) -> f64 {
        return match self {
            Token::Number { value } => value.clone(),
            Token::Binary { operation, op1, op2 } => {
                match *operation.clone() {
                    Token::Add => op1.calculate() + op2.calculate(),
                    Token::Subtract => op1.calculate() - op2.calculate(),
                    Token::Multiplication => op1.calculate() * op2.calculate(),
                    Token::Divide => op1.calculate() / op2.calculate(),
                    Token::Product => op1.calculate().powf(op2.calculate()),
                    _ => panic!("Wrong operation"),
                }
            }
            Token::Unary { operation, op1 } => operation.calculate(op1.calculate()),
            _ => panic!("I can't calculate this")
        };
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut index = 0;

    while let token = next_token(input, index) {
        if token.0.is_none() {
            break;
        }
        let value = token.0.unwrap();
        index = token.1;
        if matches!(value, Token::Unknown) {
            continue;
        }

        tokens.push(value);
    }
    return tokens;
}

fn next_token(input: &str, start: usize) -> (Option<Token>, usize) {
    let mut exp = false;
    let mut result = String::new();
    let slice: &str = &input[start..];
    for (index, char) in slice.chars().enumerate() {
        if char == SPACE && result.is_empty() {
            return (Some(Token::Space), start + index + 1);
        }
        if char.is_alphanumeric() || char == POINT {
            if (char == 'e' || char == 'E') && result.chars().all(|it| { it.is_numeric() || it == POINT }) {
                exp = true
            }
            if exp && result.chars().any(|it| { it.is_alphabetic() }) {
                exp = false
            }
            result.push(char);
            continue;
        }
        if char == '-' && exp {
            result.push(char);
            continue;
        }
        if !result.is_empty() {
            return (Some(Token::from_string(&result[..])), start + index);
        }

        return (Some(Token::from_string(&char.to_string())), start + index + 1);
    }

    if !result.is_empty() {
        return (Some(Token::from_string(&result[..])), input.len());
    }

    return (None, 0);
}

pub fn cleanup(input: &Vec<Token>) -> Vec<Token> {
    return input.into_iter()
        .filter(|it| { !matches!(it, Token::Space) })
        .map(|it| {it.clone()})
        .collect::<Vec<_>>();
}

pub fn validate(input: &Vec<Token>) -> bool {
    return true
}

const PLUS: &str = "+";
const MINUS: &str = "-";
const MULTIPLICATION: &str = "*";
const DIVIDE: &str = "/";
const PRODUCT: &str = "^";
const EMPTY: &str = "";
const OPEN: &str = "(";
const CLOSE: &str = ")";

pub const POINT: char = '.';
pub const SPACE: char = ' ';

fn is_exp_float(value: &str) -> bool {
    return value.to_string().parse::<f64>().is_ok();
}