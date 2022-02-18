use crate::operation_executor::{BinaryOperationExecutor, OperationExecutor, UnaryoperationExecutor};

pub struct ExprCalculator<T: Clone> {
    pub(crate) operations: Vec<Operation<T>>,
    pub(crate) handler: Box<dyn PrimitiveHandler<T>>,
}

#[derive(Clone)]
#[derive(PartialEq)]
pub enum OperationType {
    Constant,
    Prefix,
    Postfix,
    Infix,
    Function,
}
pub struct Operation<T: Clone> {
    signature: String,
    description: String,
    op_type: OperationType,
    operands: u8,
    priority: u8,
    calculate: Box<dyn OperationExecutor<T>>,
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
}

pub enum Token<T: Clone> {
    WhiteSpace { pos: usize, val: String },
    Open { pos: usize },
    Close { pos: usize },
    Primitive { pos: usize, val: T, original: String },
    Operation { pos: usize, val: Box<Operation<T>> },
    Unknown { pos: usize, val: String },
}

impl<T: 'static + Clone> Clone for Token<T> {
    fn clone(&self) -> Self {
        return match self {
            Token::WhiteSpace { pos, val } => Token::WhiteSpace { pos: *pos, val: val.clone() },
            Token::Open { pos } => Token::Open { pos: *pos },
            Token::Close { pos } => Token::Close { pos: *pos },
            Token::Primitive { pos, val, original } => Token::Primitive { pos: *pos, val: val.clone(), original: original.clone() },
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

pub const LOWEST_ORDER: u8 = 10;
pub const LOW_ORDER: u8 = 20;
pub const MEDIUM_ORDER: u8 = 30;
pub const HIGH_ORDER: u8 = 40;
pub const HIGHEST_ORDER: u8 = 50;

impl<T: 'static + Clone> ExprCalculator<T> {
    pub fn new(handler: Box<dyn PrimitiveHandler<T>>) -> ExprCalculator<T> {
        return ExprCalculator {
            operations: Vec::<Operation<T>>::new(),
            handler,
        };
    }

    pub fn add(
        &mut self,
        signature: String,
        description: String,
        op_type: OperationType,
        calculate: Box<dyn OperationExecutor<T>>,
        operands: u8,
        order: u8,
    ) {
        self.operations.push(Operation {
            signature,
            description,
            op_type,
            operands,
            priority: order,
            calculate,
        });
    }

    pub fn add_prefix(
        &mut self,
        signature: String,
        description: String,
        calculate: Box<dyn UnaryoperationExecutor<T>>,
        order: u8,
    ) {
        self.add_unary(signature, description, calculate, OperationType::Prefix, order);
    }

    pub fn add_postfix(
        &mut self,
        signature: String,
        description: String,
        calculate: Box<dyn UnaryoperationExecutor<T>>,
        order: u8,
    ) {
        self.add_unary(signature, description, calculate, OperationType::Postfix, order);
    }

    fn add_unary(
        &mut self,
        signature: String,
        description: String,
        calculate: Box<dyn UnaryoperationExecutor<T>>,
        op_type: OperationType,
        order: u8,
    ) {
        self.operations.push(Operation {
            signature,
            description,
            op_type,
            operands: 1,
            priority: order,
            calculate: Box::new(move |operands| { calculate(operands[0].clone()) }),
        });
    }

    pub fn add_constant(&mut self, signature: String, description: String, value: T) {
        self.operations.push(Operation {
            signature,
            description,
            op_type: OperationType::Constant,
            operands: 0,
            priority: u8::MAX,
            calculate: Box::new(move |_| { value.clone() }),
        });
    }

    pub fn add_one_argument_function(
        &mut self,
        signature: String,
        description: String,
        calculate: Box<dyn UnaryoperationExecutor<T>>,
        order: u8,
    ) {
        self.operations.push(Operation {
            signature,
            description,
            op_type: OperationType::Function,
            operands: 1,
            priority: order,
            calculate: Box::new(move |operands| { calculate(operands[0].clone()) }),
        });
    }

    pub fn add_infix(&mut self, signature: String, description: String, calculate: Box<dyn BinaryOperationExecutor<T>>, order: u8) {
        self.operations.push(Operation {
            signature,
            description,
            op_type: OperationType::Infix,
            operands: 2,
            priority: order,
            calculate: Box::new(move |operands| { calculate(operands[0].clone(), operands[1].clone()) }),
        });
    }
}

impl<T: 'static + Clone> ExprCalculator<T> {
    pub fn calculate(&self, input: &str) -> Result<T, Token<T>> {
        let tokens = self.tokenize(input)?;
        let ast = self.build_ast(&tokens)?;
        let result = ast.calculate();

        return Result::Ok(result.ok().unwrap().clone());
    }

    fn can_be_operation(&self, text: &String) -> bool {
        for operation in self.operations.iter() {
            if operation.signature.starts_with(text) {
                return true;
            }
        }

        return false;
    }

    fn make_by_type(&self, value: &String, op_type: OperationType) -> Option<&Operation<T>> {
        for operation in self.operations.iter() {
            if operation.signature == *value && operation.op_type == op_type {
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
                Token::Primitive { pos: _pos, val, .. } => operands.push(AstNode::Primitive { val: val.clone(), token: token.clone() }),
                Token::Open { .. } => stack.push(token.clone()),
                Token::Close { .. } => {
                    loop {
                        let last = stack.last();
                        if last.is_none() {
                            return Result::Err(token.clone());
                        }
                        if matches!(stack.last().unwrap(), Token::Open {..}) {
                            break;
                        }
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
                        if last_op.priority <= val.priority {
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

impl<T: 'static + Clone> Token<T> {
    pub fn to_string(&self) -> String {
        return match self {
            Token::WhiteSpace { pos, val } => format!("'{}' at position {}", val, pos),
            Token::Open { pos } => format!("'(' at position {}", pos),
            Token::Close { pos } => format!("')' at position {}", pos),
            Token::Primitive { pos, val: _val, original } => format!("'{}' at position {}", original, pos),
            Token::Operation { pos, val } => format!("'{}' at position {}", val.signature.to_string(), pos),
            Token::Unknown { pos, val } => format!("'{}' at position {}", val, pos),
        };
    }
}

impl<T: 'static + Clone> Token<T> {
    pub fn get_pos(&self) -> usize {
        return match self {
            Token::WhiteSpace { pos, .. } => *pos,
            Token::Open { pos } => *pos,
            Token::Close { pos } => *pos,
            Token::Primitive { pos, .. } => *pos,
            Token::Operation { pos, .. } => *pos,
            Token::Unknown { pos, .. } => *pos,
        };
    }

    pub fn get_value(&self) -> String {
        return match self {
            Token::WhiteSpace { pos: _pos, val } => val.clone(),
            Token::Open { .. } => "(".to_string(),
            Token::Close { .. } => ")".to_string(),
            Token::Primitive { pos: _pos, val: _val, original } => original.clone(),
            Token::Operation { pos: _pos, val } => val.signature.clone(),
            Token::Unknown { pos: _pos, val } => val.clone()
        };
    }
}

impl<T: 'static + Clone> AstNode<T> {
    fn calculate(&self) -> Result<T, Token<T>> {
        let result = match self {
            AstNode::Primitive { val, .. } => val.clone(),
            AstNode::Unary { op, p1, .. } => (&op.calculate)(vec![p1.calculate()?]),
            AstNode::Binary { op, p1, p2, .. } => (&op.calculate)(vec![p1.calculate()?, p2.calculate()?]),
        };

        return Result::Ok(result);
    }
}

impl<T: 'static + Clone> Context<T> {
    pub fn new() -> Context<T> {
        return Context { out: Vec::new(), state: State::Empty, value: String::new(), pos: 0 };
    }

    fn get_tokens(&self) -> &Vec<Token<T>> {
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

    fn collect_token(&mut self, expr_calculator: &ExprCalculator<T>) -> Result<(), Token<T>> {
        let token = match self.state {
            State::Operation => {
                let op = self.value.clone();
                let op_type = if self.suitable_for_prefix(&op, expr_calculator) {
                    OperationType::Prefix
                } else if self.suitable_for_postfix(&op, expr_calculator) {
                    OperationType::Postfix
                } else if self.suitable_for_infix(&op, expr_calculator) {
                    OperationType::Infix
                } else if self.suitable_for_function(&op, expr_calculator) {
                    OperationType::Function
                } else if expr_calculator.make_by_type(&op, OperationType::Constant).is_some() {
                    OperationType::Constant
                } else {
                    return Result::Err(Token::Unknown { pos: self.pos, val: self.value.clone() });
                };

                let operation = expr_calculator.make_by_type(&op, op_type);

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

    fn init_whitespace(&mut self, pos: usize) {
        self.state = State::WhiteSpace;
        self.value = String::from(' ');
        self.pos = pos;
    }

    fn init_primitive(&mut self, val: char, pos: usize) {
        self.state = State::Primitive;
        self.value = String::from(val);
        self.pos = pos;
    }

    fn init_operation(&mut self, val: char, pos: usize) {
        self.state = State::Operation;
        self.value = String::from(val);
        self.pos = pos;
    }

    fn add_symbol(&mut self, symbol: char) {
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

        return Result::Ok(Token::Primitive { pos: self.pos, val: val.unwrap(), original: self.value.clone() });
    }

    fn is_empty(&self) -> bool {
        return self.value.is_empty();
    }

    fn suitable_for_prefix(&self, value: &String, expr_calculator: &ExprCalculator<T>) -> bool {
        if expr_calculator.make_by_type(value, OperationType::Prefix).is_none() {
            return false;
        }

        let last = self.take_last();
        if last.is_none() {
            return true;
        }

        return match last.unwrap() {
            Token::Open { .. } => true,
            Token::Operation { pos: _pos, val } => match val.op_type {
                OperationType::Constant => false,
                _ => true
            }
            _ => false
        };
    }

    fn suitable_for_postfix(&self, value: &String, expr_calculator: &ExprCalculator<T>) -> bool {
        if expr_calculator.make_by_type(value, OperationType::Postfix).is_none() {
            return false;
        }

        let last = self.take_last();
        if last.is_none() {
            return false;
        }

        return match last.unwrap() {
            Token::Close { .. } => true,
            Token::Primitive { .. } => true,
            Token::Operation { pos: _pos, val } => match val.op_type {
                OperationType::Constant | OperationType::Postfix => true,
                _ => false
            }
            _ => false
        };
    }

    fn suitable_for_infix(&self, value: &String, expr_calculator: &ExprCalculator<T>) -> bool {
        if expr_calculator.make_by_type(value, OperationType::Infix).is_none() {
            return false;
        }

        let last = self.take_last();
        if last.is_none() {
            return false;
        }

        return match last.unwrap() {
            Token::Close { .. } => true,
            Token::Primitive { .. } => true,
            Token::Operation { pos: _pos, val } => match val.op_type {
                OperationType::Constant | OperationType::Postfix => true,
                _ => false
            }
            _ => false
        };
    }

    fn suitable_for_function(&self, value: &String, expr_calculator: &ExprCalculator<T>) -> bool {
        if expr_calculator.make_by_type(value, OperationType::Function).is_none() {
            return false;
        }

        let last = self.take_last();
        if last.is_none() {
            return true;
        }

        return match last.unwrap() {
            Token::Open { .. } => true,
            Token::Operation { pos: _pos, val } => match val.op_type {
                OperationType::Infix | OperationType::Prefix => true,
                _ => false
            },
            _ => false
        };
    }
}

impl<T: 'static + Clone> Operation<T> {
    pub fn pretty(&self) -> String {
        return match self.op_type.clone() {
            OperationType::Constant => self.signature.clone(),
            OperationType::Prefix => format!("{}x", &self.signature),
            OperationType::Postfix => format!("x{}", &self.signature),
            OperationType::Infix => format!("x{}y", &self.signature),
            OperationType::Function => format!(
                "{}({})",
                &self.signature,
                vec!["x", "y", "z", "a", "b", "c", "..."]
                    .iter()
                    .take(usize::from(self.operands))
                    .map(|it| { it.to_string() })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        };
    }

    pub fn description(&self) -> String {
        return self.description.clone();
    }

    pub fn priority(&self) -> u8 {
        return self.priority;
    }
}