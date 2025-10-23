use std::{str::FromStr, sync::Arc};

use bcrypt::{DEFAULT_COST, hash};
use uuid::Uuid;

use crate::{managers::database_manager::DatabaseManager, models::user::User};

pub struct UserManager {
    database_manager: Arc<DatabaseManager>,
}

impl UserManager {
    pub fn new(database_manager: Arc<DatabaseManager>) -> Self {
        Self::setup(&database_manager);
        Self { database_manager }
    }

    fn setup(database_manager: &DatabaseManager) {
        database_manager
            .0
            .lock()
            .unwrap()
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                        id TEXT NOT NULL,
                        username TEXT NOT NULL,
                        password TEXT NOT NULL
                    )",
                [],
            )
            .unwrap();
    }

    pub fn register_user(&self, username: String, password: String) -> User {
        let hashed_password = Self::hash_password(&password);
        let id = Uuid::new_v4();

        self.database_manager
            .0
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO users (id, username, password) VALUES (?, ?, ?)",
                [id.to_string(), username.clone(), hashed_password],
            )
            .unwrap();

        User { id, username }
    }

    pub fn get_users(&self) -> Result<Vec<User>, rusqlite::Error> {
        let users = self
            .database_manager
            .0
            .lock()
            .unwrap()
            .prepare("SELECT id, username FROM users")?
            .query_map([], |row| {
                Ok(User {
                    id: Uuid::from_str(&row.get::<_, String>(0)?).unwrap(),
                    username: row.get(1)?,
                })
            })?
            .map(|result| result.unwrap())
            .collect();

        Ok(users)
    }

    fn hash_password(password: &String) -> String {
        hash(password, DEFAULT_COST).expect("Failed to hash password")
    }

    // fn validate_password(password: &String, hashed_password: &String) -> bool {
    //     verify(password, hashed_password).expect("Failed to verify password")
    // }
}
