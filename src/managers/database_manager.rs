use std::sync::Mutex;

use rusqlite::Connection;

pub struct DatabaseManager(pub Mutex<Connection>);

impl DatabaseManager {
    pub fn new() -> Self {
        let conn = Mutex::new(Connection::open("app.db").unwrap());

        Self(conn)
    }
}
