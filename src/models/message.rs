use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub sender: User,
    pub content: String,
}
