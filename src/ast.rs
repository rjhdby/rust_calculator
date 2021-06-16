use std::fmt;

use regex::Regex;

use crate::dumb_token::DumbToken;
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
}

pub fn build_ast(mut raw: &Vec<DumbToken>) -> Result<AstNode, String> {
    let mut operators: Vec<Operation> = Vec::new();
    let mut operands: Vec<AstNode> = Vec::new();

    for token in raw {
        match token {
            DumbToken::WhiteSpace { .. } => (),
            DumbToken::Open { .. } => operators.push(Operation::Open),
            DumbToken::Close { .. } => {
                while let opr = operators.pop() {
                    if opr.is_none() {
                        println!("NONE: {}", operands.last().unwrap().to_string())
                    }
                    let op= opr.unwrap();
                    if matches!(op, Operation::Open) {
                        break;
                    }
                    let op1 = operands.pop().unwrap();
                    if op.operands() == 1 {
                        operands.push(AstNode::Unary { op, p1: Box::new(op1) })
                    } else {
                        let op2 = operands.pop().unwrap();
                        operands.push(AstNode::Binary { op, p1: Box::new(op1), p2: Box::new(op2) })
                    }
                };
            }
            DumbToken::Number { pos, val } => operands.push(AstNode::Number { val: *val }),
            DumbToken::Operation { pos, val } => {
                if operators.is_empty() || operators.last().unwrap().priority() < val.priority() {
                    operators.push(val.clone())
                } else {
                    let operation = operators.pop().unwrap();
                    let op1 = operands.pop().unwrap();
                    if operation.operands() == 1 {
                        operands.push(AstNode::Unary { op: operation, p1: Box::new(op1) })
                    } else {
                        let op2 = operands.pop().unwrap();
                        operands.push(AstNode::Binary { op: operation, p1: Box::new(op1), p2: Box::new(op2) })
                    }
                }
            }
        }

    }
    for l in &operands {
        println!("{}", l)
    }
    while let operation = operators.pop() {
        if operation.is_none() {
            break
        }
        let op = operation.unwrap();
        if matches!(op, Operation::Open) {
            break;
        }
        let op1 = operands.pop();
        if op1.is_none() {
            print!("err3: {} ", op.to_string());
            break
        }
        if op.operands() == 1 {
            operands.push(AstNode::Unary { op: op, p1: Box::new(op1.unwrap()) })
        } else {
            let op2 = operands.pop();
            if op2.is_none() {
                print!("err4: {} ", op.to_string());
                break
            }
            operands.push(AstNode::Binary { op: op, p1: Box::new(op1.unwrap()), p2: Box::new(op2.unwrap()) })
        }
    };
    return Result::Ok(operands.pop().unwrap());
}
