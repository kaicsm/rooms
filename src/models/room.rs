use rocket::tokio::sync::broadcast::Sender;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{message::Message, user::User};

#[derive(Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub users: Vec<User>,

    pub messages: Vec<Message>,

    #[serde(skip)]
    moderators: Vec<User>,
}

impl Room {
    pub fn new() -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            users: Vec::new(),
            messages: Vec::new(),
            moderators: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn add_moderator(&mut self, moderator: User) {
        self.moderators.push(moderator);
    }

    pub fn add_message(&mut self, message: Message, sender: Sender<Message>) {
        if let Err(e) = sender.send(message.clone()) {
            eprintln!("Erro ao enviar mensagem para o canal: {e}")
        }
        self.messages.push(message);
    }

    pub fn is_moderator(&self, user_id: Uuid) -> bool {
        self.moderators.iter().any(|m| m.id == user_id)
    }

    pub fn is_user(&self, user_id: Uuid) -> bool {
        self.users.iter().any(|u| u.id == user_id)
    }

    pub fn remove_user(&mut self, id: Uuid) {
        self.users.retain(|user| user.id != id);
    }
}
