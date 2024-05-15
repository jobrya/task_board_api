use actix_web::{get, post, delete, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
use crate::data::service;
mod data;
mod models;
#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/postgres-db")
        .await
        .expect("failed to create database connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(db_up)
            .service(db_down)
            .service(auth)
            .service(post_account)
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

#[get("/admin/dbUp")]
async fn db_up(data_pool: Data<Pool<Postgres>>) -> impl Responder {
    let db_up_result = service::db_up(data_pool).await;
    match db_up_result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("database initialized - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[get("/admin/dbDown")]
async fn db_down(data_pool: Data<Pool<Postgres>>) -> impl Responder {
    let db_down_result = service::db_down(data_pool).await;
    match db_down_result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("database dropped - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[post("/auth")]
async fn auth(user_name: String, password: String) -> impl Responder {
    //let _account = service::get_account(user_name, password);
    HttpResponse::Ok().body("authenticated")
}

// CRUD operations: Create, Read, Update, & Delete

// Create
#[post("/addAccount")] 
async fn post_account(data_pool: Data<Pool<Postgres>>, username: String, password: String) 
    -> impl Responder {
    let add_account_result = service::add_account(data_pool, username, password).await;
    match add_account_result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("account inserted - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

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