use crate::ast::{ExprCalculator, HIGHEST_ORDER, LOWEST_ORDER, LOW_ORDER, MEDIUM_ORDER, HIGH_ORDER, PrimitiveHandler};
use rug::Float;
use rug::float::Constant;
use regex::Regex;
use rug::ops::Pow;

struct FloatHandler {}

impl PrimitiveHandler<Float> for FloatHandler {
    fn from_string(&self, input: &String) -> Result<Float, ()> {
        let valid = Float::parse(input);
        if valid.is_err() {
            return Result::Err(());
        }

        return Result::Ok(Float::with_val(64, valid.unwrap()));
    }

    fn can_start_with(&self, input: String) -> bool {
        return PRIMITIVE_INCOMPLETE_1.is_match(&input) || PRIMITIVE_INCOMPLETE_2.is_match(&input);
    }
}

pub fn float_calculator() -> ExprCalculator<Float> {
    let mut result = ExprCalculator::<Float>::new(Box::new(FloatHandler {}));

    result.add_prefix(
        "-".to_ascii_lowercase(),
        "Negation".to_string(),
        Box::new(|op1| { -op1.clone() }),
        HIGH_ORDER
    );
    result.add_postfix(
        "!".to_ascii_lowercase(),
        "Factorial".to_string(),
        Box::new(|op1| { Float::with_val(64, Float::factorial((op1 as Float).to_u32_saturating().unwrap())) }),
        HIGHEST_ORDER
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
        Box::new(|op1, op2| { op1.pow(op2.clone()) }),
        MEDIUM_ORDER,
    );
    result.add_prefix(
        "sqrt".to_string(),
        "Square root".to_string(),
        Box::new(|op1| { op1.clone().sqrt() }),
        HIGHEST_ORDER
    );
    result.add_one_argument_function(
        "sin".to_string(),
        "Sine".to_string(),
        Box::new(|op1| { op1.clone().sin() }),
        HIGHEST_ORDER
    );
    result.add_one_argument_function(
        "cos".to_string(),
        "Cosine".to_string(),
        Box::new(|op1| { op1.clone().cos() }),
        HIGHEST_ORDER
    );
    result.add_one_argument_function(
        "ln".to_string(),
        "Natural logarithm".to_string(),
        Box::new(|op1| { op1.clone().ln() }),
        HIGHEST_ORDER
    );
    result.add_one_argument_function(
        "log10".to_string(),
        "Common logarithm".to_string(),
        Box::new(|op1| { op1.clone().log10() }),
        HIGHEST_ORDER
    );
    result.add_one_argument_function(
        "log2".to_string(),
        "Binary logarithm".to_string(),
        Box::new(|op1| { op1.clone().log2() }),
        HIGHEST_ORDER
    );
    result.add_one_argument_function(
        "exp".to_string(),
        "Exponent".to_string(),
        Box::new(|op1| { op1.clone().exp() }),
        HIGHEST_ORDER
    );
    result.add_constant(
        "pi".to_string(),
        "Constant Pi=3.1415...".to_string(),
        Float::with_val(64, Constant::Pi),
    );
    result.add_constant(
        "e".to_string(),
        "Constant e=2.7182....".to_string(),
        Float::with_val(64, 1).exp(),
    );

    return result;
}

lazy_static! {
    static ref PRIMITIVE_INCOMPLETE_1: Regex = Regex::new(r"^(\d+|\d+\.\d*)$").unwrap();
    static ref PRIMITIVE_INCOMPLETE_2: Regex = Regex::new(r"^(\d+|\d+\.\d+)[eE][-+]?\d*$").unwrap();
}
