use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::model::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ChatGroup {
    pub id: i64,
    pub title: String,
    pub users: Vec<User>,
    pub created_at: DateTime<Utc>,
}
