use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::model::user::User;
use crate::model::message::Message;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ChatGroup {
    pub id: i64,
    pub title: String,
    pub image_uri: String,
    pub admin_id: String, // or User
    pub users: Vec<User>,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
}
