use std::collections::HashMap;

use rocket::tokio::sync::broadcast::{self, Sender};
use uuid::Uuid;

use crate::{
    error::RoomError,
    models::{message::Message, room::Room, user::User},
};

pub struct RoomManager {
    rooms: HashMap<Uuid, Room>,
    senders: HashMap<Uuid, Sender<Message>>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            senders: HashMap::new(),
        }
    }

    pub fn create_room(&mut self, user: User) -> Uuid {
        let mut room = Room::new();
        room.add_user(user.clone());
        room.add_moderator(user.clone());

        let (tx, _rx) = broadcast::channel(16);
        self.senders.insert(room.id, tx);

        let id = room.id;
        self.rooms.insert(room.id, room);

        id
    }

    pub fn join_room(&mut self, room_id: Uuid, user: User) -> Result<(), RoomError> {
        if let Some(room) = self.rooms.get_mut(&room_id) {
            if room.is_user(user.id) {
                return Err(RoomError::UserAlreadyExists);
            }

            room.add_user(user);
            Ok(())
        } else {
            Err(RoomError::RoomNotExists)
        }
    }

    pub fn get_room(&self, room_id: Uuid) -> Result<Room, RoomError> {
        if let Some(room) = self.rooms.get(&room_id) {
            Ok(room.clone())
        } else {
            Err(RoomError::RoomNotExists)
        }
    }

    pub fn send_message(&mut self, message: Message, room_id: Uuid) -> Result<(), RoomError> {
        if let Some(room) = self.rooms.get_mut(&room_id) {
            if !room.is_user(message.sender.id) {
                return Err(RoomError::UserNotExists);
            }

            let tx = self.senders.get(&room_id).unwrap();
            room.add_message(message, tx.clone());
            Ok(())
        } else {
            Err(RoomError::RoomNotExists)
        }
    }

    pub fn get_sender(&self, room_id: Uuid) -> Result<Sender<Message>, RoomError> {
        self.senders
            .get(&room_id)
            .cloned()
            .ok_or(RoomError::RoomNotExists)
    }

    pub fn delete_user(
        &mut self,
        room_id: Uuid,
        user_id: Uuid,
        user_to_delete_id: Uuid,
    ) -> Result<(), RoomError> {
        if let Some(room) = self.rooms.get_mut(&room_id) {
            if !room.is_user(user_id) || !room.is_user(user_to_delete_id) {
                return Err(RoomError::UserNotExists);
            }

            if !room.is_moderator(user_id) {
                return Err(RoomError::PermissionDenied);
            }

            room.remove_user(user_to_delete_id);
            Ok(())
        } else {
            Err(RoomError::RoomNotExists)
        }
    }
}
