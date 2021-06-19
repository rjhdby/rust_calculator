use std::f64::consts::{E, PI};

use strum_macros::EnumIter;

#[derive(Clone, EnumIter)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Product,

    Sqrt,
    Sin,
    Cos,
    Ln,
    Log2,
    Log10,

    Pi,
    E,

    Tombstone,
}

impl Operation {
    pub fn to_string(&self) -> String {
        return match self {
            Operation::Addition => "+",
            Operation::Subtraction => "-",
            Operation::Multiplication => "*",
            Operation::Division => "/",
            Operation::Product => "^",
            Operation::Sqrt => "sqrt",
            Operation::Sin => "sin",
            Operation::Cos => "cos",
            Operation::Ln => "ln",
            Operation::Log2 => "log2",
            Operation::Log10 => "log10",
            Operation::Tombstone => "",
            Operation::Pi => "pi",
            Operation::E => "e",
        }.to_string();
    }

    pub fn pretty(&self, op1: String, op2: Option<String>) -> String {
        return match self {
            it if it.operands() == 2 => format!("({}{}{})", op1, self.to_string(), op2.unwrap()),
            it if it.operands() == 1 => format!("{}({})", self.to_string(), op1),
            _ => self.to_string(),
        };
    }

    pub fn priority(&self) -> i8 {
        return match self {
            Operation::Addition => 1,
            Operation::Subtraction => 1,
            Operation::Multiplication => 2,
            Operation::Division => 2,
            Operation::Product => 3,
            Operation::Sqrt => 4,
            Operation::Sin => 4,
            Operation::Cos => 4,
            Operation::Ln => 4,
            Operation::Log2 => 4,
            Operation::Log10 => 4,
            Operation::Tombstone => 0,
            Operation::Pi => 5,
            Operation::E => 5,
        };
    }

    pub fn from_string(val: &str) -> Result<Operation, ()> {
        let local = val.clone().to_ascii_lowercase();
        let result = match local.as_str() {
            "+" => Operation::Addition,
            "-" => Operation::Subtraction,
            "*" => Operation::Multiplication,
            "/" => Operation::Division,
            "^" => Operation::Product,
            "sqrt" => Operation::Sqrt,
            "sin" => Operation::Sin,
            "cos" => Operation::Cos,
            "ln" => Operation::Ln,
            "log2" => Operation::Log2,
            "log10" => Operation::Log10,
            "pi" => Operation::Pi,
            "e" => Operation::E,
            _ => return Result::Err(())
        };

        return Result::Ok(result);
    }

    pub fn operands(&self) -> i8 {
        return match self {
            Operation::Addition | Operation::Subtraction | Operation::Multiplication | Operation::Division | Operation::Product => 2,
            Operation::Pi | Operation::E | Operation::Tombstone => 0,
            _ => 1
        };
    }

    pub fn calculate(&self, op1: Option<f64>, op2: Option<f64>) -> Result<f64, String> {
        let result = match self {
            Operation::Addition => op1.unwrap() + op2.unwrap(),
            Operation::Subtraction => op1.unwrap() - op2.unwrap(),
            Operation::Multiplication => op1.unwrap() * op2.unwrap(),
            Operation::Division => op1.unwrap() / op2.unwrap(),
            Operation::Product => op1.unwrap().powf(op2.unwrap()),
            Operation::Sqrt => op1.unwrap().sqrt(),
            Operation::Sin => op1.unwrap().sin(),
            Operation::Cos => op1.unwrap().cos(),
            Operation::Ln => op1.unwrap().ln(),
            Operation::Log2 => op1.unwrap().log2(),
            Operation::Log10 => op1.unwrap().log10(),
            Operation::Pi => PI,
            Operation::E => E,
            _ => return Result::Err(format!("Incalculable operation: {}", self.to_string()))
        };

        return Result::Ok(result);
    }
}