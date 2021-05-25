use rocket_contrib::json::{Json, JsonValue};
use crate::connection::DbConn;

#[get("/create_session")]
pub fn create_session(_conn: DbConn) -> JsonValue {
    json!({

    })
}