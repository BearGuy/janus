use tokio::sync::MutexGuard;
use rusqlite::{ NO_PARAMS, params, Connection, Result };
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::types::{User};

pub struct DB {
    conn: Connection
}

impl DB {
    pub fn new() -> Self {
        let conn = Connection::open("auth.db").unwrap();
        let _result = conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id integer primary key,
                username text not null unique,
                password text not null,
                email text,
                created_at integer not null,
                updated_at integer not null
            )",
            NO_PARAMS,
        ).unwrap();

        DB {
            conn
        }     
    }

}

//
// USER SERVICE FUNCTIONS
//
pub fn contains_user(db: &MutexGuard<DB>, username: String) -> bool {
    let users = get_user_by_username(db, username).unwrap();
    return users.len() > 0
}

pub fn get_user_by_username(
    db: &MutexGuard<DB>,
    username: String
    ) -> Result<Vec<User>> 
{
    let mut stmt = db.conn.prepare("SELECT * FROM users WHERE username = :username")?;

    let rows = stmt
        .query_map_named(&[(":username", &username)], |row|
            Ok (
                User {
                    username: row.get(1)?,
                    password: row.get(2)?,
                    email: row.get(3)?,
                    created_at: convert_timestamp_to_utc(row.get(4)?),
                    updated_at: convert_timestamp_to_utc(row.get(5)?),
                } 
            )
        )?;

    let mut users = Vec::new();
    for user in rows {
        users.push(user?);
    }

    Ok(users)
}

pub fn get_user(db: &MutexGuard<DB>, username: String) -> Result<Option<User>> {
    let users = get_user_by_username(db, username)?;
    match users.first() {
        None => Ok(None),
        Some(user) => Ok(Some(user.clone())) 
    }
}

pub fn insert_user(db: &MutexGuard<DB>, new_user: User) -> Result<()> {
   db.conn.execute(
        "INSERT INTO users (username, password, email, created_at, updated_at)
                  VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            new_user.username, 
            new_user.password, 
            new_user.email,
            new_user.created_at.timestamp(),
            new_user.updated_at.timestamp(),
        ],
    )?;
   Ok(())
}

pub fn convert_timestamp_to_utc(ts: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(ts, 0), 
        Utc
    )
}
