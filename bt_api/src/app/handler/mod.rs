use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::Value;

use crate::app::action::round::answered::RoundAnswer;
use crate::app::action::round::generator::RoundGenerator;
use crate::app::action::session::SessionClaims;
use crate::app::helper::api_response::ApiResponse;
use crate::app::model::game::Game;
use crate::app::model::session::Session;
use crate::connection::DbConn;

#[post("/create_session")]
pub fn create_session(conn: DbConn) -> Result<ApiResponse, Status> {
    let jwt_token = Session::insert(&conn)
        .map(|session| SessionClaims::new(&session).get_jwt_token())
        .map_err(error_status)?;

    Ok(ApiResponse::new(
        json!({ "jwt_token": jwt_token }),
        Status::Created,
    ))
}

#[post("/create_game")]
pub fn create_game(conn: DbConn, session: Session) -> Result<JsonValue, Status> {
    let game = Game::insert(&session, &conn).map_err(error_status)?;
    let generator = RoundGenerator::new_generated(&game, &conn);

    Ok(json!({
        "game_id": game.get_id(),
        "first_round": generator.get_round_json()
    }))
}

#[post("/answer_round", format = "json", data = "<answer>")]
pub fn answer_round(
    conn: DbConn,
    _session: Session,
    mut answer: Json<Value>,
) -> Result<JsonValue, Status> {
    RoundAnswer::answer(&conn, answer.take())
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
