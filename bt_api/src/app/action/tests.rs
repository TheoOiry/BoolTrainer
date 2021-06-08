use diesel::{Connection, PgConnection};

use crate::app::action::round::generator::RoundGenerator;
use crate::app::model::game::Game;
use crate::app::model::session::Session;
use crate::connection;

#[test]
fn round_generator() {
    dotenv::dotenv().ok();
    let conn = PgConnection::establish(&connection::database_url()).unwrap();

    let session = Session::insert(&conn).unwrap();
    let game = Game::insert(&session, &conn).unwrap();
    let generator = RoundGenerator::new_generated(&game, &conn);

    println!("{}", generator.get_round_json().to_string());
}
