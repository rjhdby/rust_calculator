use rug::Float;
use rug::float::Constant;
use rug::ops::Pow;
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
    Exp,

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
            Operation::Exp => "exp",
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
            Operation::Exp => 4,
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
            "exp" => Operation::Exp,
            _ => return Result::Err(())
        };

        return Result::Ok(result);
    }

    pub fn operands(&self) -> i8 {
        return match self {
            Operation::Addition | Operation::Subtraction | Operation::Multiplication
            | Operation::Division | Operation::Product => 2,

            Operation::Sqrt | Operation::Sin | Operation::Cos
            | Operation::Ln | Operation::Log2 | Operation::Log10 | Operation::Exp => 1,

            Operation::Pi | Operation::E | Operation::Tombstone => 0,
        };
    }

    pub fn calculate(&self, op1: Option<Float>, op2: Option<Float>) -> Result<Float, String> {
        let result = match self {
            Operation::Addition => op1.unwrap() + op2.unwrap(),
            Operation::Subtraction => op1.unwrap() - op2.unwrap(),
            Operation::Multiplication => op1.unwrap() * op2.unwrap(),
            Operation::Division => op1.unwrap() / op2.unwrap(),
            Operation::Product => op1.unwrap().pow(op2.unwrap()),
            Operation::Sqrt => op1.unwrap().sqrt(),
            Operation::Sin => op1.unwrap().sin(),
            Operation::Cos => op1.unwrap().cos(),
            Operation::Ln => op1.unwrap().ln(),
            Operation::Log2 => op1.unwrap().log2(),
            Operation::Log10 => op1.unwrap().log10(),
            Operation::Pi => Float::with_val(64, Constant::Pi),
            Operation::E => Float::with_val(64, 1).exp(),
            Operation::Exp => op1.unwrap().exp(),
            Operation::Tombstone => return Result::Err(format!("Internal AST error")),
        };

        return Result::Ok(result);
    }
}