use actix_web::{get, web, HttpResponse, Responder};
use crate::server_config::ServerConfig;

#[get("/server_config")]
pub async fn get_server_config(config: web::Data<ServerConfig>) -> impl Responder {
    HttpResponse::Ok()
        .json(config.serialize())
}
