use rocket::{get, post, routes, serde::json::Json, State, Route,};
use crate::{database::{Database, models::Id}, http_models::models::{SRegister, CSignIn, User, CBookmark, BookmarkStatus, CBookmarkRemove, BookmarkFromDB, CEmail}};
use crate::database::models::{Password, Email};
use super::errors::ApiError;
use super::super::http_models::models::{CRegister};
use bcrypt::{DEFAULT_COST, hash, verify};
use super::date;

#[get("/")]
async fn home() -> String {
    "Welcome to Bookmark API".to_owned()
}

#[post("/register", data = "<user>")]
async fn register(user: Json<CRegister>, db: &State<Database>) -> Result<Json<SRegister>, Json<ApiError>> {
    let hashed_password = hash(user.password.clone(), DEFAULT_COST)
    .expect("Error while hashing the password");
    let db_clone = db.pool.clone();
    sqlx::query!("INSERT INTO users (name, email, password) values($1, $2, $3)",
    user.name.clone(),
    user.email.clone(),
    hashed_password.clone()
    ).execute(&db_clone).await.unwrap();

    let user_data = sqlx::query_as!(SRegister, "SELECT name, email, id FROM users where email = $1", user.email.clone())
    .fetch_one(&db_clone)
    .await;
    
    match user_data {
        Ok(user_d) => Ok(Json(user_d)),
        Err(_) => Err(Json(ApiError::DatabaseError(format!("Database error while getting the user."))))
    }

}


#[post("/signin", data = "<user>")]
async fn sign_in(user: Json<CSignIn>, db: &State<Database>) -> Result<Json<User>, Json<ApiError>> {
    let db_clone = db.pool.clone();
    let hashed_password = sqlx::query_as!(Password,
    "SELECT password from users where email = $1",
    user.email.clone())
    .fetch_one(&db_clone)
    .await;

    match hashed_password {
        Ok(hash) => {
            let is_password_true = verify(user.password.clone(), &hash.password).unwrap();
            match is_password_true {
                true => {
                    let user_data = sqlx::query_as!(User,
                        "SELECT id, name, email from users where email = $1 ",
                        user.email.clone())
                        .fetch_one(&db_clone)
                        .await
                        .unwrap();
                    Ok(
                        Json(
                            user_data
                        )
                    )
                },
                false => Err(Json(ApiError::WrongCredentials))
            }
        },
        
        Err(_) => {
            Err(Json(ApiError::DatabaseError(format!("Error while getting the password from the database."))))
        }
    }    
}

#[post("/bookmark-add", data = "<bookmark>")]
async fn bookmark_add(bookmark: Json<CBookmark>, database: &State<Database>) -> Result<Json<BookmarkStatus>, Json<ApiError>> {
    let db_clone = database.pool.clone();
    let id = sqlx::query_as!(Id,
        "SELECT id from users where email = $1",
        bookmark.email.clone())
        .fetch_one(&db_clone)
        .await;

    match id {
        Ok(_) => {
            let query= sqlx::query!(
                "INSERT into bookmark (bookmark_url, bookmark_name, email, date) values($1, $2, $3, $4)",
                bookmark.bookmark_url.clone(),
                bookmark.bookmark_name.clone(),
                bookmark.email.clone(),
                date()
            ).execute(&db_clone).await;
            match query {
                Ok(_) => Ok(Json(BookmarkStatus { status: true })),
                Err(_) => Err(Json(ApiError::DatabaseError(format!("Error while adding the boomark in the db"))))
            }

        },
        Err(_) => Err(Json(ApiError::UserNotFound))
    }
}


#[post("/bookmark-delete", data = "<bookmark>")] 
async fn bookmark_delete(bookmark: Json<CBookmarkRemove>, database: &State<Database>) -> Result<Json<BookmarkStatus>, Json<ApiError>> {
    let db_clone = database.pool.clone();
    let db_bookmark_data = sqlx::query_as!(Email,
        "SELECT email, bookmark_name from bookmark where email = $1 and bookmark_url = $2",
        bookmark.email.clone(),
        bookmark.bookmark_url.clone()
    ).fetch_one(&db_clone).await;

    match db_bookmark_data {
        Ok(_) => {
            let del =sqlx::query!("DELETE FROM bookmark WHERE bookmark_url = $1", bookmark.bookmark_url.clone())
            .execute(&db_clone).await;
            match del {
                Ok(_) => Ok(Json(BookmarkStatus { status: true })),
                Err(_) => Err(Json(ApiError::DatabaseError("Error while deleting".to_owned())))  
            }
        },
        Err(_) => Err(Json(ApiError::UserNotFound))
    }
}

#[post("/", data = "<bookmark>")]
async fn bookmark_list(bookmark: Json<CEmail>, database: &State<Database>) -> Result<Json<Vec<BookmarkFromDB>>, Json<ApiError>> {
    let db_clone = database.pool.clone();
    let bookmarks = sqlx::query_as!(BookmarkFromDB,
     "SELECT date,bookmark_name,bookmark_url from bookmark where email = $1", bookmark.email.clone())
      .fetch_all(&db_clone).await;

    match bookmarks {
        Ok(books) => {
            Ok(Json(books))
        },
        Err(_) => Err(Json(ApiError::NoBookmarksInDatabase))
    }
}


pub fn routes() -> Vec<Route> {
    routes![home, register, sign_in, bookmark_add, bookmark_delete, bookmark_list]
}