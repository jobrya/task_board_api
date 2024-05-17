use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
use actix_web::web::Data;
use crate::data::service;
// use crate::models::board::Board;
// use crate::models::board::BoardRequest;
// use crate::models::task_group::TaskGroup;
// use crate::models::task::Task;
//use crate::models::account::Account;

#[test]
fn test_add() {
    assert_eq!(1 + 2, 3);
}

// #[test]
// fn test_get_board() {
//     let board: Board = service::get_board();
//     let json = board.to_json();
//     println!("{}", json);
//     assert_eq!(1 + 2, 3);
// }

// #[test]
// fn test_get_task_groups() {
//     let _task_groups: Vec<TaskGroup> = service::get_task_groups();
//     assert_eq!(1 + 2, 3);
// }

// #[test]
// fn test_get_tasks() {
//     let _tasks: Vec<Task> = service::get_tasks();
//     assert_eq!(1 + 2, 3);
// }

#[actix_web::test]
async fn test_db_up() {
    let pool = init_db_pool().await;
    let data_pool = Data::new(pool);
    let _db = service::db_up(data_pool).await;
    assert_eq!(1 + 2, 3);
}

#[actix_web::test]
async fn test_add_board() {
    let pool = init_db_pool().await;
    let data_pool = Data::new(pool);
    let _db = service::add_board(data_pool, 3).await;
    assert_eq!(1 + 2, 3);
}

async fn init_db_pool() -> Pool<Postgres> {
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/postgres-db")
        .await
        .expect("failed to create connection pool");
    return pool;
}