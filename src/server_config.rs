#[allow(deprecated)]
use std::env::home_dir;
use std::sync::{Arc, Mutex};

pub struct ServerConfig {
    pub current_path_str: Arc<Mutex<String>>,
}

impl ServerConfig {
    pub fn new() -> Self {
        #[allow(deprecated)]
        let home_path = home_dir().unwrap().to_str().unwrap().to_string();
        Self {
            current_path_str: Arc::new(Mutex::new(home_path)),
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