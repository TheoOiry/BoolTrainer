use std::str::FromStr;

use diesel::PgConnection;
use rocket::http::Status;
use rocket_contrib::json::JsonValue;
use serde_json::Value;

use crate::app::action::round::generator::RoundGenerator;
use crate::app::action::round::results::RoundResult;
use crate::app::model::game::Game;
use crate::app::model::item::Item;

#[derive(Deserialize, Serialize)]
pub struct ItemAnswer {
    item_id: uuid::Uuid,
    answer: bool,
}

pub struct RoundAnswer<'a> {
    connection: &'a PgConnection,
    json_request: Value,
}

impl<'a> RoundAnswer<'a> {
    pub fn answer(connection: &'a PgConnection, json_request: Value) -> Result<JsonValue, Status> {
        let round_answer = RoundAnswer {
            connection,
            json_request,
        };
        round_answer.apply_answer()?;

        round_answer.generate_response()
    }

    fn apply_answer(&self) -> Result<(), Status> {
        for item_answer in self.get_items_answer()? {
            let item =
                Item::get(item_answer.item_id, self.connection).map_err(|_| Status::BadRequest)?;

            item.set_answer(item_answer.answer, self.connection);
        }

        Ok(())
    }

    fn get_items_answer(&self) -> Result<Vec<ItemAnswer>, Status> {
        let answers: Value = self
            .json_request
            .get("answers")
            .ok_or(Status::BadRequest)?
            .clone();

        serde_json::from_value(answers).map_err(|_| Status::BadRequest)
    }

    fn generate_response(&self) -> Result<JsonValue, Status> {
        let game = self.get_game()?;
        game.set_last_round_time_now(self.connection);

        if game.is_end(self.connection) {
            Ok(RoundResult::get_results(&game, self.connection))
        } else {
            let generator = RoundGenerator::new_generated(&game, self.connection);
            Ok(json!({
                "next_round": generator.get_round_json()
            }))
        }
    }

    fn get_game(&self) -> Result<Game, Status> {
        Game::get(self.get_game_id()?, self.connection).map_err(|_| Status::BadRequest)
    }

    fn get_game_id(&self) -> Result<uuid::Uuid, Status> {
        let game_id = self
            .json_request
            .get("game_id")
            .ok_or(Status::BadRequest)?
            .as_str()
            .ok_or(Status::BadRequest)?;

        uuid::Uuid::from_str(game_id).map_err(|_| Status::BadRequest)
    }
}
