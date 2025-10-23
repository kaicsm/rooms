use serde::{Deserialize, Serialize};

use crate::models::{message::Message, user::User};

#[derive(Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct JoinRoomRequest {
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteUserFromRoomRequest {
    pub user: User,
    pub user_to_delete: User,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub message: Message,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String,
}
