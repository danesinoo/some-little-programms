use actix_web::{web, HttpResponse, Responder};
use std::path::PathBuf;

pub async fn pages(path: web::Path<String>) -> impl Responder {
    let content = std::fs::read(get_link(&path.into_inner())).unwrap();
    HttpResponse::Ok().body(content)
}

pub async fn idx() -> impl Responder {
    let content = std::fs::read_to_string("index.html").unwrap();
    HttpResponse::Ok().body(content)
}

fn get_link(path: &str) -> String {
    let page = "./".to_string() + path;
    let index = page.clone() + "/index.html";
    let html = page.clone() + ".html";

    if PathBuf::from(index).exists() {
        page + "/index.html"
    } else if PathBuf::from(html).exists() {
        page + ".html"
    } else {
        page
    }
}
