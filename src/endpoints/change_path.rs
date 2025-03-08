use std::path::Path;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::State;
use crate::server_config::ServerConfig;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ChangePath {
    path: String,
}

#[derive(Responder)]
pub enum ChangePathResponse {
    #[response(status = 200)]
    Ok(String),
    #[response(status = 400)]
    Err(&'static str),
}

#[post("/change_path", format = "json", data = "<data>")]
pub fn change_path(data: Json<ChangePath>, config: &State<ServerConfig>) -> ChangePathResponse {
    let new_path = data.path.clone();
    if !Path::new(new_path.as_str()).is_dir() {
        ChangePathResponse::Err("Invalid path")
    } else {
        config.set_current_path(&new_path);
        ChangePathResponse::Ok(new_path)
    }
}
