use std::collections::HashMap;
use std::f32::consts::E;
use super::functions::*;

use super::ExpressionBuilder;

pub struct Calculator {
    evaluator: ExpressionBuilder,
    constants_map: HashMap<String, String>,
    operand_token: String,
    last_result: String,
    last_immediate: String,
    cached_history: String,
    input_tokens: Vec<String>,
    memory: Option<String>,
    is_operand_last: bool,
    need_sync_tokens: bool,
}
pub enum Feature {
    CE,
    C,
    MS,
    MR,
    DEL,
    Eval,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            evaluator: ExpressionBuilder::new(),
            constants_map: HashMap::new(),
            operand_token: String::new(),
            input_tokens: Vec::new(),
            last_result: "0".to_string(),
            cached_history: String::new(),
            last_immediate: String::new(),
            memory: None,
            is_operand_last: false,
            need_sync_tokens: false,
        }
    }

    fn expression_operand_input(&mut self, c: &char) -> Result<Option<String>, String> {       
        if !self.last_result.is_empty() {
            // clear last result if user input first operand of the expression
            self.last_result.clear();
        }
        self.operand_token.push(*c);
        Ok(Some(self.operand_token.clone()))
    }

    fn expression_constant_input(&mut self, const_val: &String) -> Result<Option<String>, String> {
        // just clear the temporary input if user pick another constant
        self.operand_token.clear();
        // clear last result we don't need it anymore
        self.last_result.clear();

        self.operand_token = const_val.clone();
        Ok(Some(self.operand_token.clone()))
    }

    fn put_functor(&mut self, token: String) -> Result<Option<String>, String> {
        let res = self.evaluator.push_functor(token, self.is_operand_last);
        self.is_operand_last = match res {
            Ok(Some(_)) => {
                self.need_sync_tokens = true;
                self.evaluator.get_last_exp_token_arg_count() == 1
            },
            _ => false
        };
        res
    }

    fn put_token(&mut self, token: String) -> Result<Option<String>, String> {
        if ExpressionBuilder::is_decimal(token.as_str()) {
            self.evaluator.push_operand (token.clone());
            self.is_operand_last = true;
            Ok(Some(token))            
        }
        else {            
            self.put_functor(token)
        }
    }

    fn push_temp_input(&mut self) -> Option<String> {
        let mut put_str : Option<String> = None;
        if !self.last_result.is_empty() {
            self.operand_token = self.last_result.clone();
            self.last_result.clear();
        }
        if !self.operand_token.is_empty() {
            let _ = self.put_token(self.operand_token.clone());
            put_str.replace(self.operand_token.clone());

            if self.need_sync_tokens {
                self.input_tokens = ExpressionBuilder::tokenize(self.evaluator.to_exp_string());
                self.need_sync_tokens = false;
            }
            else {
                self.input_tokens.push(self.operand_token.clone());
            }
            
            self.operand_token.clear();
        }
        put_str
    }

    fn expression_op_input(&mut self, op_name: &String) -> Result<Option<String>, String> {
        let _ = self.push_temp_input();
        let res = self.put_functor(op_name.clone());
        if self.need_sync_tokens {
            self.input_tokens = ExpressionBuilder::tokenize(self.evaluator.to_exp_string());
            self.need_sync_tokens = false;
        }
        else {
            self.input_tokens.push(op_name.clone());
        }
        return res;
    }

    pub fn build_history(&self) -> String {
        if self.cached_history.is_empty() {
            let mut history = self.evaluator.to_exp_string();
            history.push_str(&self.operand_token);
            history
        }
        else {
            self.cached_history.clone()
        }        
    }

    pub fn perform_exp_input(&mut self, input: String) -> Result<Option<String>, String> {
        if input.is_empty() {
            return Err("Empty input".to_string());
        }
        self.cached_history.clear();

        let immediate_result: Result<Option<String>, String>;

        loop {
            if input.len() == 1 {
                let c: char = input.chars().next().unwrap();
                if c.is_ascii_digit() || c == '.' {
                    immediate_result = self.expression_operand_input(&c);
                    break;
                }
            }
            let constant = self.constants_map.get(&input);
            match constant {
                Some(value) => {
                    immediate_result = self.expression_constant_input(&value.clone());
                }
                None => {
                    immediate_result = self.expression_op_input(&input);
                }
            }
            break;
        }

        match immediate_result.clone() {
            Ok(Some(v)) => {
                self.last_immediate = v;
            },
            _ => {}
        };
        
        immediate_result
    }

    pub fn perform_feature(&mut self, feature: &Feature) -> Result<Option<String>, String> {
        match feature {
            Feature::CE => self.reset_temp(),
            Feature::C => self.reset(),
            Feature::MS => self.memory_store(),
            Feature::MR => self.memory_recover(),
            Feature::Eval => self.eval(),
            Feature::DEL => self.delete_input(),
        }
    }

    fn eval_error(&mut self, temp_token_updated: bool, err: String) -> Result<Option<String>, String> {
         // reset the evaluator due to it may damaged by evaluation
         self.evaluator = ExpressionBuilder::new();

         // recover evaluator to state before evaluation
         if temp_token_updated {
             self.operand_token = self.input_tokens.pop().unwrap();
         }

         let x = self.input_tokens.clone();
         x.iter().for_each(|t| {
             let _ = self.put_token(t.clone());
         });         
         // return none like nothing happened
         Ok(None)
    }

    fn eval(&mut self) -> Result<Option<String>, String> {
        let mut temp_token_updated = false;
        if !self.operand_token.is_empty() {
            let _ = self.evaluator.push_operand(self.operand_token.clone());
            self.input_tokens.push(self.operand_token.clone());
            self.operand_token.clear();
            temp_token_updated = true;
        }
        let res = self.evaluator.finish();
        match res {
            Ok(e) => {
                // store the final result so that it can be used as the begin of next expression
                let vr = e.execute();
                match vr {
                    Ok(v) => {
                        self.last_result = v.to_string();
                        self.last_immediate = self.last_result.clone();
                        // reset the evaluator after evaluation
                        self.evaluator = ExpressionBuilder::new();
                        self.cached_history = e.to_string() + " =";
                        self.operand_token.clear();
                        self.input_tokens.clear();

                        // return the result in String
                        Ok(Some(self.last_result.clone()))
                    },
                    Err(s) => {
                        self.eval_error(temp_token_updated, s)
                    }
                }
            },
            Err(s) => {
                self.eval_error(temp_token_updated, s)
            }
        }
    }

    fn recaculate_after_delete(&mut self) -> Result<Option<String>, String> {
        // reset the evaluator due to its state is one step forward
        self.evaluator = ExpressionBuilder::new();

        // recover evaluator to current state of inputs
        let mut results = Vec::new();

        let x = self.input_tokens.clone();
        for token in x  {
            let last_res = self.put_token(token);
            results.push(last_res);
        }

        if self.operand_token.is_empty() {
            let mut last_val = String::new();
            let i_opt = results.iter().rev().position(|r| {
                match r {
                    Ok(Some(v)) => {
                        last_val = v.clone();
                        true
                    },
                    _ => false
                }
            });
            
            match i_opt {
                Some(_) => {
                    Ok(Some(last_val))
                },
                None => Ok(Some("0".to_string()))
            }
        }
        else {
            // if temporary input is not empty then return the new temporary input
            Ok(Some(self.operand_token.clone()))
        }
    }

    fn delete_input(&mut self) -> Result<Option<String>, String> {

        // try to delete one last char in temporary input...
        match self.operand_token.pop() {
            Some(_) => {
                // ...if it's possible then return the new temporary input
                if self.operand_token.is_empty() {
                    self.recaculate_after_delete()
                }
                else {
                    Ok(Some(self.operand_token.clone()))
                }                
            },
            None => {
                // ...if it's not possible then take input tokens to exmaine
                if self.input_tokens.is_empty() {
                    return Ok(None);
                }
                // take the last token from input tokens
                self.input_tokens.pop();
                
                self.recaculate_after_delete()
            }            
        }
    }

    pub fn reset(&mut self) -> Result<Option<String>, String> {
        self.last_result = "0".to_string();
        self.last_immediate = "0".to_string();
        self.operand_token.clear();
        self.input_tokens.clear();
        self.evaluator = ExpressionBuilder::new();
        self.cached_history.clear();
        self.is_operand_last = false;
        self.need_sync_tokens = false;

        Ok(Some(self.last_result.clone()))
    }

    fn reset_temp(&mut self) -> Result<Option<String>, String> {
        self.operand_token.clear();
        self.last_result.clear();

        self.evaluator.eval_immediate().map(|v| Some(v.to_string()))
    }

    fn memory_store(&mut self) -> Result<Option<String>, String> {
        if self.last_immediate.is_empty() {
            return Ok(None);
        }

        if ExpressionBuilder::is_decimal(self.last_immediate.as_str()) {
            self.memory.replace(self.last_immediate.clone());
        }
        Ok(None)
    }

    fn memory_recover(&mut self) -> Result<Option<String>, String> {
        match self.memory.clone() {
            Some(v) => {
                self.operand_token = v.clone();
                Ok(Some(v))
            },
            None => Ok(None)
        }
    }

    pub fn add_constant(&mut self, name: String, value: String) {
        self.constants_map.insert(name, value);
    }
}