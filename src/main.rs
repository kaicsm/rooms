mod error;
mod managers;
mod models;

use std::sync::RwLock;

use rocket::{
    Build, Rocket, State, delete,
    fs::{FileServer, Options},
    get, launch, post,
    response::stream::{Event, EventStream},
    routes,
    serde::{json::Json, uuid::Uuid},
};
use serde::{Deserialize, Serialize};

use crate::{
    error::RoomError,
    managers::room_manager::RoomManager,
    models::{message::Message, room::Room, user::User},
};

#[derive(Serialize, Deserialize)]
struct CreateRoomRequest {
    user: User,
}

#[derive(Serialize, Deserialize)]
struct JoinRoomRequest {
    user: User,
}

#[derive(Serialize, Deserialize)]
struct DeleteUserRequest {
    user: User,
    user_to_delete: User,
}

#[derive(Serialize, Deserialize)]
struct SendMessageRequest {
    message: Message,
}

#[post("/rooms", format = "json", data = "<request>")]
fn create_room(
    room_manager: &State<RwLock<RoomManager>>,
    request: Json<CreateRoomRequest>,
) -> Json<Uuid> {
    let id = room_manager
        .write()
        .unwrap()
        .create_room(request.user.clone());
    Json(id)
}

#[post("/rooms/<room_id>/join", format = "json", data = "<request>")]
fn join_room(
    room_manager: &State<RwLock<RoomManager>>,
    room_id: Uuid,
    request: Json<JoinRoomRequest>,
) -> Result<(), RoomError> {
    room_manager
        .write()
        .unwrap()
        .join_room(room_id, request.user.clone())
}

#[get("/rooms/<room_id>")]
fn get_room(
    room_manager: &State<RwLock<RoomManager>>,
    room_id: Uuid,
) -> Result<Json<Room>, RoomError> {
    room_manager
        .read()
        .unwrap()
        .get_room(room_id)
        .map(|r| Json(r))
}

#[delete("/rooms/<room_id>", format = "json", data = "<request>")]
fn delete_user(
    room_manager: &State<RwLock<RoomManager>>,
    room_id: Uuid,
    request: Json<DeleteUserRequest>,
) -> Result<(), RoomError> {
    room_manager
        .write()
        .unwrap()
        .delete_user(room_id, request.user.id, request.user_to_delete.id)
}

#[post("/rooms/<room_id>", format = "json", data = "<request>")]
fn send_message(
    room_manager: &State<RwLock<RoomManager>>,
    room_id: Uuid,
    request: Json<SendMessageRequest>,
) -> Result<(), RoomError> {
    room_manager
        .write()
        .unwrap()
        .send_message(request.message.clone(), room_id)
}

#[get("/rooms/<room_id>/stream")]
fn stream_messages(
    room_manager: &State<RwLock<RoomManager>>,
    room_id: Uuid,
) -> Result<EventStream![], RoomError> {
    let tx = room_manager.read().unwrap().get_sender(room_id)?;
    let mut rx = tx.subscribe();

    Ok(EventStream! {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    yield Event::json(&msg);
                },

                Err(_) => continue,
            }
        }
    })
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(RwLock::new(RoomManager::new()))
        .mount(
            "/",
            FileServer::new("public", Options::Index | Options::DotFiles),
        )
        .mount(
            "/",
            routes![
                create_room,
                join_room,
                get_room,
                delete_user,
                send_message,
                stream_messages
            ],
        )
}
