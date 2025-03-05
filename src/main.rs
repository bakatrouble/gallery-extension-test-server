#[macro_use] extern crate rocket;

pub mod endpoints;
mod server_config;

use rocket_dyn_templates::Template;
use server_config::ServerConfig;
use endpoints::{
    change_path::change_path,
    index::index,
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8474)))
        .attach(Template::fairing())
        .manage(ServerConfig::new())
        .mount("/", routes![change_path])
        .mount("/", routes![index])
}
