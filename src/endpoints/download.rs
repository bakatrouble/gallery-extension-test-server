use std::path::Path;
use rocket::fs::NamedFile;
use rocket::State;
use crate::server_config::ServerConfig;

#[derive(Responder)]
pub enum DownloadResponse {
    #[response(status = 200)]
    File(NamedFile),
    #[response(status = 404)]
    NotFound(&'static str),
}

#[get("/download?<path>")]
pub async fn download(path: String, config: &State<ServerConfig>) -> DownloadResponse {
    let current_path = config.current_path();
    let root = Path::new(&current_path);
    let mut path = path;
    while path.starts_with('/') {
        path = path[1..].to_string()
    }
    let joined_path = root.join(&path);
    println!("{} + {} = {}", (&root).to_str().unwrap(), &path, (&joined_path).to_str().unwrap());

    if joined_path.is_file() {
        DownloadResponse::File(NamedFile::open(joined_path).await.unwrap())
    } else {
        DownloadResponse::NotFound("404 Not Found")
    }
}