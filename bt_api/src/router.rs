use rocket;

use crate::connection;
use rocket::Rocket;
use crate::app::handler;

pub fn get_rocket() -> Rocket {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/api",
               routes![
                    handler::create_session
                    ],
        )
}