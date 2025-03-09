use std::path::Path;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::server_config::ServerConfig;

#[derive(Deserialize)]
struct ChangePath {
    path: String,
}

#[derive(Serialize)]
struct Response {
    success: bool,
    path: String,
    message: Option<&'static str>,
}

#[post("/change_path")]
pub async fn change_path(body: web::Json<ChangePath>, config: web::Data<ServerConfig>) -> impl Responder {
    let new_path = body.path.clone();
    if !Path::new(new_path.as_str()).is_dir() {
        HttpResponse::BadRequest()
            .json(Response {
                success: false,
                path: new_path,
                message: Some("Not a directory"),
            })
    } else {
        config.set_current_path(&new_path);
        HttpResponse::Ok()
            .json(Response {
                success: true,
                path: new_path,
                message: None,
            })
    }
}
