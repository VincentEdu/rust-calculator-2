use core::borrow;
use std::{borrow::Borrow, collections::HashMap};
use super::functions::*;
use lazy_static::lazy_static;

pub const EXP_UNIT_NAME_CONSTANT: &str = "constant";
pub const EXP_UNIT_NAME_ADD: &str = "+";
pub const EXP_UNIT_NAME_SUB: &str = "−";
pub const EXP_UNIT_NAME_MUL: &str = "×";
pub const EXP_UNIT_NAME_DIV: &str = "÷";
pub const EXP_UNIT_NAME_SIN: &str = "sin";
pub const EXP_UNIT_NAME_COS: &str = "cos";
pub const EXP_UNIT_NAME_TAN: &str = "tan";
pub const EXP_UNIT_NAME_INV: &str = "⅟";
pub const EXP_UNIT_NAME_SQR: &str = "²";
pub const EXP_UNIT_NAME_SQRT: &str = "√";
pub const EXP_UNIT_NAME_OPEN_BRK: &str = "(";
pub const EXP_UNIT_NAME_CLOSE_BRK: &str = ")";


//// structures
pub struct Expression {
    pub root: Option<Box<dyn ExcutableUnit>>,
}

impl Expression {
    pub fn execute(&self) -> Result<f64, String> {
        if self.root.is_none() {
            return Err("Empty expression".to_string());
        }
        self.root.as_ref().unwrap().execute()
    }

    pub fn to_string(&self) -> String {
        if self.root.is_none() {
            return "".to_string();
        }
        self.root.as_ref().unwrap().to_string()
    }
}

struct ExpUnitBase {
    pub exp_idx: i32,
}

impl ExpUnitBase {
    pub fn new() -> Self {
        Self {
            exp_idx: -1
        }
    }
}

struct ExpOpBase {
    pub unitbase: ExpUnitBase,
    pub id: FunctionId,
    pub precedence: i32,
}

struct BinaryFunctionBase {
    unitbase: ExpOpBase,
    _1: Option<Box<dyn ExcutableUnit>>,
    _2: Option<Box<dyn ExcutableUnit>>,
}

struct UnaryFunctionBase {
    unitbase: ExpOpBase,
    _1: Option<Box<dyn ExcutableUnit>>,
}


struct ConstantUnit {
    pub unitbase: ExpUnitBase,
    pub value: f64,
}

impl ConstantUnit {
    pub fn new(value: f64) -> Self {
        Self {
            unitbase: ExpUnitBase::new(),
            value,
        }
    }
}

impl ExcutableUnit for ConstantUnit {
    fn execute(&self) -> Result<f64, String> {
        Ok(self.value)
    }
}

pub trait ExpUnit {
    fn to_string(&self) -> String;
    fn exp_name(&self) -> &str;
    fn get_exp_unit_base(&self) -> &ExpUnitBase;
    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase;

    fn get_exp_idx(&self) -> i32 {
        self.get_exp_unit_base().exp_idx
    }
    fn set_exp_idx(&mut self, idx: i32) {
        self.get_exp_unit_base_mut().exp_idx = idx;
    }
}

impl ExpUnit for ConstantUnit {
    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn exp_name(&self) -> &str {
        "constant"
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.unitbase
    }
}


//// structures implementation
impl BinaryFunctionBase {
    pub fn new(id: FunctionId, precedence: i32) -> Self {
        Self {
            unitbase: ExpOpBase {
                unitbase: ExpUnitBase::new(),
                id,
                precedence,
            },
            _1: None,
            _2: None,
        }
    }
}


impl UnaryFunctionBase {
    pub fn new(id: FunctionId, precedence: i32) -> Self {
        Self {
            unitbase: ExpOpBase {
                unitbase: ExpUnitBase::new(),
                id,
                precedence
            },
            _1: None,
        }
    }
}

pub trait ExcutableUnit : ExpUnit {
    fn execute(&self) -> Result<f64, String>;    
}

pub trait ExpOpUnit : ExcutableUnit {
    fn get_op_base(&self) -> &ExpOpBase;
    fn get_op_base_mut(&mut self) -> &mut ExpOpBase;
    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32;
    fn arg_count(&self) -> i32;
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit>;
}

pub trait BinaryFunctionUnit: ExpOpUnit {
    fn get_func_base(&self) -> &BinaryFunctionBase;
    fn get_func_base_mut(&mut self) -> &mut BinaryFunctionBase;
    fn execute_with_args(&self, _1: f64, _2: f64) -> Result<f64, String>;

    fn get_op_base(&self) -> &ExpOpBase {
        &self.get_func_base().unitbase
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        &mut self.get_func_base_mut().unitbase
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        let base = self.get_func_base_mut();

        if base._2.is_none() {
            base._2.replace(operand);
            return 1;
        }

        if base._1.is_none() {
            base._1.replace(operand);
            return 0;
        }
        return -1;
    }

    fn execute(&self) -> Result<f64, String> {
        let base = self.get_func_base();

        if base._1.is_none() || base._2.is_none() {
            return Err("Missing operand".to_string());
        }
        
        let res1 = base._1.as_ref().unwrap().execute();
        if res1.is_err() {
            return res1;
        }
        
        let res2 = base._2.as_ref().unwrap().execute();
        if res2.is_err() {
            return res2;
        }

        self.execute_with_args(res1.unwrap(), res2.unwrap())
    }

    fn to_string(&self) -> String {
        let base: &BinaryFunctionBase = self.get_func_base();
        let _1 = &base._1;
        let _2 = &base._2;

        match _1 {
            None => {
                self.exp_name().to_string()
            },
            Some(op_1) => {
                if _2.is_none() {
                    return op_1.to_string() + self.exp_name();
                }
                else {
                    return op_1.to_string() + self.exp_name() + _2.as_ref().unwrap().to_string().borrow();
                }
            }
        }
    }

    fn arg_count(&self) -> i32 {
        2
    }
}

pub trait UnaryFunctionUnit: ExpOpUnit {
    fn get_func_base(&self) -> &UnaryFunctionBase;
    fn get_func_base_mut(&mut self) -> &mut UnaryFunctionBase;
    fn execute_with_args(&self, _1: f64) -> Result<f64, String>;

    fn get_op_base(&self) -> &ExpOpBase {
        &self.get_func_base().unitbase
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        &mut self.get_func_base_mut().unitbase
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        let base = self.get_func_base_mut();

        if base._1.is_none() {
            base._1.replace(operand);
            return 0;
        }
        return -1;
    }

    fn execute(&self) -> Result<f64, String> {
        let base = self.get_func_base();

        if base._1.is_none() {
            return Err("Missing operand".to_string());
        }
        
        let res1 = base._1.as_ref().unwrap().execute();
        if res1.is_err() {
            return res1;
        }

        self.execute_with_args(res1.unwrap())
    }

    fn to_string(&self) -> String {
        let base = self.get_func_base();
        let _1 = &base._1;

        match _1 {
            None => {
                self.exp_name().to_string()
            },
            Some(op_1) => {
                if op_1.exp_name() == EXP_UNIT_NAME_OPEN_BRK {
                    format!("{}{}", self.exp_name(), op_1.to_string())
                }
                else {
                    format!("{}({})", self.exp_name(), op_1.to_string())
                }
            }
        }
    }

    fn arg_count(&self) -> i32 {
        1
    }
}

struct CollectOperator {
    base: UnaryFunctionBase,
}

impl CollectOperator {
    pub fn new() -> Self {
        Self {
            base: UnaryFunctionBase::new(ID_OPEN_BRACKET, 999),
        }
    }    
}

impl UnaryFunctionUnit for CollectOperator {
    fn get_func_base(&self) -> &UnaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut UnaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64) -> Result<f64, String> {
        Ok(_1)
    }
}

impl ExpOpUnit for CollectOperator {
    fn get_op_base(&self) -> &ExpOpBase {
        UnaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        UnaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        UnaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        UnaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = CollectOperator::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExcutableUnit for CollectOperator {
    fn execute(&self) -> Result<f64, String> {
        UnaryFunctionUnit::execute(self)
    }
}

impl ExpUnit for CollectOperator {
    fn to_string(&self) -> String {
        let base = self.get_func_base();
        let _1 = &base._1;

        match _1 {
            None => {
                "(".to_string()
            },
            Some(op_1) => {                
                format!("({})", op_1.to_string())
            }
        }
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_OPEN_BRK
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}

/// sin function
struct SinFunc {
    base: UnaryFunctionBase,
}

impl SinFunc {
    pub fn new() -> Self {
        Self {
            base: UnaryFunctionBase::new(ID_SIN, PRIODITY_UNARY_OP),
        }
    }    
}

impl UnaryFunctionUnit for SinFunc {
    fn get_func_base(&self) -> &UnaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut UnaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64) -> Result<f64, String> {
        Ok(_1.sin())
    }
}

impl ExpOpUnit for SinFunc {
    fn get_op_base(&self) -> &ExpOpBase {
        UnaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        UnaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        UnaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        UnaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = SinFunc::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExcutableUnit for SinFunc {
    fn execute(&self) -> Result<f64, String> {
        UnaryFunctionUnit::execute(self)
    }
}

impl ExpUnit for SinFunc {
    fn to_string(&self) -> String {
        UnaryFunctionUnit::to_string(self)
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_SIN
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}

/// cos function
struct CosFunc {
    base: UnaryFunctionBase,
}

impl CosFunc {
    pub fn new() -> Self {
        Self {
            base: UnaryFunctionBase::new(ID_COS, PRIODITY_UNARY_OP),
        }
    }    
}

impl UnaryFunctionUnit for CosFunc {
    fn get_func_base(&self) -> &UnaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut UnaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64) -> Result<f64, String> {
        Ok(_1.cos())
    }
}

impl ExpOpUnit for CosFunc {
    fn get_op_base(&self) -> &ExpOpBase {
        UnaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        UnaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        UnaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        UnaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = CosFunc::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExcutableUnit for CosFunc {
    fn execute(&self) -> Result<f64, String> {
        UnaryFunctionUnit::execute(self)
    }
}

impl ExpUnit for CosFunc {
    fn to_string(&self) -> String {
        UnaryFunctionUnit::to_string(self)
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_COS
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}

/// tan function
struct TanFunc {
    base: UnaryFunctionBase,
}

impl TanFunc {
    pub fn new() -> Self {
        Self {
            base: UnaryFunctionBase::new(ID_TAN, PRIODITY_UNARY_OP),
        }
    }    
}

impl UnaryFunctionUnit for TanFunc {
    fn get_func_base(&self) -> &UnaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut UnaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64) -> Result<f64, String> {
        Ok(_1.tan())
    }
}

impl ExpOpUnit for TanFunc {
    fn get_op_base(&self) -> &ExpOpBase {
        UnaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        UnaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        UnaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        UnaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = TanFunc::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExcutableUnit for TanFunc {
    fn execute(&self) -> Result<f64, String> {
        UnaryFunctionUnit::execute(self)
    }
}

impl ExpUnit for TanFunc {
    fn to_string(&self) -> String {
        UnaryFunctionUnit::to_string(self)
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_TAN
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}

/// square function
struct SquareFunc {
    base: UnaryFunctionBase,
}

impl SquareFunc {
    pub fn new() -> Self {
        Self {
            base: UnaryFunctionBase::new(ID_SQR, PRIODITY_UNARY_OP),
        }
    }    
}

impl UnaryFunctionUnit for SquareFunc {
    fn get_func_base(&self) -> &UnaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut UnaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64) -> Result<f64, String> {
        Ok(_1 * _1)
    }
}

impl ExpOpUnit for SquareFunc {
    fn get_op_base(&self) -> &ExpOpBase {
        UnaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        UnaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        UnaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        UnaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = SquareFunc::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExcutableUnit for SquareFunc {
    fn execute(&self) -> Result<f64, String> {
        UnaryFunctionUnit::execute(self)
    }
}

impl ExpUnit for SquareFunc {
    fn to_string(&self) -> String {
        let base = self.get_func_base();
        let _1 = &base._1;

        match _1 {
            None => {
                self.exp_name().to_string()
            },
            Some(op_1) => {
                format!("{}{}", op_1.to_string(), self.exp_name())
            }
        }
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_SQR
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}

/// square root function
struct SqrtFunc {
    base: UnaryFunctionBase,
}

impl SqrtFunc {
    pub fn new() -> Self {
        Self {
            base: UnaryFunctionBase::new(ID_SQRT, PRIODITY_UNARY_OP),
        }
    }    
}

impl UnaryFunctionUnit for SqrtFunc {
    fn get_func_base(&self) -> &UnaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut UnaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64) -> Result<f64, String> {
        Ok(_1.sqrt())
    }
}

impl ExpOpUnit for SqrtFunc {
    fn get_op_base(&self) -> &ExpOpBase {
        UnaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        UnaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        UnaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        UnaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = SqrtFunc::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExcutableUnit for SqrtFunc {
    fn execute(&self) -> Result<f64, String> {
        UnaryFunctionUnit::execute(self)
    }
}

impl ExpUnit for SqrtFunc {
    fn to_string(&self) -> String {
        UnaryFunctionUnit::to_string(self)
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_SQRT
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}

/// inverse function
struct InvFunc {
    base: UnaryFunctionBase,
}

impl InvFunc {
    pub fn new() -> Self {
        Self {
            base: UnaryFunctionBase::new(ID_INV, PRIODITY_UNARY_OP),
        }
    }    
}

impl UnaryFunctionUnit for InvFunc {
    fn get_func_base(&self) -> &UnaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut UnaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64) -> Result<f64, String> {
        Ok(1.0 / _1)
    }
}

impl ExpOpUnit for InvFunc {
    fn get_op_base(&self) -> &ExpOpBase {
        UnaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        UnaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        UnaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        UnaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = InvFunc::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExcutableUnit for InvFunc {
    fn execute(&self) -> Result<f64, String> {
        UnaryFunctionUnit::execute(self)
    }
}

impl ExpUnit for InvFunc {
    fn to_string(&self) -> String {
        UnaryFunctionUnit::to_string(self)
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_INV
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}

/// add operator
struct AddOperator {
    base: BinaryFunctionBase,
}

impl AddOperator {
    pub fn new() -> Self {
        Self {
            base: BinaryFunctionBase::new(ID_ADD, PRIODITY_ADDITIVE),
        }
    }    
}

impl BinaryFunctionUnit for AddOperator {
    fn get_func_base(&self) -> &BinaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut BinaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64, _2: f64) -> Result<f64, String> {
        Ok(_1 + _2)
    }
}

impl ExcutableUnit for AddOperator {
    fn execute(&self) -> Result<f64, String> {
        BinaryFunctionUnit::execute(self)
    }
}

impl ExpOpUnit for AddOperator {
    fn get_op_base(&self) -> &ExpOpBase {
        BinaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        BinaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        BinaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        BinaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = AddOperator::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.base._2 = self.base._2.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExpUnit for AddOperator {
    fn to_string(&self) -> String {
        BinaryFunctionUnit::to_string(self)
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_ADD
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}

/// sub operator
struct SubOperator {
    base: BinaryFunctionBase,
}

impl SubOperator {
    pub fn new() -> Self {
        Self {
            base: BinaryFunctionBase::new(ID_SUB, PRIODITY_ADDITIVE),
        }
    }    
}

impl BinaryFunctionUnit for SubOperator {
    fn get_func_base(&self) -> &BinaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut BinaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64, _2: f64) -> Result<f64, String> {
        Ok(_1 - _2)
    }
}

impl ExcutableUnit for SubOperator {
    fn execute(&self) -> Result<f64, String> {
        BinaryFunctionUnit::execute(self)
    }
}

impl ExpOpUnit for SubOperator {
    fn get_op_base(&self) -> &ExpOpBase {
        BinaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        BinaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        BinaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        BinaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = SubOperator::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.base._2 = self.base._2.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExpUnit for SubOperator {
    fn to_string(&self) -> String {
        BinaryFunctionUnit::to_string(self)
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_SUB
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}

/// mul operator
struct MulOperator {
    base: BinaryFunctionBase,
}

impl MulOperator {
    pub fn new() -> Self {
        Self {
            base: BinaryFunctionBase::new(ID_MUL, PRIODITY_MULTIPLICATIVE),
        }
    }    
}

impl BinaryFunctionUnit for MulOperator {
    fn get_func_base(&self) -> &BinaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut BinaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64, _2: f64) -> Result<f64, String> {
        Ok(_1 * _2)
    }
}

impl ExcutableUnit for MulOperator {
    fn execute(&self) -> Result<f64, String> {
        BinaryFunctionUnit::execute(self)
    }
}

impl ExpOpUnit for MulOperator {
    fn get_op_base(&self) -> &ExpOpBase {
        BinaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        BinaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        BinaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        BinaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = MulOperator::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.base._2 = self.base._2.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExpUnit for MulOperator {
    fn to_string(&self) -> String {
        BinaryFunctionUnit::to_string(self)
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_MUL
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}

/// div operator
struct DivOperator {
    base: BinaryFunctionBase,
}

impl DivOperator {
    pub fn new() -> Self {
        Self {
            base: BinaryFunctionBase::new(ID_DIV, PRIODITY_MULTIPLICATIVE),
        }
    }    
}

impl BinaryFunctionUnit for DivOperator {
    fn get_func_base(&self) -> &BinaryFunctionBase {
        &self.base
    }

    fn get_func_base_mut(&mut self) -> &mut BinaryFunctionBase {
        &mut self.base
    }

    fn execute_with_args(&self, _1: f64, _2: f64) -> Result<f64, String> {
        if _2 == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(_1 / _2)
        }
    }
}

impl ExcutableUnit for DivOperator {
    fn execute(&self) -> Result<f64, String> {
        BinaryFunctionUnit::execute(self)
    }
}

impl ExpOpUnit for DivOperator {
    fn get_op_base(&self) -> &ExpOpBase {
        BinaryFunctionUnit::get_op_base(self)
    }

    fn get_op_base_mut(&mut self) -> &mut ExpOpBase {
        BinaryFunctionUnit::get_op_base_mut(self)
    }

    fn push_operand(&mut self, operand: Box<dyn ExcutableUnit>) -> i32 {
        BinaryFunctionUnit::push_operand(self, operand)
    }

    fn arg_count(&self) -> i32 {
        BinaryFunctionUnit::arg_count(self)
    }

    
    fn as_excutable_unit(&mut self) -> Box<dyn ExcutableUnit> {
        let mut new_instance = DivOperator::new();
        new_instance.base._1 = self.base._1.take();
        new_instance.base._2 = self.base._2.take();
        new_instance.set_exp_idx(self.get_exp_idx());
        Box::new(new_instance)
    }
}

impl ExpUnit for DivOperator {
    fn to_string(&self) -> String {
        BinaryFunctionUnit::to_string(self)
    }

    fn exp_name(&self) -> &str {
        EXP_UNIT_NAME_DIV
    }

    fn get_exp_unit_base(&self) -> &ExpUnitBase {
        &self.base.unitbase.unitbase
    }

    fn get_exp_unit_base_mut(&mut self) -> &mut ExpUnitBase {
        &mut self.base.unitbase.unitbase
    }
}


pub struct ExpressionBuilder {
    token_count: i32,
    last_op_count: i32,
    operand_stack: Vec<Box<dyn ExcutableUnit>>,
    operator_stack: Vec<Box<dyn ExpOpUnit>>,
}

impl ExpressionBuilder {
    pub fn new() -> Self {
        Self {
            token_count: 0,
            last_op_count: -2,
            operator_stack: Vec::new(),
            operand_stack: Vec::new(),            
        }
    }

    fn top_op(&self) -> Option<& Box<dyn ExpOpUnit>> {
        self.operator_stack.last()
    }

    fn push_op(&mut self, op: Box<dyn ExpOpUnit>) {
        self.operator_stack.push(op);
    }

    fn build_top_op_tree(&mut self, lower_bound_idx: i32) -> Result<Option<String>, String> {
        let mut op = self.operator_stack.pop().unwrap();        
        let mut args = op.arg_count();
        while args > 0 {
            if self.operand_stack.len() == 0 {
                return Err("Invalid expression".to_string());
            }            
            let operand = self.operand_stack.pop().unwrap();
            if lower_bound_idx >= 0 && operand.get_exp_idx() <= lower_bound_idx {
                return Err("Invalid expression".to_string());
            }

            op.push_operand(operand);

            args -= 1;
        }

        let imediate_result = op.execute();

        // issue: https://github.com/rust-lang/rust/issues/65991
        // self.operand_stack.push(op);
        // use ExpOpUnit::as_excutable_unit to overcome the issue
        self.operand_stack.push(op.as_excutable_unit());

        return imediate_result.map(|v| Some(v.to_string()));
    }

    pub fn build_tree_inside_bracket(&mut self) -> Result<Option<String>, String> {
        let x = self.operator_stack.iter().find(|op| op.get_op_base().id == ID_OPEN_BRACKET);
        let mut lower_bound_idx = if x.is_none() { -1 } else { x.unwrap().get_exp_idx() };

        while self.operator_stack.len() > 0 {
            let id = self.top_op().unwrap().get_op_base().id;
            let x = self.build_top_op_tree(lower_bound_idx);
            if x.is_err() {
                return x;
            }
            if ID_OPEN_BRACKET == id {
                return x;
            }
        }
        Err("Missing open bracket".to_string())
    }
    

    pub fn push_functor(&mut self, name: String, prefer_eval: bool) -> Result<Option<String>, String> {
        self.token_count += 1;

        if name == EXP_UNIT_NAME_CLOSE_BRK { // close bracket
            self.last_op_count = 1;
            return self.build_tree_inside_bracket();
        }

        let op_opt = EXP_OP_LIB.get_functor(&name);
        if op_opt.is_none() {
            self.last_op_count = -1;
            return Err("No functor found".to_string());
        }
        let mut op = op_opt.unwrap();        
        op.set_exp_idx(self.token_count);
        self.last_op_count = op.arg_count();
        let op_base = op.get_op_base();

        // if the operator is a unary operator, and the caller want to evaluate it now                
        if prefer_eval && op.arg_count() == 1 && op_base.id != ID_OPEN_BRACKET {
            self.push_op(op);
            return self.build_top_op_tree(-1);
        }

        let top_op = self.top_op();
        match top_op {
            Some(top) => {
                let top_base = top.get_op_base();                

                if op_base.id == ID_OPEN_BRACKET {
                    self.push_op(op);
                    return Ok(None);
                }

                if top_base.precedence <= op_base.precedence {
                    let x = self.build_top_op_tree(-1);
                    self.push_op(op);            
                    return x;
                }
                self.push_op(op);
                return Ok(None);
            },
            None => {
                self.operator_stack.push(op);
                Ok(None)
            }
            
        }
    }

    pub fn push_operand(&mut self, token: String) -> bool {
        self.token_count += 1;
        self.last_op_count = 1;

        let res = token.parse::<f64>();
        match res {
            Err(_) => false,
            Ok(value) => {
                let mut operand = Box::new(ConstantUnit::new(value));
                operand.set_exp_idx(self.token_count);
                self.operand_stack.push(operand);
                true
            }
        }
    }

    pub fn to_exp_string(&self) -> String {
        let mut exp_str = String::new();
        let mut exp_indices: Vec<(bool, usize)> = Vec::with_capacity(self.operand_stack.len() + self.operator_stack.len());

        for i in 0..self.operand_stack.len() {
            exp_indices.push((true, i));
        }
        
        for i in 0..self.operator_stack.len() {
            exp_indices.push((false, i));
        }

        exp_indices.sort_by(|a, b| {
            let a_idx = a.1;
            let b_idx = b.1;
            let a_exp_idx = if a.0 { self.operand_stack[a_idx].get_exp_idx() } else { self.operator_stack[a_idx].get_exp_idx() };
            let b_exp_idx = if b.0 { self.operand_stack[b_idx].get_exp_idx() } else { self.operator_stack[b_idx].get_exp_idx() };
            a_exp_idx.cmp(&b_exp_idx)
        });

        for (is_operand, idx) in exp_indices {
            if is_operand {
                exp_str += &self.operand_stack[idx].to_string();
            }
            else {
                exp_str += &self.operator_stack[idx].to_string();
            }
        }

        exp_str
    }

    pub fn finish(&mut self) -> Result<Expression, String> {
        while self.operator_stack.len() > 0 {
            let x = self.build_top_op_tree(-1);
            if x.is_err() {
                return Err(x.err().unwrap());
            }
        }
        if self.operand_stack.len() != 1 {
            return Err("Invalid expression".to_string());
        }
        Ok(Expression {
            root: self.operand_stack.pop(),
        })
    }

    pub fn eval_immediate(&mut self) -> Result<f64, String> {
        match self.operand_stack.last() {
            Some(op) => op.execute(),
            None => Err("Incomplete expression".to_string())            
        }
    }

    pub fn get_last_exp_token_arg_count(&self) -> i32 {
        // work around
        self.last_op_count
    }

    pub fn tokenize(input: String) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut token = String::new();
        for c in input.chars() {
            if c.is_alphanumeric() || c == '.' {
                token.push(c);
            }
            else {
                if !token.is_empty() {
                    tokens.push(token);
                    token = String::new();
                }
                tokens.push(c.to_string());
            }
        }
        if !token.is_empty() {
            tokens.push(token.clone());
        }
        tokens
    }

    pub fn is_decimal(s : &str) -> bool {
        if s.is_empty() {
            return false;
        }    
        let mut iterator = s.chars();
        let mut oc = iterator.next();
        let mut c = oc.unwrap();    
        if c == '-' || c == '+' {
            if s.len() == 1 {
                return false;
            }
            oc = iterator.next();
        }
        let mut i = 0;
        let mut has_dot = false;
        while oc.is_some() {
            c = oc.unwrap();
            if c == '.' {
                if has_dot { // dot should has only one
                    return false;
                }
                has_dot = true;
                if i == 0 { // dot is not allow to be first character
                    return false;
                }
            }
            else if !c.is_ascii_digit() {
                return false;
            }
            
            oc = iterator.next();
            i = i + 1;
        }
    
        true
    }    
}



type ExpOpCreator = fn(&String) -> Box<dyn ExpOpUnit>;

struct ExpOpLib {
    op_creator_map: HashMap<String, ExpOpCreator>,
}

impl ExpOpLib {
    pub fn new() -> Self {
        let mut op_creator_map: HashMap<String, ExpOpCreator> = HashMap::new();
        op_creator_map.insert(EXP_UNIT_NAME_ADD.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(AddOperator::new()) });
        op_creator_map.insert(EXP_UNIT_NAME_SUB.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(SubOperator::new()) });
        op_creator_map.insert(EXP_UNIT_NAME_MUL.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(MulOperator::new()) });
        op_creator_map.insert(EXP_UNIT_NAME_DIV.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(DivOperator::new()) });
        op_creator_map.insert(EXP_UNIT_NAME_SIN.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(SinFunc::new()) });
        op_creator_map.insert(EXP_UNIT_NAME_COS.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(CosFunc::new()) });
        op_creator_map.insert(EXP_UNIT_NAME_TAN.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(TanFunc::new()) });
        op_creator_map.insert(EXP_UNIT_NAME_INV.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(InvFunc::new()) });
        op_creator_map.insert(EXP_UNIT_NAME_SQR.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(SquareFunc::new()) });
        op_creator_map.insert(EXP_UNIT_NAME_SQRT.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(SqrtFunc::new()) });
        op_creator_map.insert(EXP_UNIT_NAME_OPEN_BRK.to_string(), |_: &String| -> Box<dyn ExpOpUnit> { Box::new(CollectOperator::new()) });
        
        Self {
            op_creator_map
        }        
    }

    pub fn get_functor(&self, name: &String) -> Option<Box<dyn ExpOpUnit>> {
        self.op_creator_map.get(name).map(|op_creator| {
            op_creator(name)
        })
    }
}

lazy_static! {
    static ref EXP_OP_LIB: ExpOpLib = ExpOpLib::new();
}