use std::sync::Arc;
use argon2::{self, Config};
use rand::Rng;
use tokio::sync::Mutex;
use warp::{Filter, http::StatusCode};

pub mod db;
pub mod types;

use crate::db::{
    DB,
    contains_user,
    get_user,
    insert_user
};
use crate::types::{User, UserRegisterRequest, UserLoginRequest};


#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(DB::new()));
    let db = warp::any().map(move || Arc::clone(&db));

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(db.clone())
        .and_then(register);
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(db.clone())
        .and_then(login);

    let routes = register.or(login);
    warp::serve(routes).run(([127,0,0,1], 3030)).await;
}

async fn register(
    new_user: UserRegisterRequest,
    db: Arc<Mutex<DB>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    if contains_user(&db, new_user.clone().username) {
        return Ok(StatusCode::BAD_REQUEST);
    }

    let hashed_user = User::new(
        new_user.username,
        hash(new_user.password.as_bytes()),
        new_user.email 
    );

    insert_user(&db, hashed_user.clone()).unwrap();
    Ok(StatusCode::CREATED)
}

async fn login (
    credentials: UserLoginRequest,
    db: Arc<Mutex<DB>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match get_user(&db, credentials.username).unwrap() {
        None => Ok(StatusCode::BAD_REQUEST),
        Some(user) => {
            if verify(&user.password, credentials.password.as_bytes()) {
                Ok(StatusCode::OK)
            } else {
                Ok(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

