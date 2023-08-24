use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    pub user_id: u64,
    pub password: String,
    pub email: String,
    pub phone_number: String,
    pub name: String,
    pub photo: String,
    pub last_connection: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct FilteredUser {
    pub email: String,
    pub phone_number: String,
    pub name: String,
    pub photo: String,
}
