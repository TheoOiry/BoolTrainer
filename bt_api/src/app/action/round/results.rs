use std::collections::HashMap;

use diesel::PgConnection;
use diesel::{BelongingToDsl, RunQueryDsl};
use rocket_contrib::json::JsonValue;

use crate::app::model::game::Game;
use crate::app::model::item::Item;
use crate::app::model::round::Round;
use crate::app::model::variable::Variable;

pub struct RoundResult<'a> {
    connection: &'a PgConnection,
    game: &'a Game,
    rounds: Vec<Round>,
    items: HashMap<Round, Vec<Item>>,
    variables: HashMap<Item, Vec<Variable>>,
}

impl<'a> RoundResult<'a> {
    pub fn get_results(game: &Game, connection: &PgConnection) -> JsonValue {
        let mut round_result = RoundResult {
            connection,
            game,
            rounds: vec![],
            items: HashMap::new(),
            variables: HashMap::new(),
        };

        round_result.fill_rounds();
        round_result.fill_items();
        round_result.fill_variables();

        round_result.build_results()
    }

    fn fill_rounds(&mut self) {
        self.rounds = self.game.get_rounds(self.connection).unwrap();
        self.rounds.sort_by(|r1, r2| {
            r1.get_time_end()
                .unwrap()
                .partial_cmp(&r2.get_time_end().unwrap())
                .unwrap()
        });
    }

    fn fill_items(&mut self) {
        let items: Vec<Item> = Item::belonging_to(&self.rounds)
            .load::<Item>(self.connection)
            .unwrap();
        for item in items {
            let round = self
                .rounds
                .iter()
                .find(|round| round.get_id() == item.get_round_id())
                .unwrap();
            self.items
                .entry(round.clone())
                .or_insert_with(Vec::new)
                .push(item);
        }
    }

    fn fill_variables(&mut self) {
        let all_items = self.get_all_items();
        let variables: Vec<Variable> = Variable::belonging_to(&all_items)
            .load::<Variable>(self.connection)
            .unwrap();
        for variable in variables {
            let item = all_items
                .iter()
                .find(|item| item.get_id() == variable.get_item_id())
                .unwrap();
            self.variables
                .entry(*item)
                .or_insert_with(Vec::new)
                .push(variable);
        }
    }

    fn get_all_items(&self) -> Vec<Item> {
        self.items.values().flatten().cloned().collect()
    }

    fn build_results(&mut self) -> JsonValue {
        json!({
            "game_id": self.game.get_id(),
            "time_start": self.game.get_time_start().to_string(),
            "score": self.build_scores_json(),
            "rounds": self.rounds.iter().map(|round| self.build_round_json(round)).collect::<Vec<JsonValue>>(),
        })
    }

    fn build_scores_json(&self) -> JsonValue {
        let time_spend = self.get_time_spend();
        let (nb_errors, nb_items) = self.get_nb_errors_items();

        json!({
            "time_spend": time_spend,
            "nb_errors": nb_errors,
            "nb_items": nb_items
        })
    }

    fn get_time_spend(&self) -> i64 {
        let end_time = self
            .game
            .get_last_round(self.connection)
            .unwrap()
            .get_time_end()
            .unwrap();
        let start_time = self.game.get_time_start();
        end_time.signed_duration_since(start_time).num_seconds()
    }

    fn get_nb_errors_items(&self) -> (usize, usize) {
        let all_items = self.get_all_items();
        let nb_errors = all_items
            .iter()
            .filter(|item| {
                item.get_found().is_none() || item.get_found().unwrap() != item.get_expected()
            })
            .count();
        let nb_item = all_items.len();

        (nb_errors, nb_item)
    }

    fn build_round_json(&self, round: &Round) -> JsonValue {
        json!({
            "round_id": round.get_id(),
            "time_end": round.get_time_end().unwrap().to_string(),
            "expression": round.get_expression(),
            "items": self.items[round].iter().map(|item| self.build_item_json(item)).collect::<Vec<JsonValue>>(),
        })
    }

    fn build_item_json(&self, item: &Item) -> JsonValue {
        json!({
            "item_id": item.get_id(),
            "variables": self.variables[item].iter().map(|variable| variable.to_json()).collect::<Vec<JsonValue>>(),
            "expected": item.get_expected(),
            "answer": item.get_found()
        })
    }
}
