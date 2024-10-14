use actix_web::{web, HttpResponse, Responder};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;

// Define the shared state for the path
pub struct AppState {
    base_path: PathBuf,
}

impl AppState {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
        }
    }

    pub fn index(&self) -> PathBuf {
        self.base_path.join("index.html")
    }

    pub fn pages(&self, path: &str) -> PathBuf {
        let page = self.base_path.join(path);
        let index = page.clone().join("index.html");
        let html = page.clone().with_extension("html");
        if PathBuf::from(&index).exists() {
            index
        } else if PathBuf::from(&html).exists() {
            html
        } else {
            page
        }
    }
}

// The `idx` handler that uses the dynamic path from the state
pub async fn idx(state: web::Data<Arc<AppState>>) -> impl Responder {
    match fs::read(state.index()).await {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(_) => HttpResponse::InternalServerError().body("Could not read file"),
    }
}

// Another handler that uses the dynamic path (e.g., for pages)
pub async fn pages(state: web::Data<Arc<AppState>>, path: web::Path<String>) -> impl Responder {
    match fs::read(state.pages(&path)).await {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(e) => {
            println!("{:?}", e);
            println!("{:?}", state.pages(&path));
            println!("{:?}", path);
            HttpResponse::InternalServerError().body("Page not found")
        }
    }
}
