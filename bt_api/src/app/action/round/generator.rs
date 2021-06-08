use std::collections::HashMap;

use diesel::PgConnection;
use rocket_contrib::json::JsonValue;

use crate::app::core::bool_expression::generator::ExpressionGenerator;
use crate::app::core::bool_expression::BoolExpression;
use crate::app::model::game::Game;
use crate::app::model::item::Item;
use crate::app::model::round::Round;
use crate::app::model::variable::Variable;

pub struct RoundGenerator<'a> {
    round: Round,
    bool_expression: BoolExpression,
    items: Vec<Item>,
    variables: HashMap<Item, Vec<Variable>>,
    connection: &'a PgConnection,
}

impl<'a> RoundGenerator<'a> {
    pub fn new_generated(game: &Game, conn: &'a PgConnection) -> RoundGenerator<'a> {
        let mut round_generator = Self::new(game, conn);
        round_generator.fill_items();

        while round_generator.items.iter().all(|item| item.get_expected())
            || round_generator.items.iter().all(|item| !item.get_expected())
        {
            round_generator.round.delete(conn).ok();
            round_generator = Self::new(game, conn);
            round_generator.fill_items();
        }

        round_generator
    }

    fn new(game: &Game, conn: &'a PgConnection) -> RoundGenerator<'a> {
        let bool_expression = ExpressionGenerator::build();
        RoundGenerator {
            round: Round::insert(&game, bool_expression.to_string(), conn).unwrap(),
            bool_expression,
            items: vec![],
            variables: HashMap::new(),
            connection: conn,
        }
    }

    fn fill_items(&mut self) {
        for _ in 0..5 {
            self.bool_expression.randomize_variables();
            let new_item = Item::insert(
                &self.round,
                self.bool_expression.get_result(),
                self.connection,
            )
            .unwrap();
            self.fill_variables(new_item);
            self.items.push(new_item);
        }
    }

    fn fill_variables(&mut self, item: Item) {
        self.variables
            .insert(item, self.build_current_variables_model_state(&item));
    }

    fn build_current_variables_model_state(&self, item: &Item) -> Vec<Variable> {
        self.bool_expression
            .get_current_variables_state()
            .iter()
            .map(|option_variable| {
                Variable::insert(&item, option_variable, self.connection).unwrap()
            })
            .collect::<Vec<Variable>>()
    }

    pub fn get_round_json(&self) -> JsonValue {
        json!({
            "round_id": self.round.get_id(),
            "expression": self.round.get_expression(),
            "items": self.items.iter().map(|item| item.json_hidden_expected(&self.variables[item])).collect::<Vec<JsonValue>>()
        })
    }
}
