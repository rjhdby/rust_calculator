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
    Unary { operation: String, op1: Box<Token> },
    Binary { operation: Box<Token>, op1: Box<Token>, op2: Box<Token> },
    Unknown,
}

impl Token {
    pub fn to_string(&self) -> String {
        let result = match self {
            Token::Number { value } => value.to_string(),
            Token::Function { value } => value.clone(),
            Token::Add => PLUS.to_string(),
            Token::Subtract => MINUS.to_string(),
            Token::Multiplication => MULTIPLICATION.to_string(),
            Token::Divide => DIVIDE.to_string(),
            Token::Product => PRODUCT.to_string(),
            Token::Unary { operation, op1 } => format!("{}({})", operation, &*op1.to_string()),
            Token::OpenBracket => OPEN.to_string(),
            Token::CloseBracket => CLOSE.to_string(),
            Token::Binary { operation, op1, op2 } => {
                format!("({}{}{})", &*op1.to_string(), &*operation.to_string(), &*op2.to_string())
            }
            Token::Unknown => EMPTY.to_string(),
        };

        return result;
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
}

impl Token {
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
            Token::Unary { operation, op1 } => op1.calculate().sin(),
            _ => panic!("I can calculate only COMPLEX")
        };
    }
}

pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const MULTIPLICATION: &str = "*";
pub const DIVIDE: &str = "/";
pub const PRODUCT: &str = "^";
pub const EMPTY: &str = "";
pub const OPEN: &str = "(";
pub const CLOSE: &str = ")";

pub const POINT: char = '.';