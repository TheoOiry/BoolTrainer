use rocket;

use crate::app::handler;
use crate::connection;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_cors::{AllowedOrigins, CorsOptions};

pub fn get_rocket() -> Rocket {
    rocket::ignite()
        .manage(connection::init_pool())
        .attach(AdHoc::on_attach(
            "Database Migrations",
            connection::run_db_migrations,
        ))
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .unwrap(),
        )
        .mount(
            "/api",
            routes![
                handler::create_session,
                handler::create_game,
                handler::answer_round,
                handler::ping_session,
            ],
        )
}
