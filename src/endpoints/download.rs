use std::path::Path;
use actix_web::{get, web, Either, HttpResponse};
use actix_files::NamedFile;
use serde::Deserialize;
use crate::server_config::ServerConfig;

#[derive(Deserialize)]
struct DownloadQuery {
    path: String,
}

#[get("/download")]
pub async fn download(query: web::Query<DownloadQuery>, config: web::Data<ServerConfig>) -> Either<NamedFile, HttpResponse> {
    let current_path = config.get_current_path();
    let root = Path::new(&current_path);
    let mut path = query.path.clone();
    while path.starts_with('/') {
        path = path[1..].to_string()
    }
    let joined_path = root.join(&path);
    println!("{} + {} = {}", (&root).to_str().unwrap(), &path, (&joined_path).to_str().unwrap());

    if joined_path.is_file() {
        Either::Left(NamedFile::open(joined_path).unwrap())
        // DownloadResponse::File(NamedFile::open(joined_path).await.unwrap())
    } else {
        Either::Right(HttpResponse::NotFound()
            .body("404 Not Found"))
    }
}
