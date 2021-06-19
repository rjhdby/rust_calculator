use std::fmt;

use crate::operation::Operation;
use crate::token::Token;
use rug::Float;

#[derive(Clone)]
pub enum AstNode {
    Number { val: Float },
    Unary { op: Operation, p1: Box<AstNode> },
    Binary { op: Operation, p1: Box<AstNode>, p2: Box<AstNode> },
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl AstNode {
    pub fn to_string(&self) -> String {
        return match self {
            AstNode::Number { val } => val.to_string(),
            AstNode::Unary { op, p1 } => op.pretty(p1.to_string(), None),
            AstNode::Binary { op, p1, p2 } => op.pretty(p1.to_string(), Some(p2.to_string())),
        };
    }

    pub fn calculate(&self) -> Result<Float, String> {
        return match self {
            AstNode::Number { val } => Result::Ok(val.clone()),
            AstNode::Unary { op, p1 } => op.calculate(Some(p1.calculate()?), None),
            AstNode::Binary { op, p1, p2 } => op.calculate(Some(p1.calculate()?), Some(p2.calculate()?)),
        };
    }
}

pub fn build_ast(tokens: &Vec<Token>) -> Result<AstNode, String> {
    let mut stack: Vec<Operation> = Vec::new();
    let mut operands: Vec<AstNode> = Vec::new();

    for token in tokens {
        match token {
            Token::Number { pos: _, val } => operands.push(AstNode::Number { val: val.clone() }),
            Token::VirtualZero { .. } => operands.push(AstNode::Number { val: Float::with_val(64, 0) }),
            Token::Open { .. } => stack.push(Operation::Tombstone),
            Token::Close { .. } => {
                while !matches!(stack.last().unwrap(), Operation::Tombstone) {
                    make_node(&mut operands, stack.pop().unwrap());
                };
                stack.pop();
            }
            Token::Operation { pos: _, val } => {
                while !stack.is_empty() && stack.last().unwrap().priority() >= val.priority() {
                    make_node(&mut operands, stack.pop().unwrap());
                }
                stack.push(val.clone())
            }
            _ => (),
        }
    }

    while stack.last().is_some() {
        make_node(&mut operands, stack.pop().unwrap());
    };

    return Result::Ok(operands.pop().unwrap());
}

fn make_node(operands: &mut Vec<AstNode>, op: Operation) {
    if op.operands() == 0 {
        operands.push(AstNode::Number { val: op.calculate(None, None).unwrap()})
    } else if op.operands() == 1 {
        let op_right = operands.pop().unwrap();
        operands.push(AstNode::Unary { op, p1: Box::new(op_right) })
    } else {
        let op_right = operands.pop().unwrap();
        let op_left = operands.pop();
        operands.push(AstNode::Binary { op, p1: Box::new(op_left.unwrap()), p2: Box::new(op_right) })
    }
}
