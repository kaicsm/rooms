use std::sync::Arc;

use crate::managers::{
    database_manager::DatabaseManager, room_manager::RoomManager, user_manager::UserManager,
};

pub struct AppState {
    pub room_manager: RoomManager,
    pub user_manager: UserManager,
}

impl AppState {
    pub fn new() -> Self {
        let database_manager = Arc::new(DatabaseManager::new());
        let user_manager = UserManager::new(database_manager.clone());
        let room_manager = RoomManager::new();

        AppState {
            room_manager,
            user_manager,
        }
    }
}
