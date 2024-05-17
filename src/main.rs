use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
use crate::data::service;
use crate::models::account::AccountRequest;
use crate::models::task::{TaskRequest, Task};
use crate::models::task_group::{TaskGroupRequest, TaskGroup};
use crate::models::board::BoardRequest;
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
            .service(post_board)
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
    let result = service::db_up(data_pool).await;
    match result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("database initialized - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[get("/admin/dbDown")]
async fn db_down(data_pool: Data<Pool<Postgres>>) -> impl Responder {
    let result = service::db_down(data_pool).await;
    match result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("database dropped - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

// TODO: add actual auth
#[post("/auth")]
async fn auth(data_pool: Data<Pool<Postgres>>, account_request: web::Json<AccountRequest>) -> impl Responder {
    let result = service::auth(
        data_pool,
        &account_request.username,
        &account_request.password)
        .await;
    match result {
        Ok(account) => HttpResponse::Ok().body(account.to_json()),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

// CRUD operations: Create, Read, Update, & Delete

// Create
#[post("/addAccount")] 
async fn post_account(data_pool: Data<Pool<Postgres>>, account_request: web::Json<AccountRequest>) 
    -> impl Responder {
    let result = service::add_account(
        data_pool,
        &account_request.username,
        &account_request.password)
        .await;
    match result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("account inserted - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

// Create
#[post("/addTask")]
async fn post_task(data_pool: Data<Pool<Postgres>>, task_request: web::Json<TaskRequest>) -> impl Responder {
    let result = service::add_task(data_pool, task_request).await;
    match result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("task inserted - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

// Create
#[post("/addTaskGroup")]
async fn post_task_group(data_pool: Data<Pool<Postgres>>, task_group_request: web::Json<TaskGroupRequest>) -> impl Responder {
    let result = service::add_task_group(data_pool, task_group_request).await;
    match result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("task group inserted - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

// Create
#[post("/addBoard")]
async fn post_board(data_pool: Data<Pool<Postgres>>, board_request: web::Json<BoardRequest>) -> impl Responder {
    let result = service::add_board(data_pool, board_request.account_id).await;
    match result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("board inserted - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

// Read
#[get("/board")]
async fn get_board(data_pool: Data<Pool<Postgres>>, board_request: web::Json<BoardRequest>) -> impl Responder {
    let response = service::get_board(data_pool, board_request.board_id).await.to_json();
    HttpResponse::Ok().body(response)
}

// Update
#[post("/updateTask")]
async fn post_update_task(data_pool: Data<Pool<Postgres>>, task: web::Json<Task>) -> impl Responder {
    let result = service::update_task(data_pool, task).await;
    match result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("task updated - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

// Update
#[post("/updateTaskGroup")]
async fn post_update_task_group(data_pool: Data<Pool<Postgres>>, task_group: web::Json<TaskGroup>) -> impl Responder {
    let result = service::update_task_group(data_pool, task_group).await;
    match result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("task group updated - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

// Delete
#[delete("/removeTask")]
async fn delete_task(data_pool: Data<Pool<Postgres>>, task: web::Json<Task>) -> impl Responder {
    let result = service::delete_task(data_pool, task).await;
    match result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("task deleted - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

// Delete
#[delete("/removeTaskGroup")]
async fn delete_task_group(data_pool: Data<Pool<Postgres>>, task_group: web::Json<TaskGroup>) -> impl Responder {
    let result = service::delete_task_group(data_pool, task_group).await;
    match result {
        Ok(pg_query_result) => HttpResponse::Ok()
            .body(format!("task group deleted - rows affected: {}", pg_query_result.rows_affected())),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}