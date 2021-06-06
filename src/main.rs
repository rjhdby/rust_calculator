use std::sync::mpsc::channel;

fn main() {
    let s = "(1.1 + 2*(20 -10))*2+(1+2)/3";

    let tokens = tokenize(s);
    let result = consolidate_all(tokens);

    print!("{} = {}", result.to_string(), result.calculate())
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut index = 0;

    while let token = next_token(input, index) {
        if token.0.is_none() {
            break;
        }
        let value = token.0.unwrap();
        index = token.1;
        if matches!(value, Token::UNKNOWN) {
            continue;
        }

        tokens.push(value);
    }

    return tokens;
}

fn consolidate_all(tokens: Vec<Token>) -> Token {
    let brackets = consolidate_brackets(tokens);
    let product = consolidate(brackets, consolidate_predicate!(Token::PRODUCT));
    let mult_div = consolidate(product, consolidate_predicate!(Token::DIVIDE, Token::MULTIPLICATION));
    return consolidate(mult_div, consolidate_predicate!(Token::PLUS, Token::MINUS)).get(0).unwrap().clone();
}

fn consolidate_brackets(tokens: Vec<Token>) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();
    let mut skip = 0;
    for token in tokens.iter().enumerate() {
        if skip > 0 {
            skip = skip - 1;
            continue;
        }
        let current: Token = (*token.1).clone();
        let index = token.0.clone();
        match (*token.1).clone() {
            Token::OPEN => {
                skip = find_close(tokens[index + 1..].to_vec());
                let inner = consolidate_all(tokens[index + 1..index + skip].to_vec());
                out.push(inner)
            }
            _ => out.push(current)
        }
    }

    return out;
}

fn find_close(tokens: Vec<Token>) -> usize {
    let mut depth = 1;
    let mut pos = 0;
    for token in tokens.iter().enumerate() {
        pos = pos + 1;
        match (*token.1).clone() {
            Token::OPEN => depth = depth + 1,
            Token::CLOSE => depth = depth - 1,
            _ => ()
        }
        if depth == 0 { return pos; }
    }

    panic!("Can't find closed bracket")
}

fn consolidate<F>(tokens: Vec<Token>, predicate: F) -> Vec<Token> where F: Fn(Token) -> bool {
    let mut out: Vec<Token> = Vec::new();
    let mut skip = false;
    for token in tokens.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        let current: Token = (*token.1).clone();
        match (*token.1).clone() {
            it if predicate(it.clone()) => {
                let previous = out.pop().unwrap();
                let next: Token = (tokens.get(token.0 + 1).unwrap()).clone();
                let complex = Token::COMPLEX { operation: Box::new(current), op1: Box::new(previous), op2: Box::new(next) };
                out.push(complex);
                skip = true
            }
            _ => out.push(current),
        }
    }

    return out;
}

fn next_token(input: &str, start: usize) -> (Option<Token>, usize) {
    let mut result = String::new();
    let slice: &str = &input[start..];
    for (index, char) in slice.chars().enumerate() {
        if char.is_numeric() || char == POINT {
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

#[derive(Clone)]
enum Token {
    NUMBER { value: f64 },
    PLUS,
    MINUS,
    MULTIPLICATION,
    DIVIDE,
    PRODUCT,
    UNKNOWN,
    COMPLEX { operation: Box<Token>, op1: Box<Token>, op2: Box<Token> },
    OPEN,
    CLOSE,
}

impl Token {
    fn to_string(&self) -> String {
        let result = match self {
            Token::NUMBER { value } => value.to_string(),
            Token::PLUS => PLUS.to_string(),
            Token::MINUS => MINUS.to_string(),
            Token::MULTIPLICATION => MULTIPLICATION.to_string(),
            Token::DIVIDE => DIVIDE.to_string(),
            Token::PRODUCT => PRODUCT.to_string(),
            Token::UNKNOWN => EMPTY.to_string(),
            Token::COMPLEX { operation, op1, op2 } => {
                format!("({}{}{})", &*op1.to_string(), &*operation.to_string(), &*op2.to_string())
            }
            Token::OPEN => OPEN.to_string(),
            Token::CLOSE => CLOSE.to_string(),
        };

        return result;
    }

    fn from_string(string: &str) -> Token {
        match string {
            c if c.chars().all(|it| { it.is_numeric() || it == POINT }) => Token::NUMBER { value: c.to_string().parse::<f64>().unwrap() },
            PLUS => Token::PLUS,
            MINUS => Token::MINUS,
            MULTIPLICATION => Token::MULTIPLICATION,
            DIVIDE => Token::DIVIDE,
            PRODUCT => Token::PRODUCT,
            OPEN => Token::OPEN,
            CLOSE => Token::CLOSE,
            _ => Token::UNKNOWN
        }
    }
}

impl Token {
    fn calculate(&self) -> f64 {
        return match self {
            Token::NUMBER { value } => value.clone(),
            Token::COMPLEX { operation, op1, op2 } => {
                match *operation.clone() {
                    Token::PLUS => op1.calculate() + op2.calculate(),
                    Token::MINUS => op1.calculate() - op2.calculate(),
                    Token::MULTIPLICATION => op1.calculate() * op2.calculate(),
                    Token::DIVIDE => op1.calculate() / op2.calculate(),
                    Token::PRODUCT => op1.calculate().powf(op2.calculate()),
                    _ => panic!("Wrong operation"),
                }
            }
            _ => panic!("I can calculate only COMPLEX")
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
const POINT: char = '.';

#[macro_export]
macro_rules! consolidate_predicate {
    ($token:path) => {
    |it| {
            match it {
                $token => true,
                _ => false
            }
        }
    };
    ($token1:path, $token2:path) => {
    |it| {
            match it {
                $token1 | $token2 => true,
                _ => false
            }
        }
    }
}