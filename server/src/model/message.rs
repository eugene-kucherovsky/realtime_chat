// use chrono::{DateTime, Utc};
use scylla::macros::FromRow;
// use scylla::macros::{FromUserType, IntoUserType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
#[allow(non_snake_case)]
pub struct Message {
    pub message_id: i64,
    // pub sender_id: String,
    // pub recipient_id: String,
    // pub chat_group_id: String,
    // pub received: bool,
    // pub readed: bool,
    pub message_text: String,
    // pub created_at: DateTime<Utc>,
}
