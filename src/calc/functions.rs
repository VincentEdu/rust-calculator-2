use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::calc::context::Context;

pub use usize as FunctionId;
// all function ids, function id must be index of corresponding function in ALL_FUNCTIONS
pub const ID_ADD: FunctionId = 0;
pub const ID_SUB: FunctionId = 1;
pub const ID_MUL: FunctionId = 2;
pub const ID_DIV: FunctionId = 3;
pub const ID_MOD: FunctionId = 4;
pub const ID_POW: FunctionId = 5;
pub const ID_SQRT: FunctionId = 6;
pub const ID_ABS: FunctionId = 7;
pub const ID_NEG: FunctionId = 8;
pub const ID_SIN: FunctionId = 9;
pub const ID_COS: FunctionId = 10;
pub const ID_TAN: FunctionId = 11;
pub const ID_LN: FunctionId = 12;
pub const ID_OPEN_BRACKET: FunctionId = 13;
pub const ID_CLOSE_BRACKET: FunctionId = 14;
pub const ID_SQR: FunctionId = 15;
pub const ID_INV: FunctionId = 16;

pub const PRIODITY_ADDITIVE: i32 = 6;
pub const PRIODITY_MULTIPLICATIVE: i32 = 5;
pub const PRIODITY_USER_FUNCTION: i32 = 2;
pub const PRIODITY_UNARY_OP: i32 = 3;
/// A trait for a function that can be executed.
pub trait Functor {
    fn execute(&self);
    fn priority(&self) -> i32;
    fn id(&self) -> FunctionId;
    fn arg_count(&self) -> i32;
}

/// open bracket
pub struct OpenBracket {}
impl Functor for OpenBracket {
    fn execute(&self) {
        Context::with_current(|c| {
            let mut t = c.borrow_mut();
            t.error_detected = true;
            t.error_message = "Open bracket is not a function".to_string();
        });
    }
    fn priority(&self) -> i32 {
        0
    }
    fn id(&self) -> FunctionId {
        ID_OPEN_BRACKET
    }
    fn arg_count(&self) -> i32 {
        0
    }
}

/// close bracket
pub struct CloseBracket {}
impl Functor for CloseBracket {
    fn execute(&self) {
        Context::with_current(|c| {
            let mut t = c.borrow_mut();
            t.error_detected = true;
            t.error_message = "Close bracket is not a function".to_string();
        });
    }
    fn priority(&self) -> i32 {
        999
    }
    fn id(&self) -> FunctionId {
        ID_CLOSE_BRACKET
    }
    fn arg_count(&self) -> i32 {
        0
    }
}

/// A trait for a function with only one parameter
pub trait UnaryFunctor : Functor {
    fn compute(&self, a: f64) -> Result<f64, &str>;
    fn execute(&self) {
        Context::with_current(|c| {
            let mut t = c.borrow_mut();
            if t.execution_stack.size() < 1 {
                t.error_detected = true;
                return;
            }
            let a = t.execution_stack.pop_val().unwrap();
            let result = self.compute(a);
            match result {
                Ok(v) => t.execution_stack.push_val(v),
                Err(s) => {
                    t.error_message = s.to_string();
                    t.error_detected = true;
                }
            }
        });
    }
}

/// A trait for a function with two parameters
pub trait BinaryFunctor {
    fn compute(&self, a: f64, b: f64) -> Result<f64, &str>;
    fn execute(&self) {
        Context::with_current(|c| {
            let mut t = c.borrow_mut();
            if t.execution_stack.size() < 2 {
                t.error_detected = true;
                return;
            }
            let b = t.execution_stack.pop_val().unwrap();
            let a = t.execution_stack.pop_val().unwrap();
            let result = self.compute(a, b);
            match result {
                Ok(v) => t.execution_stack.push_val(v),
                Err(s) => {
                    t.error_message = s.to_string();
                    t.error_detected = true;
                }
            }
        });
    }
}
/// Add function
pub struct Add {
}
impl Functor for Add {
    fn execute(&self) {
        BinaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_ADD
    }
    fn priority(&self) -> i32 {
        PRIODITY_ADDITIVE
    }
    fn arg_count(&self) -> i32 {
        2
    }
}
impl BinaryFunctor for Add {
    fn compute(&self, a: f64, b: f64) -> Result<f64, &str> {
        Ok(a + b)
    }
}

/// Sub function
pub struct Sub {
}
impl Functor for Sub {
    fn execute(&self) {
        BinaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_SUB
    }
    fn priority(&self) -> i32 {
        PRIODITY_ADDITIVE
    }
    fn arg_count(&self) -> i32 {
        2
    }
}
impl BinaryFunctor for Sub {
    fn compute(&self, a: f64, b: f64) -> Result<f64, &str> {
        Ok(a - b)
    }
}

/// Mul function
pub struct Mul {
}
impl Functor for Mul {
    fn execute(&self) {
        BinaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_MUL
    }
    fn priority(&self) -> i32 {
        PRIODITY_MULTIPLICATIVE
    }
    fn arg_count(&self) -> i32 {
        2
    }
}
impl BinaryFunctor for Mul {
    fn compute(&self, a: f64, b: f64) -> Result<f64, &str> {
        Ok(a * b)
    }
}

/// Div function
pub struct Div {
}
impl Functor for Div {
    fn execute(&self) {
        BinaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_DIV
    }
    fn priority(&self) -> i32 {
        PRIODITY_MULTIPLICATIVE
    }
    fn arg_count(&self) -> i32 {
        2
    }
}
impl BinaryFunctor for Div {
    fn compute(&self, a: f64, b: f64) -> Result<f64, &str> {
        if b == 0.0 {Err("divide to zero")} else {Ok(a / b)}
    }
}

/// sin function
pub struct Sin {
}
impl Functor for Sin {
    fn execute(&self) {
        UnaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_SIN
    }
    fn priority(&self) -> i32 {
        PRIODITY_UNARY_OP
    }
    fn arg_count(&self) -> i32 {
        1
    }
}
impl UnaryFunctor for Sin {
    fn compute(&self, a: f64) -> Result<f64, &str> {
        Ok(a.sin())
    }
}

/// cos function
pub struct Cos {
}
impl Functor for Cos {
    fn execute(&self) {
        UnaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_COS
    }
    fn priority(&self) -> i32 {
        PRIODITY_UNARY_OP
    }
    fn arg_count(&self) -> i32 {
        1
    }
}
impl UnaryFunctor for Cos {
    fn compute(&self, a: f64) -> Result<f64, &str> {
        Ok(a.cos())
    }
}

/// tan function
pub struct Tan {
}
impl Functor for Tan {
    fn execute(&self) {
        UnaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_TAN
    }
    fn priority(&self) -> i32 {
        PRIODITY_UNARY_OP
    }
    fn arg_count(&self) -> i32 {
        1
    }
}
impl UnaryFunctor for Tan {
    fn compute(&self, a: f64) -> Result<f64, &str> {
        Ok(a.tan())
    }
}

/// sqrt function
pub struct Sqrt {}
impl Functor for Sqrt {
    fn execute(&self) {
        UnaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_SQRT
    }
    fn priority(&self) -> i32 {
        PRIODITY_UNARY_OP
    }
    fn arg_count(&self) -> i32 {
        1
    }
}
impl UnaryFunctor for Sqrt {
    fn compute(&self, a: f64) -> Result<f64, &str> {
        if a < 0.0 {
            Err("Square root of negative number is undefined")
        } else {
            Ok(a.sqrt())
        }
    }
}

/// sqr function
pub struct Sqr {}
impl Functor for Sqr {
    fn execute(&self) {
        UnaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_SQR
    }
    fn priority(&self) -> i32 {
        PRIODITY_UNARY_OP
    }
    fn arg_count(&self) -> i32 {
        1
    }
}
impl UnaryFunctor for Sqr {
    fn compute(&self, a: f64) -> Result<f64, &str> {
        Ok(a * a)
    }
}
/// 1/x function
pub struct Inv {}
impl Functor for Inv {
    fn execute(&self) {
        UnaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_INV
    }
    fn priority(&self) -> i32 {
        PRIODITY_UNARY_OP
    }
    fn arg_count(&self) -> i32 {
        1
    }
}
impl UnaryFunctor for Inv {
    fn compute(&self, a: f64) -> Result<f64, &str> {
        if a == 0.0 {
            Err("Divide to zero")
        } else {
            Ok(1.0 / a)
        }
    }
}


type FunctionCreator = fn(&String) -> Box<dyn Functor>;

pub struct FunctionLib {
    function_creator_map: HashMap<String, FunctionCreator>,
}

impl FunctionLib {
    pub fn new() -> Self {
        let mut function_creator_map: HashMap<String, FunctionCreator> = HashMap::new();
        function_creator_map.insert("+".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Add{}) });
        function_creator_map.insert("-".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Sub{}) });
        function_creator_map.insert("*".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Mul{}) });
        function_creator_map.insert("/".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Div{}) });
        function_creator_map.insert("sin".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Sin{}) });
        function_creator_map.insert("cos".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Cos{}) });
        function_creator_map.insert("tan".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Tan{}) });
        function_creator_map.insert("√".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Sqrt{}) });
        function_creator_map.insert("²".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Sqr{}) });
        function_creator_map.insert("⅟".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Inv{}) });
        function_creator_map.insert("(".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(OpenBracket{}) });
        function_creator_map.insert(")".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(CloseBracket{}) });
        Self {
            function_creator_map
        }        
    }

    pub fn get_functor(&self, name: &String) -> Option<Box<dyn Functor>> {
        self.function_creator_map.get(name).map(|fn_creator| {
            fn_creator(name)
        })
    }
}

lazy_static! {
    pub static ref FUNCTION_LIB: FunctionLib = FunctionLib::new();
}