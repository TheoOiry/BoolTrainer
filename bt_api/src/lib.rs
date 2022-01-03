#![feature(decl_macro, proc_macro_hygiene, trait_alias)]
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate jsonwebtoken;
extern crate uuid;
#[macro_use]
extern crate diesel_migrations;

pub mod app;
mod connection;
pub mod router;
mod schema;
