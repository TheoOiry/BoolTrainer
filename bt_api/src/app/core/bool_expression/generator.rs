use super::expression_option::{BoolOption, OrdOption, Value, Variable, VariableCell};
use super::BoolExpression;
use rand;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ExpressionGenerator {
    potential_variables: Vec<VariableCell<i32>>,
    used_variables: RefCell<Vec<VariableCell<i32>>>,
}

impl ExpressionGenerator {
    pub fn build() -> BoolExpression {
        let mut generator = ExpressionGenerator {
            potential_variables: vec![],
            used_variables: RefCell::new(vec![]),
        };

        generator.fill_potential_variables();
        let generated_expression = generator.build_expression();

        BoolExpression {
            variables: generator.used_variables.take(),
            expression_start: generated_expression,
        }
    }

    fn fill_potential_variables(&mut self) {
        self.potential_variables = (0..thread_rng().gen_range(2..4))
            .map(|index| Variable::new(Self::get_alphabet_letter_from(index).to_string(), 0))
            .map(|variable| Rc::new(RefCell::new(variable)))
            .collect();
    }

    fn get_alphabet_letter_from(index: usize) -> char {
        ('a'..='z').take(index + 1).last().unwrap()
    }

    fn build_expression(&mut self) -> BoolOption {
        if rand::random() {
            BoolOption::And(self.build_options_col(0))
        } else {
            BoolOption::Or(self.build_options_col(0))
        }
    }

    fn build_options_col(&self, depth: i32) -> Vec<BoolOption> {
        (0..thread_rng().gen_range(2..4))
            .map(|_| self.build_bool_expression_option(depth + 1))
            .collect()
    }

    fn build_bool_expression_option(&self, depth: i32) -> BoolOption {
        match ((thread_rng().gen_range(0..20)) as i32) - (depth * 3) {
            i if i < 7 => BoolOption::OrdExpr(Box::new(self.build_ord_expression_option())),
            i if i < 8 => BoolOption::Const(rand::random()),
            i if i < 12 => BoolOption::Or(self.build_options_col(depth + 1)),
            i if i < 16 => BoolOption::And(self.build_options_col(depth + 1)),
            _ => BoolOption::Not(Box::new(self.build_bool_expression_option(depth + 1))),
        }
    }

    fn build_ord_expression_option(&self) -> OrdOption<i32> {
        match thread_rng().gen_range(0..=5) {
            0 => OrdOption::Greater(self.build_value(), self.build_value()),
            1 => OrdOption::GreaterEqual(self.build_value(), self.build_value()),
            2 => OrdOption::Less(self.build_value(), self.build_value()),
            3 => OrdOption::LessEqual(self.build_value(), self.build_value()),
            4 => OrdOption::Equal(self.build_value(), self.build_value()),
            _ => OrdOption::NotEqual(self.build_value(), self.build_value()),
        }
    }

    fn build_value(&self) -> Value<i32> {
        match thread_rng().gen_range(0..4) {
            0 => Value::Const(thread_rng().gen_range(0..5)),
            _ => {
                let variable = self.potential_variables.choose(&mut thread_rng()).unwrap();
                self.add_variable_to_used(&variable);
                Value::Compute(Rc::clone(variable))
            }
        }
    }

    fn add_variable_to_used(&self, variable: &VariableCell<i32>) {
        if !self
            .used_variables
            .borrow()
            .iter()
            .any(|var| {
                let var1: &RefCell<Variable<i32>> = var.borrow();
                let var2: &RefCell<Variable<i32>> = variable.borrow();
                var1.borrow().get_name() == var2.borrow().get_name()
            })
        {
            self.used_variables.borrow_mut().push(Rc::clone(variable));
        }
    }
}
