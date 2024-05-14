use actix_web::{get, post, delete, App, HttpResponse, HttpServer, Responder};
use crate::data::service;
mod data;
mod models;
#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(post_task)
            .service(post_task_group)
            .service(get_board)
            .service(post_update_task)
            .service(post_update_task_group)
            .service(delete_task)
            .service(delete_task_group)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// CRUD operations: Create, Read, Update, & Delete

// Create
#[post("/addTask")]
async fn post_task() -> impl Responder {
    HttpResponse::Ok().body("added a task")
}

// Create
#[post("/addTaskGroup")]
async fn post_task_group() -> impl Responder {
    HttpResponse::Ok().body("added a task group")
}

// Read
#[get("/board")]
async fn get_board() -> impl Responder {
    let response = service::get_board().to_json();
    HttpResponse::Ok().body(response)
}

// Update
#[post("/updateTask")]
async fn post_update_task() -> impl Responder {
    HttpResponse::Ok().body("updated a task")
}

// Update
#[post("/updateTask")]
async fn post_update_task_group() -> impl Responder {
    HttpResponse::Ok().body("updated a task group")
}

// Delete
#[delete("/removeTask")]
async fn delete_task() -> impl Responder {
    HttpResponse::Ok().body("deleted a task")
}

// Delete
#[delete("/removeTaskGroup")]
async fn delete_task_group() -> impl Responder {
    HttpResponse::Ok().body("deleted a task group")
}