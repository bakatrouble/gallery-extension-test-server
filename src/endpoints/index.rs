use std::path::Path;
use pathdiff::diff_paths;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::State;
use crate::server_config::ServerConfig;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Item {
    path: String,
    mime: String,
    name: String,
    is_dir: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct IndexJsonSuccess {
    items: Vec<Item>,
    current_path: String,
    parent_path: String,
    root_path: String,
}

#[derive(Responder)]
pub enum IndexJsonResponse {
    #[response(status = 200)]
    Html(Json<IndexJsonSuccess>),
    #[response(status = 404)]
    NotFound(&'static str),
}

#[get("/index?<path>")]
pub async fn index(path: Option<String>, config: &State<ServerConfig>) -> IndexJsonResponse {
    let current_path = config.current_path();
    let root = Path::new(&current_path);
    let mut path = path.unwrap_or("/".into());
    while path.starts_with('/') {
        path = path[1..].to_string()
    }
    let joined_path = root.join(&path);
    println!("{} + {} = {}", (&root).to_str().unwrap(), &path, (&joined_path).to_str().unwrap());

    let path = joined_path.as_path();

    if !path.is_dir() {
        return IndexJsonResponse::NotFound("404 Not Found");
    }

    let parent = diff_paths(path.parent().unwrap_or(path), root).unwrap();
    let files: Vec<_> = std::fs::read_dir(&joined_path).unwrap()
        .map(|r| r.unwrap().path())
        .collect();

    let items = files.iter().map(|entry| {
        let path = String::from(diff_paths(entry, root).unwrap().to_str().unwrap());
        let name = String::from(entry.file_name().unwrap().to_str().unwrap());
        let mime = mime_guess::from_path(entry).first_or_octet_stream().to_string();
        let is_dir = entry.is_dir();
        Item {
            path,
            name,
            mime,
            is_dir,
        }
    }).collect();

    let current_path = String::from(joined_path.to_str().unwrap());
    let parent_path = String::from(parent.to_str().unwrap());
    let root_path = String::from(root.to_str().unwrap());

    let success = IndexJsonSuccess {
        items,
        current_path,
        parent_path,
        root_path,
    };

    IndexJsonResponse::Html(Json(success))
}