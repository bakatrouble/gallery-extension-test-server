use std::io::{Cursor};
use std::path::{Path};
use cached::{
    proc_macro::cached,
    SizedCache,
};
use mime_guess::{mime, Mime};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use thumbnailer::{create_thumbnails, ThumbnailSize};
use thumbnailer::error::ThumbError;
use crate::server_config::ServerConfig;

#[cached(
    ty = "SizedCache<u128, Vec<u8>>",
    create = "{ SizedCache::with_size(1000) }",
    convert = r#"{ xxhash_rust::xxh3::xxh3_128(file_bytes) }"#,
    result = true,
)]
fn generate_thumbnail(file_bytes: &Vec<u8>, mime: &Mime) -> Result<Vec<u8>, ThumbError> {
    let reader = Cursor::new(file_bytes);
    let mut thumbnails = create_thumbnails(reader, mime.clone(), [ThumbnailSize::Medium])?;
    let thumb = thumbnails.pop().unwrap();
    let mut buf = Cursor::new(Vec::new());
    thumb.write_jpeg(&mut buf, 80)?;
    let vec = buf.into_inner();
    Ok(vec)
}

#[derive(Deserialize)]
struct ThumbnailQuery {
    path: String,
}

#[get("/thumbnail")]
pub async fn thumbnail(query: web::Query<ThumbnailQuery>, config: web::Data<ServerConfig>) -> impl Responder {
    let current_path = config.get_current_path();
    let root = Path::new(&current_path);
    let mut path = query.path.clone();
    while path.starts_with('/') {
        path = path[1..].to_string()
    }
    let joined_path = root.join(&path);
    println!("{} + {} = {}", (&root).to_str().unwrap(), &path, (&joined_path).to_str().unwrap());

    if joined_path.is_file() {
        let mime = mime_guess::from_path(joined_path.to_str().unwrap()).first_or_octet_stream();
        if mime.type_() != mime::IMAGE {
            return HttpResponse::BadRequest()
                .body("400 Not an image");
        }

        if let Ok(bytes) = std::fs::read(&joined_path) {
            let vec = generate_thumbnail(&bytes, &mime).unwrap();
            HttpResponse::Ok()
                .content_type("jpeg")
                .body(vec)
        } else {
            HttpResponse::InternalServerError()
                .body("500 Couldn't open")
        }
    } else {
        HttpResponse::NotFound()
            .body("404 Not Found")
    }
}