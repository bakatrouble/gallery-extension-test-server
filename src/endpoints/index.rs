use std::path::{Path, PathBuf};
use rocket::figment::util::diff_paths;
use rocket::fs::NamedFile;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket_dyn_templates::{context, Template};
use crate::server_config::ServerConfig;
use human_sort;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Item {
    path: String,
    mime: String,
    name: String,
    is_dir: bool,
}

#[derive(Responder)]
pub enum IndexResponse {
    #[response(status = 200)]
    Html(Template),
    #[response(status = 200)]
    File(NamedFile),
    #[response(status = 404)]
    NotFound(String),
}

#[get("/<query_path..>")]
pub async fn index(query_path: PathBuf, config: &State<ServerConfig>) -> IndexResponse {
    let current_path = config.current_path();
    let root = Path::new(&current_path);
    let joined_path = root.join(query_path);

    if joined_path.is_file() {
        return IndexResponse::File(NamedFile::open(joined_path).await.unwrap());
    }

    let path = joined_path.as_path();

    if !path.is_dir() {
        return IndexResponse::NotFound("404 Not Found".to_string());
    }

    let parent = diff_paths(path.parent().unwrap_or(path), root).unwrap();
    let mut files: Vec<_> = std::fs::read_dir(&joined_path).unwrap()
        .map(|r| r.unwrap())
        .collect();
    files.sort_by(|this, other| {
        println!("{} : {}", this.file_name().to_str().unwrap(), other.file_name().to_str().unwrap());
        human_sort::compare(
            this.file_name().to_str().unwrap(),
            other.file_name().to_str().unwrap(),
        )
    });

    println!("{}", joined_path.to_str().unwrap());

    let ctx = context! {
        path: path.to_str().unwrap(),
        root: root.to_str().unwrap(),
        parent: parent.to_str().unwrap(),
        listdir: files.iter().map(|entry| {
            let path = String::from(diff_paths(entry.path(), root).unwrap().to_str().unwrap());
            let name = String::from(entry.file_name().to_str().unwrap());
            let mime = mime_guess::from_path(entry.path()).first_or_octet_stream().to_string();
            let is_dir = entry.path().is_dir();
            Item {
                path,
                mime,
                name,
                is_dir,
            }
        }).collect::<Vec<_>>()
    };

    IndexResponse::Html(
        Template::render("list", ctx)
    )
}
