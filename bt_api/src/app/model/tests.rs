use diesel::pg::PgConnection;
use diesel::Connection;

use super::session::Session;
use crate::connection;

#[test]
fn it_creates_a_session() {
    dotenv::dotenv().ok();
    let conn = PgConnection::establish(&connection::database_url()).unwrap();
    assert!(Session::get(Session::insert(&conn).unwrap().get_id(), &conn).is_ok())
}
