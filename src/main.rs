pub mod endpoints;
mod server_config;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use server_config::ServerConfig;
use rust_embed::Embed;
use env_logger::Env;
use endpoints::{
    change_path::change_path,
    thumbnail::thumbnail,
    index::index,
    download::download,
};

#[derive(Embed)]
#[folder = "ui/dist"]
struct Asset;

async fn assets(req: HttpRequest) -> impl Responder {
    let path_tmp = format!("__assets/{}", req.match_info().query("path"));
    let path = path_tmp.as_str();
    if let Some(file) = Asset::get(path) {
        let content_type = mime_guess::from_path(path).first_or_octet_stream().to_string();
        HttpResponse::Ok()
            .content_type(content_type)
            .body(file.data)
    } else {
        HttpResponse::NotFound()
            .body(format!("404 Not Found: {}", path))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let is_debug = cfg!(debug_assertions);
    let port = if is_debug { 9474 } else { 8474 };

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app = HttpServer::new(move || {
        let cors = if is_debug {
            Cors::default()
                .allowed_methods(vec!["GET", "POST"])
                .allow_any_header()
                .allow_any_origin()
        } else {
            Cors::default()
        };

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(ServerConfig::new()))
            .service(
                web::scope("/api")
                    .service(download)
                    .service(thumbnail)
                    .service(index)
                    .service(change_path)
            )
            .service(
                web::resource("/__assets/{path:.*}")
                    .route(web::get().to(assets))
            )
            .default_service(web::route().to(async || {
                HttpResponse::Ok()
                    .content_type("html")
                    .body(Asset::get("index.html").unwrap().data)
            }))
    })
        .bind(("127.0.0.1", port))?
        .run();
    app.await
}
