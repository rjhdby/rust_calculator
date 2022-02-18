use crate::ast::{ExprCalculator, HIGH_ORDER, LOW_ORDER, PrimitiveHandler};

struct BoolHandler {}

impl PrimitiveHandler<bool> for BoolHandler {
    fn from_string(&self, input: &String) -> Result<bool, ()> {
        return match input.to_lowercase().as_str() {
            "true" => Result::Ok(true),
            "false" => Result::Ok(false),
            _ => Result::Err(()),
        };
    }

    fn can_start_with(&self, input: String) -> bool {
        return "true".starts_with(&input) || "false".starts_with(&input);
    }
}

pub fn boolean_calculator() -> ExprCalculator<bool> {
    let mut result = ExprCalculator::<bool>::new(Box::new(BoolHandler {}));

    result.add_infix(
        "|".to_lowercase(),
        "OR".to_string(),
        Box::new(|op1, op2| { op1 | op2 }),
        LOW_ORDER,
    );

    result.add_infix(
        "&".to_lowercase(),
        "AND".to_string(),
        Box::new(|op1, op2| { op1 & op2 }),
        LOW_ORDER,
    );

    result.add_infix(
        "^".to_lowercase(),
        "XOR".to_string(),
        Box::new(|op1, op2| { op1 ^ op2 }),
        LOW_ORDER,
    );

    result.add_prefix(
        "!".to_lowercase(),
        "NOT".to_string(),
        Box::new(|op1| { !op1 }),
        HIGH_ORDER,
    );

    return result;
}