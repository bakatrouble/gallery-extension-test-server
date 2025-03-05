#[macro_use] extern crate rocket;

pub mod endpoints;

use std::sync::{Arc, Mutex};
use rocket_dyn_templates::{Template};

pub struct Config {
    pub current_path_str: Arc<Mutex<String>>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            current_path_str: Arc::new(Mutex::new(String::from("/home/user"))),
        }
    }

    pub fn set_current_path(&self, path: &String) {
        let mut current_path = self.current_path_str.lock().unwrap();
        *current_path = path.clone();
    }

    pub fn current_path(&self) -> String {
        let current_path = self.current_path_str.lock().unwrap().to_string();
        current_path
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8474)))
        .attach(Template::fairing())
        .manage(Config::new())
        .mount("/", routes![endpoints::change_path::change_path])
        .mount("/", routes![endpoints::index::index])
}
