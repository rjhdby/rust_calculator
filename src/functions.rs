use std::fmt;

#[derive(Clone)]
pub enum Function {
    Sin,
    Cos,
    Sqrt,
    Ln,
    Log2,
    Log10,
    Unknown { name: String },
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Function {
    fn to_string(&self) -> String {
        match self {
            Function::Sin => "sin",
            Function::Cos => "cos",
            Function::Sqrt => "sqrt",
            Function::Ln => "ln",
            Function::Log2 => "log2",
            Function::Log10 => "log10",
            Function::Unknown { name } => &name,
        }.to_string()
    }

    pub fn from_string(value: &str) -> Function {
        match value {
            "sin" => Function::Sin,
            "cos" => Function::Cos,
            "sqrt" => Function::Sqrt,
            "ln" => Function::Ln,
            "log2" => Function::Log2,
            "log10" => Function::Log10,
            _ => Function::Unknown { name: value.to_string() }
        }
    }

    pub fn calculate(&self, op1: f64) -> f64 {
        match self {
            Function::Sin => op1.sin(),
            Function::Cos => op1.cos(),
            Function::Sqrt => op1.sqrt(),
            Function::Ln => op1.ln(),
            Function::Log2 => op1.log2(),
            Function::Log10 => op1.log10(),
            Function::Unknown { name } => panic!(format!("Unknown function: {}", name))
        }
    }
}
