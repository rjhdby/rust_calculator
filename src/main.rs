use crate::tokens::*;
use crate::tokens::Token::Function;

mod functions;
mod tokens;

fn main() {
    let s = "123*ln(10)/(3-7)+15^2*sin(3)";
    let result = consolidate_all(&tokenize(s));

    print!("{} = {}", result, result.calculate())
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
        if matches!(value, Token::Unknown) {
            continue;
        }

        tokens.push(value);
    }

    return tokens;
}

fn consolidate_all(tokens: &Vec<Token>) -> Token {
    let brackets = consolidate_brackets(tokens);
    let unary = consolidate(&brackets, |it| { matches!(it, Token::Function{value}) });
    let product = consolidate(&unary, consolidate_predicate!(Token::Product));
    let mult_div = consolidate(&product, consolidate_predicate!(Token::Divide, Token::Multiplication));
    return consolidate(&mult_div, consolidate_predicate!(Token::Add, Token::Subtract)).get(0).unwrap().clone();
}

fn consolidate_brackets(tokens: &Vec<Token>) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();
    let mut skip = 0;
    for token in tokens.iter().enumerate() {
        if skip > 0 {
            skip = skip - 1;
            continue;
        }
        let current: &Token = token.1;
        let index = token.0;
        match current {
            Token::OpenBracket => {
                skip = find_close(&tokens[index + 1..].to_vec());
                let inner = consolidate_all(&tokens[index + 1..index + skip].to_vec());
                out.push(inner)
            }
            _ => out.push(current.clone())
        }
    }

    return out;
}

fn find_close(tokens: &Vec<Token>) -> usize {
    let mut depth = 1;
    let mut pos = 0;
    for token in tokens.iter().enumerate() {
        pos = pos + 1;
        match token.1 {
            Token::OpenBracket => depth = depth + 1,
            Token::CloseBracket => depth = depth - 1,
            _ => ()
        }
        if depth == 0 { return pos; }
    }

    panic!("Can't find closed bracket")
}

fn consolidate<F>(tokens: &Vec<Token>, predicate: F) -> Vec<Token> where F: Fn(&Token) -> bool {
    let mut out: Vec<Token> = Vec::new();
    let mut skip = false;
    for token in tokens.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        let current: &Token = token.1;
        match current {
            it if predicate(it) => {
                let next: Token = (tokens.get(token.0 + 1).unwrap()).clone();
                let binary = match &it {
                    Token::Function { .. } => it.make_unary(next),
                    _ => {
                        let previous = out.pop().unwrap();
                        Token::Binary { operation: Box::new(current.clone()), op1: Box::new(previous), op2: Box::new(next) }
                    }
                };

                out.push(binary);
                skip = true
            }
            _ => out.push(current.clone()),
        }
    }

    return out;
}

fn next_token(input: &str, start: usize) -> (Option<Token>, usize) {
    let mut result = String::new();
    let slice: &str = &input[start..];
    for (index, char) in slice.chars().enumerate() {
        if char.is_alphanumeric() || char == POINT {
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