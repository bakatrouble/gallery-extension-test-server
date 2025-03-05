use std::sync::{Arc, Mutex};

pub struct ServerConfig {
    pub current_path_str: Arc<Mutex<String>>,
}

impl ServerConfig {
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