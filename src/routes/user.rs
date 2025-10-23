use std::sync::RwLock;

use rocket::{State, get, post, serde::json::Json};

use crate::{
    app::AppState,
    models::{requests::RegisterUserRequest, user::User},
};

#[post("/", format = "json", data = "<request>")]
pub fn register_user(
    app: &State<RwLock<AppState>>,
    request: Json<RegisterUserRequest>,
) -> Json<User> {
    let user = app
        .read()
        .unwrap()
        .user_manager
        .register_user(request.username.clone(), request.password.clone());

    Json(user)
}

#[get("/")]
pub fn get_users(app: &State<RwLock<AppState>>) -> Json<Vec<User>> {
    let users = app.read().unwrap().user_manager.get_users().unwrap();
    Json(users)
}
