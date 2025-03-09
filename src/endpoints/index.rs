use std::path::Path;
use actix_web::{get, web, HttpResponse, Responder};
use pathdiff::diff_paths;
use serde::{Deserialize, Serialize};
use crate::server_config::ServerConfig;

#[derive(Serialize)]
struct Item {
    path: String,
    mime: String,
    name: String,
    is_dir: bool,
}

#[derive(Serialize)]
struct ResponseSuccess {
    success: bool,
    items: Vec<Item>,
    current_path: String,
    parent_path: String,
    root_path: String,
}

#[derive(Serialize)]
struct ResponseError {
    success: bool,
    message: &'static str,
}

#[derive(Deserialize)]
struct IndexQuery {
    path: Option<String>,
}


#[get("/index")]
pub async fn index(query: web::Query<IndexQuery>, config: web::Data<ServerConfig>) -> impl Responder {
    let current_path = config.get_current_path().replace("\\", "/");
    let root = Path::new(&current_path);
    let mut path = query.path.clone().unwrap_or("/".into()).replace("\\", "/");
    while path.starts_with('/') {
        path = path[1..].to_string()
    }
    let joined_path = root.join(&path);
    println!("{} + {} = {}", (&root).to_str().unwrap(), &path, (&joined_path).to_str().unwrap());

    let path = joined_path.as_path();

    if !path.is_dir() {
        return HttpResponse::NotFound()
            .json(ResponseError {
                success: false,
                message: "Not a directory"
            });
    }

    let parent = diff_paths(path.parent().unwrap_or(path), root).unwrap();
    let files: Vec<_> = std::fs::read_dir(&joined_path).unwrap()
        .map(|r| r.unwrap().path())
        .collect();

    let items = files.iter().map(|entry| {
        let path = String::from(diff_paths(entry, root).unwrap().to_str().unwrap()).replace("\\", "/");
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

    let current_path = String::from(joined_path.to_str().unwrap()).replace("\\", "/");
    let parent_path = String::from(parent.to_str().unwrap()).replace("\\", "/");
    let root_path = String::from(root.to_str().unwrap()).replace("\\", "/");

    HttpResponse::Ok()
        .json(ResponseSuccess {
            success: true,
            items,
            current_path,
            parent_path,
            root_path,
        })
}