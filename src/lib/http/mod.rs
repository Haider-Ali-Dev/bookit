pub mod routes;
pub mod errors;


pub fn date() -> String {
    sqlx::types::chrono::Utc::today().to_string()
}