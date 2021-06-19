use std::fmt;

use crate::context::{Context, State};
use crate::operation::Operation;
use rug::Float;

#[derive(Clone)]
pub enum Token {
    WhiteSpace { pos: usize, val: String },
    Open { pos: usize },
    Close { pos: usize },
    Number { pos: usize, val: Float },
    VirtualZero { pos: usize },
    Operation { pos: usize, val: Operation },
    Unknown { pos: usize, val: String },
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Token::WhiteSpace { pos, val } => format!("'{}' at position {}", val, pos),
            Token::Open { pos } => format!("'(' at position {}", pos),
            Token::Close { pos } => format!("')' at position {}", pos),
            Token::Number { pos, val } => format!("'{}' at position {}", val, pos),
            Token::VirtualZero { pos } => format!("Virtual zero at position {}", pos),
            Token::Operation { pos, val } => format!("'{}' at position {}", val.to_string(), pos),
            Token::Unknown { pos, val } => format!("'{}' at position {}", val, pos),
        };
        write!(f, "{}", str)
    }
}

impl Token {
    pub fn get_pos(&self) -> usize {
        return match self {
            Token::WhiteSpace { pos, val: _val } => *pos,
            Token::Open { pos } => *pos,
            Token::Close { pos } => *pos,
            Token::Number { pos, val: _val } => *pos,
            Token::VirtualZero { pos } => *pos,
            Token::Operation { pos, val: _val } => *pos,
            Token::Unknown { pos, val: _val } => *pos,
        };
    }

    pub fn is_unary(&self) -> bool {
        return match self {
            Token::Operation { pos: _pos, val } => if val.operands() == 1 { true } else { false }
            _ => false
        };
    }

    pub fn is_binary(&self) -> bool {
        return match self {
            Token::Operation { pos: _pos, val } => if val.operands() == 2 { true } else { false }
            _ => false
        };
    }

    pub fn is_constant(&self) -> bool {
        return match self {
            Token::Operation { pos: _pos, val } => if val.operands() == 0 { true } else { false }
            _ => false
        };
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Token> {
    let mut context = Context::new();

    for entry in input.chars().into_iter().enumerate() {
        let pos = entry.0;
        let val = entry.1;

        match context.state() {
            State::Empty => match val {
                ' ' => context.init_whitespace(pos),
                c if c.is_alphanumeric() || c == '.' => context.init_alfanum(c, pos),
                it if context.is_suitable_for_negation(it) => context.add_negation_token(pos)?,
                _ => context.add_symbolic_token(val, pos)?,
            }
            State::AlphaNumeric => match val {
                ' ' => {
                    context.collect_token()?;
                    context.init_whitespace(pos);
                }
                c if c.is_alphanumeric() || c == '.' => context.add_symbol(val),
                it if (it == '-' || it == '+') && context.is_partial_exp() => context.add_symbol(it),
                _ => {
                    context.collect_token()?;
                    context.add_symbolic_token(val, pos)?
                }
            }
            State::WhiteSpace => match val {
                ' ' => context.add_symbol(' '),
                c if c.is_alphanumeric() => {
                    context.collect_whitespace()?;
                    context.init_alfanum(val, pos);
                }
                _ => {
                    context.collect_whitespace()?;
                    context.add_symbolic_token(val, pos)?
                }
            }
        }
    }

    if !context.is_empty() {
        context.collect_token()?;
    }

    return Result::Ok(context.get_tokens().to_owned());
}
