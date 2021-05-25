use super::expression::{ VariableCell, BoolOption, Variable };
use super::BoolExpression;
use std::cell::RefCell;
use std::rc::Rc;
use rand;
use rand::{thread_rng, Rng};




pub struct ExpressionGenerator {
    variables: Vec<VariableCell<u32>>,
    expression_start: Option<BoolOption>,
}

impl ExpressionGenerator {
    fn build() -> BoolExpression {
        todo!()
    }

    fn generate_variables(&mut self) {
        self.variables = (0..rand::thread_rng().gen_range(2..4))
            .map(|index| Variable::new(('a'..'z').take(index).last().unwrap().to_string(), thread_rng().gen_range(-100..100)))
            .map(|variable| Rc::new(RefCell::new(variable)))
            .collect();
    }

    fn generate_expression(&mut self) {
        if rand::random() {
            self.expression_start = Some(BoolOption::And(self.generate_option_col()))
        } else {
            self.expression_start = Some(BoolOption::Or(self.generate_option_col()))
        }
    }

    fn generate_option_col(&self) -> Vec<Box<BoolOption>> {
        (0..thread_rng().gen_range(2..4))
            .map(|index| Box::new(self.generate_expression_option(thread_rng().gen_range(0..2))))
            .collect()
    }

    fn generate_expression_option(&self, depth: usize) -> BoolOption {
        todo!()
    }
}