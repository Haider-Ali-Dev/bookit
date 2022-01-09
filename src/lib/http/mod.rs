pub mod errors;
pub mod routes;

pub fn date() -> String {
    sqlx::types::chrono::Utc::today().to_string()
}
