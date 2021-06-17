#[derive(Clone)]
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

    Negation,

    Open,
}


impl Operation {
    pub fn to_string(&self) -> String {
        return match self {
            Operation::Addition => String::from("+"),
            Operation::Subtraction => String::from("-"),
            Operation::Multiplication => String::from("*"),
            Operation::Division => String::from("/"),
            Operation::Product => String::from("^"),
            Operation::Sqrt => String::from("sqrt"),
            Operation::Sin => String::from("sin"),
            Operation::Cos => String::from("cos"),
            Operation::Ln => String::from("ln"),
            Operation::Log2 => String::from("log2"),
            Operation::Log10 => String::from("log10"),
            Operation::Negation => String::from("~"),
            Operation::Open => String::from("("),
        };
    }

    pub fn pretty(&self, op1: String, op2: Option<String>) -> String {
        return match self {
            Operation::Addition | Operation::Subtraction | Operation::Multiplication | Operation::Division | Operation::Product => format!("({}{}{})", op1, self.to_string(), op2.unwrap()),
            Operation::Negation => format!("(-{})", op1),
            Operation::Open => self.to_string(),
            _ => format!("{}({})", self.to_string(), op1),
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
            Operation::Negation => 4,
            Operation::Open => 0,
        };
    }

    pub fn from_string(val: &str) -> Result<Operation, String> {
        let result = match val {
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
            _ => return Result::Err(String::from(format!("Unknown operation: {}", val)))
        };

        return Result::Ok(result);
    }

    pub fn operands(&self) -> i8 {
        return match self {
            Operation::Addition | Operation::Subtraction | Operation::Multiplication | Operation::Division | Operation::Product => 2,
            _ => 1
        };
    }

    pub fn calculate(&self, op1: f64, op2: Option<f64>) -> f64 {
        return match self {
            Operation::Addition => op1 + op2.unwrap(),
            Operation::Subtraction => op1 - op2.unwrap(),
            Operation::Multiplication => op1 * op2.unwrap(),
            Operation::Division => op1 / op2.unwrap(),
            Operation::Product => op1.powf(op2.unwrap()),
            Operation::Sqrt => op1.sqrt(),
            Operation::Sin => op1.sin(),
            Operation::Cos => op1.cos(),
            Operation::Ln => op1.ln(),
            Operation::Log2 => op1.log2(),
            Operation::Log10 => op1.log10(),
            Operation::Negation => -op1,
            _ => panic!("Incalculable operation")
        };
    }
}