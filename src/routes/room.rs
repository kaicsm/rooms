use std::sync::RwLock;

use rocket::{
    State, delete, get, post,
    response::stream::{Event, EventStream},
    serde::json::Json,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    error::RoomError,
    models::{
        requests::{
            CreateRoomRequest, DeleteUserFromRoomRequest, JoinRoomRequest, SendMessageRequest,
        },
        room::Room,
    },
};

#[post("/", format = "json", data = "<request>")]
pub fn create_room(app: &State<RwLock<AppState>>, request: Json<CreateRoomRequest>) -> Json<Uuid> {
    let id = app
        .write()
        .unwrap()
        .room_manager
        .create_room(request.user.clone());
    Json(id)
}

#[post("/<room_id>/join", format = "json", data = "<request>")]
pub fn join_room(
    app: &State<RwLock<AppState>>,
    room_id: Uuid,
    request: Json<JoinRoomRequest>,
) -> Result<(), RoomError> {
    app.write()
        .unwrap()
        .room_manager
        .join_room(room_id, request.user.clone())
}

#[get("/<room_id>")]
pub fn get_room(app: &State<RwLock<AppState>>, room_id: Uuid) -> Result<Json<Room>, RoomError> {
    app.read()
        .unwrap()
        .room_manager
        .get_room(room_id)
        .map(|r| Json(r))
}

#[delete("/<room_id>", format = "json", data = "<request>")]
pub fn delete_user_from_room(
    app: &State<RwLock<AppState>>,
    room_id: Uuid,
    request: Json<DeleteUserFromRoomRequest>,
) -> Result<(), RoomError> {
    app.write().unwrap().room_manager.delete_user_from_room(
        room_id,
        request.user.id,
        request.user_to_delete.id,
    )
}

#[post("/<room_id>", format = "json", data = "<request>")]
pub fn send_message(
    app: &State<RwLock<AppState>>,
    room_id: Uuid,
    request: Json<SendMessageRequest>,
) -> Result<(), RoomError> {
    app.write()
        .unwrap()
        .room_manager
        .send_message(request.message.clone(), room_id)
}

#[get("/<room_id>/stream")]
pub fn stream_messages(
    app: &State<RwLock<AppState>>,
    room_id: Uuid,
) -> Result<EventStream![], RoomError> {
    let tx = app.read().unwrap().room_manager.get_sender(room_id)?;
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
