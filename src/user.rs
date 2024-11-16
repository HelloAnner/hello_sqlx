use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    created_at: DateTime<Utc>,
}