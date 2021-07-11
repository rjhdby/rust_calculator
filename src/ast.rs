use fmt::{Display, Formatter};
use std::fmt;

use regex::Regex;
use rug::Float;
use rug::float::Constant;
use rug::ops::Pow;

use crate::lambdas::{BinaryCalculator, Calculator, UnaryCalculator};

pub struct ExprCalculator<T: Clone> {
    pub operations: Vec<Operation<T>>,
    handler: Box<dyn PrimitiveHandler<T>>,
}

#[derive(Clone)]
#[derive(PartialEq)]
pub enum OperationType {
    Constant,
    UnaryPrefix,
    UnaryPostfix,
    BinaryInfix,
    Function,
}

pub struct Operation<T: Clone> {
    signature: String,
    description: String,
    op_type: OperationType,
    operands: i8,
    priority: i8,
    calculate: Box<dyn Calculator<T>>,
}

impl<T: 'static + Clone> Clone for Operation<T> {
    fn clone(&self) -> Self {
        return Operation {
            signature: self.signature.clone(),
            description: self.description.clone(),
            op_type: self.op_type.clone(),
            operands: self.operands,
            priority: self.priority,
            calculate: self.calculate.clone(),
        };
    }
}

pub trait PrimitiveHandler<T> {
    fn from_string(&self, input: &String) -> Result<T, ()>;
    fn can_start_with(&self, input: String) -> bool;
    fn to_string(&self, input: T) -> String;
}

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

    fn to_string(&self, input: Float) -> String {
        todo!()
    }
}

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

    fn to_string(&self, input: bool) -> String {
        todo!()
    }
}

pub enum Token<T: Clone> {
    WhiteSpace { pos: usize, val: String },
    Open { pos: usize },
    Close { pos: usize },
    Primitive { pos: usize, val: T },
    Operation { pos: usize, val: Box<Operation<T>> },
    Unknown { pos: usize, val: String },
}

impl<T: 'static + Clone> Clone for Token<T> {
    fn clone(&self) -> Self {
        return match self {
            Token::WhiteSpace { pos, val } => Token::WhiteSpace { pos: *pos, val: val.clone() },
            Token::Open { pos } => Token::Open { pos: *pos },
            Token::Close { pos } => Token::Close { pos: *pos },
            Token::Primitive { pos, val } => Token::Primitive { pos: *pos, val: val.clone() },
            Token::Operation { pos, val } => Token::Operation { pos: *pos, val: val.clone() },
            Token::Unknown { pos, val } => Token::Unknown { pos: *pos, val: val.clone() },
        };
    }
}

enum AstNode<T: Clone> {
    Primitive { val: T, token: Token<T> },
    Unary { op: Box<Operation<T>>, p1: Box<AstNode<T>>, token: Token<T> },
    Binary { op: Box<Operation<T>>, p1: Box<AstNode<T>>, p2: Box<AstNode<T>>, token: Token<T> },
}

impl<T: 'static + Clone> Clone for AstNode<T> {
    fn clone(&self) -> Self {
        return match self {
            AstNode::Primitive { val, token } => AstNode::Primitive { val: val.clone(), token: token.clone() },
            AstNode::Unary { op, p1, token } => AstNode::Unary { op: op.clone(), p1: p1.clone(), token: token.clone() },
            AstNode::Binary { op, p1, p2, token } => AstNode::Binary { op: op.clone(), p1: p1.clone(), p2: p2.clone(), token: token.clone() },
        };
    }
}

#[derive(Clone)]
enum State {
    Empty,
    Primitive,
    Operation,
    WhiteSpace,
}

struct Context<T: Clone> {
    out: Vec<Token<T>>,
    state: State,
    value: String,
    pos: usize,
}

impl<T: 'static + Clone> Clone for Context<T> {
    fn clone(&self) -> Self {
        return Context {
            out: self.out.clone(),
            state: self.state.clone(),
            value: self.value.clone(),
            pos: self.pos,
        };
    }
}

impl<T: 'static + Clone> ExprCalculator<T> {
    const ADD_SUB_ORDER: i8 = 1;
    const MUL_DIV_ORDER: i8 = 2;
    const FUNCTION_ORDER: i8 = 3;
    const CONSTANT_ORDER: i8 = 4;
}

impl<T: 'static + Clone> ExprCalculator<T> {
    pub fn boolean_calculator() -> ExprCalculator<bool> {
        let mut result = ExprCalculator::<bool> {
            operations: Vec::new(),
            handler: Box::new(BoolHandler {}),
        };

        result.add_binary_low_order(
            "|".to_lowercase(),
            "OR".to_string(),
            Box::new(|op1, op2| { op1 | op2 }),
        );

        result.add_binary_low_order(
            "&".to_lowercase(),
            "AND".to_string(),
            Box::new(|op1, op2| { op1 & op2 }),
        );

        result.add_binary_low_order(
            "^".to_lowercase(),
            "XOR".to_string(),
            Box::new(|op1, op2| { op1 ^ op2 }),
        );

        result.add_unary_prefix(
            "!".to_lowercase(),
            "NOT".to_string(),
            Box::new(|op1| { !op1 }),
        );

        return result;
    }

    pub fn float_calculator() -> ExprCalculator<Float> {
        let mut result = ExprCalculator::<Float> {
            operations: Vec::new(),
            handler: Box::new(FloatHandler {}),
        };

        result.add_unary_prefix(
            "-".to_ascii_lowercase(),
            "Negation".to_string(),
            Box::new(|op1| { -op1.clone() }),
        );
        result.add_unary_postfix(
            "!".to_ascii_lowercase(),
            "Factorial".to_string(),
            Box::new(|op1| { Float::with_val(64, Float::factorial((op1 as Float).to_u32_saturating().unwrap())) }),
        );

        result.add_binary_low_order(
            "+".to_ascii_lowercase(),
            "Addition".to_string(),
            Box::new(|op1, op2| { op1 + op2.clone() }),
        );
        result.add_binary_low_order(
            "-".to_ascii_lowercase(),
            "Subtraction".to_string(),
            Box::new(|op1, op2| { op1 - op2.clone() }),
        );
        result.add_binary_high_order(
            "*".to_ascii_lowercase(),
            "Multiplication".to_string(),
            Box::new(|op1, op2| { op1 * op2.clone() }),
        );
        result.add_binary_high_order(
            "/".to_ascii_lowercase(),
            "Division".to_string(),
            Box::new(|op1, op2| { op1 / op2.clone() }),
        );
        result.add_binary(
            "^".to_ascii_lowercase(),
            "Product".to_string(),
            Box::new(|op1, op2| { op1.pow(op2.clone()) }),
            ExprCalculator::<T>::FUNCTION_ORDER,
        );
        result.add_function(
            "sqrt".to_string(),
            "Square root".to_string(),
            Box::new(|op1| { op1.clone().sqrt() }),
        );
        result.add_function(
            "sin".to_string(),
            "Sine".to_string(),
            Box::new(|op1| { op1.clone().sin() }),
        );
        result.add_function(
            "cos".to_string(),
            "Cosine".to_string(),
            Box::new(|op1| { op1.clone().cos() }),
        );
        result.add_function(
            "ln".to_string(),
            "Natural logarithm".to_string(),
            Box::new(|op1| { op1.clone().ln() }),
        );
        result.add_function(
            "log10".to_string(),
            "Common logarithm".to_string(),
            Box::new(|op1| { op1.clone().log10() }),
        );
        result.add_function(
            "log2".to_string(),
            "Binary logarithm".to_string(),
            Box::new(|op1| { op1.clone().log2() }),
        );
        result.add_function(
            "exp".to_string(),
            "Exponent".to_string(),
            Box::new(|op1| { op1.clone().exp() }),
        );
        result.add_constant(
            "pi".to_string(),
            "Constant 𝜋=3.1415...".to_string(),
            Float::with_val(64, Constant::Pi),
        );
        result.add_constant(
            "e".to_string(),
            "Constant e=2.7182....".to_string(),
            Float::with_val(64, 1).exp(),
        );

        return result;
    }

    pub fn new(handler: Box<dyn PrimitiveHandler<T>>) -> ExprCalculator<T> {
        return ExprCalculator {
            operations: Vec::<Operation<T>>::new(),
            handler,
        };
    }

    pub fn add_unary_prefix(&mut self, signature: String, description: String, calculate: Box<dyn UnaryCalculator<T>>) {
        self.operations.push(Operation {
            signature,
            description,
            op_type: OperationType::UnaryPrefix,
            operands: 1,
            priority: ExprCalculator::<T>::CONSTANT_ORDER,
            calculate: Box::new(move |operands| { calculate(operands[0].clone()) }),
        });
    }

    pub fn add_unary_postfix(&mut self, signature: String, description: String, calculate: Box<dyn UnaryCalculator<T>>) {
        self.operations.push(Operation {
            signature,
            description,
            op_type: OperationType::UnaryPostfix,
            operands: 1,
            priority: ExprCalculator::<T>::CONSTANT_ORDER,
            calculate: Box::new(move |operands| { calculate(operands[0].clone()) }),
        });
    }

    pub fn add_constant(&mut self, signature: String, description: String, value: T) {
        self.operations.push(Operation {
            signature,
            description,
            op_type: OperationType::Constant,
            operands: 0,
            priority: ExprCalculator::<T>::CONSTANT_ORDER,
            calculate: Box::new(move |_| { value.clone() }),
        });
    }

    pub fn add_function(&mut self, signature: String, description: String, calculate: Box<dyn UnaryCalculator<T>>) {
        self.operations.push(Operation {
            signature,
            description,
            op_type: OperationType::Function,
            operands: 1,
            priority: ExprCalculator::<T>::FUNCTION_ORDER,
            calculate: Box::new(move |operands| { calculate(operands[0].clone()) }),
        });
    }

    pub fn add_binary_high_order(&mut self, signature: String, description: String, calculate: Box<dyn BinaryCalculator<T>>) {
        self.add_binary(signature, description, calculate, ExprCalculator::<T>::MUL_DIV_ORDER)
    }

    pub fn add_binary_low_order(&mut self, signature: String, description: String, calculate: Box<dyn BinaryCalculator<T>>) {
        self.add_binary(signature, description, calculate, ExprCalculator::<T>::ADD_SUB_ORDER)
    }

    fn add_binary(&mut self, signature: String, description: String, calculate: Box<dyn BinaryCalculator<T>>, order: i8) {
        self.operations.push(Operation {
            signature,
            description,
            op_type: OperationType::BinaryInfix,
            operands: 2,
            priority: order,
            calculate: Box::new(move |operands| { calculate(operands[0].clone(), operands[1].clone()) }),
        });
    }
}

impl<T: 'static + Clone> ExprCalculator<T> {
    pub fn calculate(&self, input: &str) -> Result<T, Token<T>> {
        let tokens = self.tokenize(input)?;
        println!("TOKENIZED");
        let ast = self.build_ast(&tokens)?;
        println!("ASTED");
        let result = ast.calculate();
        println!("CALCULATED");

        return Result::Ok(result.ok().unwrap().clone());
    }

    pub fn make_not_unary_operation(&self, value: String) -> Option<&Operation<T>> {
        for operation in self.operations.iter() {
            if operation.signature == value && operation.op_type != OperationType::UnaryPrefix {
                return Option::Some(&operation);
            }
        }

        return None;
    }

    pub(crate) fn can_be_operation(&self, text: &String) -> bool {
        for operation in self.operations.iter() {
            if operation.signature.starts_with(text) {
                return true;
            }
        }

        return false;
    }

    pub fn make_by_type(&self, value: String, op_type: OperationType) -> Option<&Operation<T>> {
        for operation in self.operations.iter() {
            if operation.signature == value && operation.op_type == op_type {
                return Option::Some(&operation);
            }
        }

        return None;
    }

    fn tokenize(&self, input: &str) -> Result<Vec<Token<T>>, Token<T>> {
        let mut context = Context::new();

        for entry in input.chars().into_iter().enumerate() {
            let pos = entry.0;
            let val = entry.1;

            match context.state {
                State::Empty => context.init_token_creation(pos, val, self)?,
                _ if context.is_suitable_for_current_state(val, self) => context.add_symbol(val),
                State::Primitive if context.can_add_to_operation(val, self) => context.mutate_to_operation(val),
                _ => {
                    context.collect_token(self)?;
                    context.init_token_creation(pos, val, self)?
                }
            }
        }

        if !context.is_empty() {
            context.collect_token(self)?;
        }

        return Result::Ok(context.get_tokens().clone());
    }
}

impl<T: 'static + Clone> ExprCalculator<T> {
    fn build_ast(&self, tokens: &Vec<Token<T>>) -> Result<AstNode<T>, Token<T>> {
        let mut stack: Vec<Token<T>> = Vec::new();
        let mut operands: Vec<AstNode<T>> = Vec::new();

        for token in tokens {
            match token {
                Token::Primitive { pos: _, val } => operands.push(AstNode::Primitive { val: val.clone(), token: token.clone() }),
                Token::Open { .. } => stack.push(token.clone()),
                Token::Close { .. } => {
                    while !matches!(stack.last().unwrap(), Token::Open {..}) {
                        self.make_node(&mut operands, stack.pop().unwrap())?;
                    };
                    stack.pop();
                }
                Token::Operation { pos: _, val } => {
                    while !stack.is_empty() {
                        let last_op = match stack.last().unwrap().clone() {
                            Token::Operation { pos: _, val } => val,
                            Token::Open { .. } => break,
                            _ => return Result::Err(stack.last().unwrap().clone())
                        };
                        if last_op.priority < val.priority {
                            break;
                        }
                        self.make_node(&mut operands, stack.pop().unwrap())?;
                    }
                    stack.push(token.clone())
                }
                _ => (),
            }
        }

        while stack.last().is_some() {
            self.make_node(&mut operands, stack.pop().unwrap())?;
        };

        return Result::Ok(operands.pop().unwrap());
    }

    fn make_node(&self, operands: &mut Vec<AstNode<T>>, token: Token<T>) -> Result<(), Token<T>> {
        let copy = token.clone();
        let op = match token {
            Token::Operation { pos: _, val } => val.clone(),
            _ => return Result::Err(token)
        };

        if op.operands == 0 {
            operands.push(AstNode::Primitive { val: (&op.calculate)(Vec::new()), token: copy })
        } else if op.operands == 1 {
            let op_right = operands.pop().unwrap();
            operands.push(AstNode::Unary { op, p1: Box::new(op_right), token: copy })
        } else {
            let op_right = operands.pop().unwrap();
            let op_left = operands.pop();
            operands.push(AstNode::Binary { op, p1: Box::new(op_left.unwrap()), p2: Box::new(op_right), token: copy })
        }

        return Result::Ok(());
    }
}

impl<T: 'static + Clone + Display> Display for Token<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let str = match self {
            Token::WhiteSpace { pos, val } => format!("'{}' at position {}", val, pos),
            Token::Open { pos } => format!("'(' at position {}", pos),
            Token::Close { pos } => format!("')' at position {}", pos),
            Token::Primitive { pos, val } => format!("'{}' at position {}", val, pos),
            Token::Operation { pos, val } => format!("'{}' at position {}", val.signature.to_string(), pos),
            Token::Unknown { pos, val } => format!("'{}' at position {}", val, pos),
        };
        write!(f, "{}", str)
    }
}

impl<T: 'static + Clone> Token<T> {
    pub fn get_pos(&self) -> usize {
        return match self {
            Token::WhiteSpace { pos, val: _val } => *pos,
            Token::Open { pos } => *pos,
            Token::Close { pos } => *pos,
            Token::Primitive { pos, val: _val } => *pos,
            Token::Operation { pos, val: _val } => *pos,
            Token::Unknown { pos, val: _val } => *pos,
        };
    }
}

// impl<T: 'static + Clone> Display for AstNode<T> {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

impl<T: 'static + Clone> AstNode<T> {
    // pub fn to_string(&self) -> String {
    //     return match self {
    //         AstNode::Primitive { val, token: _ } => val.to_string(),
    //         AstNode::Unary { op, p1, token: _ } => op.pretty(p1.to_string(), None),
    //         AstNode::Binary { op, p1, p2, token: _ } => op.pretty(p1.to_string(), Some(p2.to_string())),
    //     };
    // }

    pub fn calculate(&self) -> Result<T, Token<T>> {
        let result = match self {
            AstNode::Primitive { val, token: _ } => val.clone(),
            AstNode::Unary { op, p1, token: _ } => (&op.calculate)(vec![p1.calculate()?]),
            AstNode::Binary { op, p1, p2, token: _ } => (&op.calculate)(vec![p1.calculate()?, p2.calculate()?]),
        };

        return Result::Ok(result);
    }
}

impl<T: 'static + Clone> Context<T> {
    pub fn new() -> Context<T> {
        return Context { out: Vec::new(), state: State::Empty, value: String::new(), pos: 0 };
    }

    pub fn get_tokens(&self) -> &Vec<Token<T>> {
        return &self.out;
    }

    fn init_token_creation(&mut self, pos: usize, val: char, expr_calculator: &ExprCalculator<T>) -> Result<(), Token<T>> {
        self.value = String::new();
        self.state = State::Empty;

        match val {
            ' ' => self.init_whitespace(pos),
            '(' => self.add_token(Token::Open { pos })?,
            ')' => self.add_token(Token::Close { pos })?,
            it if self.can_add_to_primitive(it, expr_calculator) => self.init_primitive(it, pos),
            it if expr_calculator.can_be_operation(&it.to_string()) => self.init_operation(it, pos),
            _ => return Result::Err(Token::Unknown { pos, val: val.to_string() })
        }

        return Result::Ok(());
    }

    pub fn collect_token(&mut self, expr_calculator: &ExprCalculator<T>) -> Result<(), Token<T>> {
        let token = match self.state {
            State::Operation => if self.suitable_for_unary_prefix(self.value.clone(), expr_calculator) {
                let operation = expr_calculator.make_by_type(self.value.clone(), OperationType::UnaryPrefix);
                Token::Operation { pos: self.pos, val: Box::new(operation.unwrap().clone()) }
            } else if self.suitable_for_unary_postfix(self.value.clone(), expr_calculator) {
                let operation = expr_calculator.make_by_type(self.value.clone(), OperationType::UnaryPostfix);
                Token::Operation { pos: self.pos, val: Box::new(operation.unwrap().clone()) }
            } else {
                let operation = expr_calculator.make_not_unary_operation(self.value.clone());
                if operation.is_none() {
                    return Result::Err(Token::Unknown { pos: self.pos, val: self.value.clone() });
                }
                Token::Operation { pos: self.pos, val: Box::new(operation.unwrap().clone()) }
            }
            State::WhiteSpace => self.to_whitespace_token(),
            State::Empty => return Result::Ok(()),
            State::Primitive => self.to_primitive(expr_calculator)?
        };
        self.add_token(token)?;

        Result::Ok(())
    }

    fn add_token(&mut self, token: Token<T>) -> Result<(), Token<T>> {
        // if !self.validate(&self.take_last(), &token) {
        //     return Result::Err(token);
        // }
        self.out.push(token);

        return Result::Ok(());
    }

    fn take_last(&self) -> Option<Token<T>> {
        if self.out.is_empty() {
            return None;
        }
        for x in self.out.clone().iter().rev() {
            if !matches!(x, Token::WhiteSpace {..}) {
                return Some(x.clone());
            }
        }

        return None;
    }

    fn can_add_to_primitive(&self, char: char, expr_calculator: &ExprCalculator<T>) -> bool {
        let mut test = String::from(&self.value);
        test.push(char);

        return expr_calculator.handler.can_start_with(test);
    }

    fn can_add_to_operation(&self, char: char, expr_calculator: &ExprCalculator<T>) -> bool {
        let mut test = String::from(&self.value);
        test.push(char);

        return expr_calculator.can_be_operation(&test);
    }

    fn mutate_to_operation(&mut self, value: char) {
        self.state = State::Operation;
        self.add_symbol(value)
    }

    fn is_suitable_for_current_state(&self, char: char, expr_calculator: &ExprCalculator<T>) -> bool {
        return match self.state {
            State::Empty => false,
            State::Primitive => self.can_add_to_primitive(char, expr_calculator),
            State::Operation => self.can_add_to_operation(char, expr_calculator),
            State::WhiteSpace => char == ' '
        };
    }

    pub fn init_whitespace(&mut self, pos: usize) {
        self.state = State::WhiteSpace;
        self.value = String::from(' ');
        self.pos = pos;
    }

    pub fn init_primitive(&mut self, val: char, pos: usize) {
        self.state = State::Primitive;
        self.value = String::from(val);
        self.pos = pos;
    }

    pub fn init_operation(&mut self, val: char, pos: usize) {
        self.state = State::Operation;
        self.value = String::from(val);
        self.pos = pos;
    }

    pub fn add_symbol(&mut self, symbol: char) {
        self.value.push(symbol)
    }

    fn to_whitespace_token(&self) -> Token<T> {
        return Token::WhiteSpace { pos: self.pos, val: self.value.clone() };
    }

    fn to_primitive(&self, expr_calculator: &ExprCalculator<T>) -> Result<Token<T>, Token<T>> {
        let val = expr_calculator.handler.from_string(&self.value);

        if val.is_err() {
            return Result::Err(Token::Unknown { pos: self.pos, val: self.value.clone() });
        }

        return Result::Ok(Token::Primitive { pos: self.pos, val: val.unwrap() });
    }

    pub fn is_empty(&self) -> bool {
        return self.value.is_empty();
    }

    pub fn suitable_for_unary_prefix(&self, value: String, expr_calculator: &ExprCalculator<T>) -> bool {
        if expr_calculator.make_by_type(value, OperationType::UnaryPrefix).is_none() {
            return false;
        }

        if self.out.is_empty() {
            return true;
        }

        let last = self.take_last();
        if last.is_none() {
            return true;
        }

        return match last.unwrap() {
            Token::Open { .. } => true,
            _ => false
        };
    }

    pub fn suitable_for_unary_postfix(&self, value: String, expr_calculator: &ExprCalculator<T>) -> bool {
        if expr_calculator.make_by_type(value, OperationType::UnaryPostfix).is_none() {
            return false;
        }

        if self.out.is_empty() {
            return false;
        }

        let last = self.take_last();
        if last.is_none() {
            return false;
        }

        return match self.out.last().unwrap() {
            Token::Close { .. } => true,
            Token::Primitive { .. } => true,
            Token::Operation { pos: _pos, val } => return val.op_type == OperationType::Constant,
            _ => true
        };
    }
}

impl<T: 'static + Clone> Operation<T> {
    pub fn pretty(&self, op1: String, op2: Option<String>) -> String {
        return match self {
            it if it.operands == 2 => format!("({}{}{})", op1, self.signature, op2.unwrap()),
            it if it.operands == 1 => format!("{}({})", self.signature, op1),
            _ => self.signature.clone(),
        };
    }

    pub fn description(&self) -> String {
        return self.description.clone();
    }
}

lazy_static! {
    // static ref EXP: Regex = Regex::new(r"^-?(\d+|\d+\.\d+)[eE][-+]?\d+$").unwrap();
    static ref PRIMITIVE_INCOMPLETE_1: Regex = Regex::new(r"^(\d+|\d+\.\d*)$").unwrap();
    static ref PRIMITIVE_INCOMPLETE_2: Regex = Regex::new(r"^(\d+|\d+\.\d+)[eE][-+]?\d*$").unwrap();
}
