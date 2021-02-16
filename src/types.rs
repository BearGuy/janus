use serde::Deserialize;
use chrono::prelude::*;

#[derive(Clone, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize)]
pub struct UserRegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Clone, Deserialize)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new (
        username: String, 
        password: String,
        email: String,
    ) -> Self {
        return User { 
            username, 
            password,
            email,
            created_at: Utc::now(), 
            updated_at: Utc::now(), 
        };
    }
}
