use actix_web::{get, web, web::ServiceConfig, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use std::sync::Mutex;

struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntry>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TodolistEntry {
    id: i32,
    date: i64,
    title: String,
}


#[get("/")]
async fn hello_world() -> impl Responder {
    let a = TodolistEntry {
        id: 1,
        title: "Source".to_string(),
        date: 1612396800,
    };
    HttpResponse::Ok().json(a)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}
