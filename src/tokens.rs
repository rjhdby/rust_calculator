use crate::functions::Function;
use crate::tokens::Token::Unary;
use std::fmt;

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
        };
    }

    pub fn from_string(string: &str) -> Token {
        match string {
            c if c.chars().all(|it| { it.is_numeric() || it == POINT }) => Token::Number { value: c.to_string().parse::<f64>().unwrap() },
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

    pub fn make_unary(&self, op1: Token) -> Token {
        match self {
            Token::Function { value } => Token::Unary { operation: Function::from_string(value), op1: Box::new(op1) },
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

const PLUS: &str = "+";
const MINUS: &str = "-";
const MULTIPLICATION: &str = "*";
const DIVIDE: &str = "/";
const PRODUCT: &str = "^";
const EMPTY: &str = "";
const OPEN: &str = "(";
const CLOSE: &str = ")";

pub const POINT: char = '.';