use rocket_contrib::json::{Json, JsonValue};
use crate::connection::DbConn;

pub mod handler;
mod helper;

#[get("/create_session")]
pub fn create_session(conn: DbConn) -> JsonValue {
    json!({

    })
}