pub mod expression_option;
pub mod generator;
#[cfg(test)]
mod tests;

use expression_option::{BoolOption, Variable, VariableCell};
use rand::{thread_rng, Rng};
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};

pub struct BoolExpression {
    variables: Vec<VariableCell<i32>>,
    expression_start: BoolOption,
}

impl BoolExpression {
    pub fn randomize_variables(&mut self) {
        for var in self.variables.iter() {
            var.borrow_mut().set_value(thread_rng().gen_range(0..=5))
        }
    }

    pub fn get_current_variables_state(&self) -> Vec<Variable<i32>> {
        self.variables
            .iter()
            .map(|variable_cell| {
                let variable: &RefCell<Variable<i32>> = variable_cell.borrow();
                variable.borrow().clone()
            })
            .collect()
    }

    pub fn get_result(&self) -> bool {
        self.expression_start.get_result()
    }
}

impl ToString for BoolExpression {
    fn to_string(&self) -> String {
        self.expression_start.to_string()
    }
}
