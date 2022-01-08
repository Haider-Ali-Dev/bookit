use self::database::Database;

pub mod database;
pub mod http;
pub mod http_models;
use rocket::{Rocket, Build};
pub struct Config {
    pub database: Database
}

pub fn rocket(config: Config) -> Rocket<Build> {
    rocket::build()
        .manage(config.database)
        .mount("/", http::routes::routes())
}