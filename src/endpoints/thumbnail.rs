use std::io::{Cursor};
use std::path::{Path};
use cached::{
    proc_macro::cached,
    SizedCache,
};
use mime_guess::{mime, Mime};
use rocket::http::ContentType;
use rocket::State;
use thumbnailer::{create_thumbnails, ThumbnailSize};
use thumbnailer::error::ThumbError;
use crate::server_config::ServerConfig;

#[derive(Responder)]
pub enum ThumbnailResponse {
    #[response(status = 200)]
    File((ContentType, Vec<u8>)),
    #[response(status = 400)]
    ParamError(&'static str),
    #[response(status = 404)]
    NotFound(&'static str),
    #[response(status = 500)]
    InternalError(&'static str),
}

#[cached(
    ty = "SizedCache<u128, Vec<u8>>",
    create = "{ SizedCache::with_size(1000) }",
    convert = r#"{ xxhash_rust::xxh3::xxh3_128(file_bytes) }"#,
    result = true,
)]
fn generate_thumbnail(file_bytes: &Vec<u8>, mime: &Mime) -> Result<Vec<u8>, ThumbError> {
    let reader = Cursor::new(file_bytes);
    let mut thumbnails = create_thumbnails(reader, mime.clone(), [ThumbnailSize::Medium])?;
    let thumbnail = thumbnails.pop().unwrap();
    let mut buf = Cursor::new(Vec::new());
    thumbnail.write_jpeg(&mut buf, 80)?;
    let vec = buf.into_inner();
    Ok(vec)
}

#[get("/thumbnail?<path>")]
pub async fn thumbnail(mut path: String, config: &State<ServerConfig>) -> ThumbnailResponse {
    let current_path = config.current_path();
    let root = Path::new(&current_path);
    while path.starts_with('/') {
        path = path[1..].to_string()
    }
    let joined_path = root.join(&path);
    println!("{} + {} = {}", (&root).to_str().unwrap(), &path, (&joined_path).to_str().unwrap());

    if joined_path.is_file() {
        let mime = mime_guess::from_path(joined_path.to_str().unwrap()).first_or_octet_stream();
        if mime.type_() != mime::IMAGE {
            return ThumbnailResponse::ParamError("400 Not an image");
        }

        if let Ok(bytes) = std::fs::read(&joined_path) {
            let vec = generate_thumbnail(&bytes, &mime).unwrap();
            ThumbnailResponse::File((ContentType::JPEG, vec))
        } else {
            ThumbnailResponse::InternalError("500 Couldn't open")
        }
    } else {
        ThumbnailResponse::NotFound("404 Not Found")
    }
}