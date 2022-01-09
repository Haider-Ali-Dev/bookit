// C -> Client -- Deserialize
// S -> Server -- Serialize
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct CRegister {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct SRegister {
    pub name: String,
    pub email: String,
    pub id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CBookmark {
    pub bookmark_url: String,
    pub bookmark_name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct BookmarkStatus {
    pub status: bool,
}

#[derive(Debug, Deserialize)]
pub struct CSignIn {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct CBookmarkRemove {
    pub email: String,
    pub bookmark_url: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct BookmarkFromDB {
    pub bookmark_url: String,
    pub bookmark_name: String,
    pub date: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CEmail {
    pub email: String,
}
