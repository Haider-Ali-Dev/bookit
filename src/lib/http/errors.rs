use serde::Serialize;




#[derive(Debug, thiserror::Error, Serialize)]
pub enum ApiError {
    #[error("User not found")]
    UserNotFound,
    #[error("Database Error: {0}")]
    DatabaseError(String),
    #[error("Wrong Credentials")]
    WrongCredentials,
    #[error("Bookmarks not found in the database")]
    NoBookmarksInDatabase
}
