#[macro_use] extern crate rocket;

pub mod endpoints;
mod server_config;

use std::ffi::OsStr;
use std::path::PathBuf;
use rocket::http::{ContentType, Method};
use rocket::response::status::NotFound;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_dyn_templates::Template;
use server_config::ServerConfig;
use rust_embed::Embed;
use endpoints::{
    change_path::change_path,
    thumbnail::thumbnail,
    index::index,
    download::download,
};

#[derive(Embed)]
#[folder = "ui/dist"]
struct Asset;

#[get("/<path..>")]
fn get_static(path: PathBuf) -> Result<(ContentType, Vec<u8>), NotFound<String>> {
    let mut content_type = ContentType::from_extension(path.extension().unwrap_or(OsStr::new("/")).to_str().unwrap())
        .unwrap_or(ContentType::Binary);
    let mut path = path.to_str().unwrap();
    if !path.starts_with("assets/") {
        path = "index.html";
        content_type = ContentType::HTML;
    }
    if let Some(file) = Asset::get(path) {
        Ok((
            content_type,
            file.data.to_vec(),
        ))
    } else {
        Err(NotFound(format!("404 Not Found: {}", path)))
    }
}

#[launch]
fn rocket() -> _ {
    let is_debug = cfg!(debug_assertions);
    let port = if is_debug { 9474 } else { 8474 };

    let cors = CorsOptions::default()
        .allowed_origins(if is_debug {
            AllowedOrigins::All
        } else {
            AllowedOrigins::some_null()
        })
        .allowed_methods(
            vec![Method::Get, Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", port)))
        .attach(Template::fairing())
        .attach(cors.to_cors().unwrap())
        .manage(ServerConfig::new())
        .mount("/api/", routes![index, thumbnail, change_path, download])
        .mount("/", routes![get_static])
}
