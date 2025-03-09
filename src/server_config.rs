#[allow(deprecated)]
use std::env::home_dir;
use std::sync::{Arc, Mutex};
use serde::Serialize;

pub struct ServerConfig {
    pub current_path: Arc<Mutex<String>>,
}

#[derive(Serialize)]
pub struct ServerConfigSerialized {
    current_path: String,
}

impl ServerConfig {
    pub fn new() -> Self {
        #[allow(deprecated)]
        let home_path = home_dir().unwrap().to_str().unwrap().to_string();
        Self {
            current_path: Arc::new(Mutex::new(home_path)),
        }
    }

    pub fn get_current_path(&self) -> String {
        self.current_path.lock().unwrap().clone()
    }

    pub fn serialize(&self) -> ServerConfigSerialized {
        ServerConfigSerialized {
            current_path: self.get_current_path(),
        }
    }
}