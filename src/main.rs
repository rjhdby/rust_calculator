use crate::tokens::*;

mod functions;
mod tokens;

fn main() {
    let s = "123*ln(10)/(3-7)+15^2*sin(3) + 2e3 - 2E-3 + e-2";
    // let s = "e-2";
    let result = consolidate_all(&tokenize(s));

    print!("{} = {}", result, result.calculate())
}

fn consolidate_all(tokens: &Vec<Token>) -> Token {
    let brackets = consolidate_brackets(tokens);
    let unary = consolidate(&brackets, |it| { matches!(it, Token::Function{..}) });
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

        if predicate(current) {
            let next: &Token = tokens.get(token.0 + 1).unwrap();
            let operation = match &current {
                Token::Function { .. } => current.make_unary(next),
                _ => {
                    let previous = out.pop().unwrap();
                    Token::Binary { operation: Box::new(current.clone()), op1: Box::new(previous.clone()), op2: Box::new(next.clone()) }
                }
            };

            out.push(operation);
            skip = true
        } else {
            out.push(current.clone())
        }
    }

    return out;
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