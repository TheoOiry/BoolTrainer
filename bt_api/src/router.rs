use rocket;

use crate::connection;
use rocket::Rocket;
use crate::app;

pub fn get_rocket() -> Rocket {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/api",
               routes![
                    app::create_session
                    ],
        )
}