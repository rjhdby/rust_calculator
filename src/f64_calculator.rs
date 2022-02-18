use crate::ast::{ExprCalculator, HIGHEST_ORDER, LOWEST_ORDER, LOW_ORDER, MEDIUM_ORDER, HIGH_ORDER, PrimitiveHandler};
use regex::Regex;
use std::str::FromStr;
use std::f64::consts::{PI, E};

struct F64Handler {}

impl PrimitiveHandler<f64> for F64Handler {
    fn from_string(&self, input: &String) -> Result<f64, ()> {
        let result = f64::from_str(input);
        if result.is_err() {
            return Result::Err(())
        }

        return Result::Ok(result.ok().unwrap());
    }

    fn can_start_with(&self, input: String) -> bool {
        return PRIMITIVE_INCOMPLETE_1.is_match(&input) || PRIMITIVE_INCOMPLETE_2.is_match(&input);
    }
}

pub fn f64_calculator() -> ExprCalculator<f64> {
    let mut result = ExprCalculator::<f64>::new(Box::new(F64Handler {}));

    result.add_prefix(
        "-".to_ascii_lowercase(),
        "Negation".to_string(),
        Box::new(|op1| { -op1.clone() }),
        HIGH_ORDER
    );

    result.add_infix(
        "+".to_ascii_lowercase(),
        "Addition".to_string(),
        Box::new(|op1, op2| { op1 + op2.clone() }),
        LOWEST_ORDER,
    );
    result.add_infix(
        "-".to_ascii_lowercase(),
        "Subtraction".to_string(),
        Box::new(|op1, op2| { op1 - op2.clone() }),
        LOWEST_ORDER,
    );
    result.add_infix(
        "*".to_ascii_lowercase(),
        "Multiplication".to_string(),
        Box::new(|op1, op2| { op1 * op2.clone() }),
        LOW_ORDER,
    );
    result.add_infix(
        "/".to_ascii_lowercase(),
        "Division".to_string(),
        Box::new(|op1, op2| { op1 / op2.clone() }),
        LOW_ORDER,
    );
    result.add_infix(
        "^".to_ascii_lowercase(),
        "Product".to_string(),
        Box::new(|op1, op2| { op1.powf(op2.clone()) }),
        MEDIUM_ORDER,
    );
    result.add_postfix(
        "sqrt".to_string(),
        "Square root".to_string(),
        Box::new(|op1| { op1.clone().sqrt() }),
        HIGHEST_ORDER
    );
    result.add_prefix(
        "sin".to_string(),
        "Sine".to_string(),
        Box::new(|op1| { op1.clone().sin() }),
        HIGHEST_ORDER
    );
    result.add_prefix(
        "cos".to_string(),
        "Cosine".to_string(),
        Box::new(|op1| { op1.clone().cos() }),
        HIGHEST_ORDER
    );
    result.add_prefix(
        "ln".to_string(),
        "Natural logarithm".to_string(),
        Box::new(|op1| { op1.clone().ln() }),
        HIGHEST_ORDER
    );
    result.add_prefix(
        "log10".to_string(),
        "Common logarithm".to_string(),
        Box::new(|op1| { op1.clone().log10() }),
        HIGHEST_ORDER
    );
    result.add_prefix(
        "log2".to_string(),
        "Binary logarithm".to_string(),
        Box::new(|op1| { op1.clone().log2() }),
        HIGHEST_ORDER
    );
    result.add_prefix(
        "exp".to_string(),
        "Exponent".to_string(),
        Box::new(|op1| { op1.clone().exp() }),
        HIGHEST_ORDER
    );
    result.add_constant(
        "pi".to_string(),
        "Constant Pi=3.1415...".to_string(),
        PI
    );
    result.add_constant(
        "e".to_string(),
        "Constant e=2.7182....".to_string(),
        E,
    );

    return result;
}

lazy_static! {
    static ref PRIMITIVE_INCOMPLETE_1: Regex = Regex::new(r"^(\d+|\d+\.\d*)$").unwrap();
    static ref PRIMITIVE_INCOMPLETE_2: Regex = Regex::new(r"^(\d+|\d+\.\d+)[eE][-+]?\d*$").unwrap();
}
