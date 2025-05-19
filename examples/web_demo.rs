use actix_web::{get, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[get("/todos/{id}")]
async fn get_todo(path: actix_web::web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let todo = Todo {
        id,
        title: format!("Todo {}", id),
        completed: false,
    };
    HttpResponse::Ok().json(&todo)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        actix_web::App::new()
            .service(get_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}