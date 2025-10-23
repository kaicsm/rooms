mod app;
mod error;
mod managers;
mod models;
mod routes;

use std::sync::RwLock;

use rocket::{
    Build, Rocket,
    fs::{FileServer, Options},
    launch, routes,
};

use crate::routes::{room::*, user::get_users};
use crate::{app::AppState, routes::user::register_user};

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(RwLock::new(AppState::new()))
        .mount(
            "/",
            FileServer::new("public", Options::Index | Options::DotFiles),
        )
        .mount(
            "/room",
            routes![
                create_room,
                join_room,
                get_room,
                delete_user_from_room,
                send_message,
                stream_messages
            ],
        )
        .mount("/user", routes![register_user, get_users])
}
