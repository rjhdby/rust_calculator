use std::fmt;

use crate::token::Token;
use crate::operation::Operation;

#[derive(Clone)]
pub enum AstNode {
    Number { val: f64 },
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
            AstNode::Unary { op, p1 } => {
                op.pretty(p1.to_string(), None)
            }
            AstNode::Binary { op, p1, p2 } => {
                op.pretty(p1.to_string(), Some(p2.to_string()))
            }
        };
    }

    pub fn calculate(&self) -> f64 {
        return match self {
            AstNode::Number { val } => *val,
            AstNode::Unary { op, p1 } => op.calculate(p1.calculate(), None),
            AstNode::Binary { op, p1, p2 } => op.calculate(p1.calculate(), Some(p2.calculate())),
        };
    }
}

pub fn build_ast(raw: &Vec<Token>) -> Result<AstNode, String> {
    let mut stack: Vec<Operation> = Vec::new();
    let mut operands: Vec<AstNode> = Vec::new();

    for token in raw {
        match token {
            Token::WhiteSpace { .. } => (),
            Token::Open { .. } => stack.push(Operation::Open),
            Token::Close { .. } => {
                while let op = stack.pop().unwrap() {
                    if matches!(op, Operation::Open) {
                        break;
                    }
                    let op_right = operands.pop().unwrap();
                    if op.operands() == 1 {
                        operands.push(AstNode::Unary { op, p1: Box::new(op_right) })
                    } else {
                        let op_left = operands.pop().unwrap();
                        operands.push(AstNode::Binary { op, p1: Box::new(op_left), p2: Box::new(op_right) })
                    }
                };
            }
            Token::Number { pos, val } => operands.push(AstNode::Number { val: *val }),
            Token::Operation { pos, val } => {
                while !stack.is_empty() && stack.last().unwrap().priority() >= val.priority() {
                    let operation = stack.pop().unwrap();
                    let op_right = operands.pop().unwrap();
                    if operation.operands() == 1 {
                        operands.push(AstNode::Unary { op: operation, p1: Box::new(op_right) })
                    } else {
                        let op_left = operands.pop().unwrap();
                        operands.push(AstNode::Binary { op: operation, p1: Box::new(op_left), p2: Box::new(op_right) })
                    }
                }
                stack.push(val.clone())
            }
        }
    }

    while let operation = stack.pop() {
        if operation.is_none() {
            break;
        }
        let op = operation.unwrap();
        let op_right = operands.pop();
        if op.operands() == 1 {
            operands.push(AstNode::Unary { op, p1: Box::new(op_right.unwrap()) })
        } else {
            let op_left = operands.pop();
            operands.push(AstNode::Binary { op, p1: Box::new(op_left.unwrap()), p2: Box::new(op_right.unwrap()) })
        }
    };

    return Result::Ok(operands.pop().unwrap());
}

