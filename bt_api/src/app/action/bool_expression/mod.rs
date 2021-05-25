#[cfg(test)] mod tests;
mod expression;
mod generator;

use expression::{VariableCell, BoolOption, Variable};
use rand::Rng;
use serde::de::Unexpected::Bool;
use std::rc::Rc;
use std::cell::RefCell;

struct BoolExpression {
    variables: Vec<VariableCell<u32>>,
    expression_start: BoolOption
}

impl BoolExpression {
    pub fn new() {

    }

    fn randomize_variables(&self) {

    }

    fn get_result(&self) -> bool {
        self.expression_start.get_result()
    }
}