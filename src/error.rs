use rocket::{http::Status, response::Responder};
use serde::Serialize;

#[derive(Serialize)]
pub enum RoomError {
    UserAlreadyExists,
    UserNotExists,
    RoomNotExists,
    PermissionDenied,
}

impl<'r> Responder<'r, 'static> for RoomError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            RoomError::PermissionDenied => Status::Forbidden,
            RoomError::RoomNotExists => Status::NotFound,
            RoomError::UserNotExists => Status::NotFound,
            RoomError::UserAlreadyExists => Status::Conflict,
        };
        Err(status)
    }
}
