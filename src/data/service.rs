use std::fs;
use actix_web::web::{Data, Json};
use sqlx::{postgres, Postgres, Pool, Row};
use futures::TryStreamExt;
use crate::models::board::Board;
use crate::models::task_group::TaskGroup;
use crate::models::task_group::TaskGroupRequest;
use crate::models::task::Task;
use crate::models::task::TaskRequest;
use crate::models::account::Account;

pub async fn db_up(data_pool: Data<Pool<Postgres>>) -> Result<postgres::PgQueryResult, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/db_up.sql")
        .expect("failed to read db_up.sql");
    let pool_ref = data_pool.get_ref();
    let query_result = sqlx::raw_sql(&sql)
        .execute(pool_ref)
        .await;
    return query_result;
}

pub async fn db_down(data_pool: Data<Pool<Postgres>>) -> Result<postgres::PgQueryResult, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/db_down.sql")
        .expect("failed to read db_down.sql");
    let pool_ref = data_pool.get_ref();
    let query_result = sqlx::raw_sql(&sql)
        .execute(pool_ref)
        .await;
    return query_result;
}

pub async fn auth(data_pool: Data<Pool<Postgres>>, username: &String, password: &String) 
    -> Result<Account, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/auth.sql")
        .expect("failed to read auth.sql");
    let pool_ref = data_pool.get_ref();
    let row_result = sqlx::query_as::<_, Account>(&sql)
        .bind(username)
        .bind(password)
        .fetch_one(pool_ref)
        .await;
    return row_result;
}

pub async fn add_account(data_pool: Data<Pool<Postgres>>, username: &String, password: &String) 
    -> Result<postgres::PgQueryResult, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/insert_account.sql")
        .expect("failed to read insert_account.sql");
    let pool_ref = data_pool.get_ref();
    let query_result = sqlx::query(&sql)
        .bind(username)
        .bind(password)
        .execute(pool_ref)
        .await;
    return query_result;
}

pub async fn add_task(data_pool: Data<Pool<Postgres>>, task_request: Json<TaskRequest>) 
    -> Result<postgres::PgQueryResult, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/insert_task.sql")
        .expect("failed to read insert_task.sql");
    let pool_ref = data_pool.get_ref();
    let query_result = sqlx::query(&sql)
        .bind(task_request.task_group_id)
        .bind(&task_request.task_text)
        .execute(pool_ref)
        .await;
    return query_result;
}

pub async fn add_task_group(data_pool: Data<Pool<Postgres>>, task_group_request: Json<TaskGroupRequest>) 
    -> Result<postgres::PgQueryResult, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/insert_task_group.sql")
        .expect("failed to read insert_task_group.sql");
    let pool_ref = data_pool.get_ref();
    let query_result = sqlx::query(&sql)
        .bind(task_group_request.board_id)
        .bind(&task_group_request.group_name)
        .execute(pool_ref)
        .await;
    return query_result;
}

pub async fn add_board(data_pool: Data<Pool<Postgres>>, account_id: i32) 
    -> Result<postgres::PgQueryResult, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/insert_board.sql")
        .expect("failed to read insert_board.sql");
    let pool_ref = data_pool.get_ref();
    let query_result = sqlx::query(&sql)
        .bind(account_id)
        .execute(pool_ref)
        .await;
    return query_result;
}

pub async fn get_board(data_pool: Data<Pool<Postgres>>, board_id: i32) -> Board {
    return Board {
        board_id: board_id,
        account_id: 1,
        task_groups: get_task_groups(&data_pool, board_id).await,
        tasks: get_tasks(&data_pool, board_id).await,
    }
}

pub async fn get_task_groups(data_pool: &Data<Pool<Postgres>>, board_id: i32) -> Vec<TaskGroup> {
    let mut task_groups = Vec::new(); 
    let sql = fs::read_to_string("./src/sql/select_task_groups.sql")
        .expect("failed to read select_task_groups.sql");
    let pool_ref = data_pool.get_ref();
    let mut rows = sqlx::query(&sql)
        .bind(board_id)
        .fetch(pool_ref);
    while let Some(row) = rows.try_next().await.unwrap() {
        task_groups.push(
            TaskGroup {
                board_id: board_id,
                task_group_id: row.get("task_group_id"),
                group_name: row.get("group_name"),
            }
        );
    }
    return task_groups;
}

pub async fn get_tasks(data_pool: &Data<Pool<Postgres>>, board_id: i32) -> Vec<Task> {
    let mut tasks = Vec::new();
    let sql = fs::read_to_string("./src/sql/select_tasks.sql")
        .expect("failed to read select_tasks.sql");
    let pool_ref = data_pool.get_ref();
    let mut rows = sqlx::query(&sql)
        .bind(board_id)
        .fetch(pool_ref);
    while let Some(row) = rows.try_next().await.unwrap() {
        tasks.push(
            Task {
                task_id: row.get("task_id"),
                task_group_id: row.get("task_group_id"),
                task_text: row.get("task_text"),
            }
        );
    }
    return tasks;
}

pub async fn update_task(data_pool: Data<Pool<Postgres>>, task: Json<Task>) 
    -> Result<postgres::PgQueryResult, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/update_task.sql")
        .expect("failed to read update_task.sql");
    let pool_ref = data_pool.get_ref();
    let query_result = sqlx::query(&sql)
        .bind(&task.task_text)
        .bind(task.task_group_id)
        .bind(task.task_id)
        .execute(pool_ref)
        .await;
    return query_result;
}

pub async fn update_task_group(data_pool: Data<Pool<Postgres>>, task_group: Json<TaskGroup>) 
    -> Result<postgres::PgQueryResult, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/update_task_group.sql")
        .expect("failed to read update_task_group.sql");
    let pool_ref = data_pool.get_ref();
    let query_result = sqlx::query(&sql)
        .bind(&task_group.group_name)
        .bind(task_group.board_id)
        .bind(task_group.task_group_id)
        .execute(pool_ref)
        .await;
    return query_result;
}

pub async fn delete_task(data_pool: Data<Pool<Postgres>>, task: Json<Task>) 
    -> Result<postgres::PgQueryResult, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/delete_task.sql")
        .expect("failed to read delete_task.sql");
    let pool_ref = data_pool.get_ref();
    let query_result = sqlx::query(&sql)
        .bind(task.task_group_id)
        .bind(task.task_id)
        .execute(pool_ref)
        .await;
    return query_result;
}

pub async fn delete_task_group(data_pool: Data<Pool<Postgres>>, task_group: Json<TaskGroup>) 
    -> Result<postgres::PgQueryResult, sqlx::Error> {
    let sql = fs::read_to_string("./src/sql/delete_task_group.sql")
        .expect("failed to read delete_task_group.sql");
    let pool_ref = data_pool.get_ref();
    let query_result = sqlx::query(&sql)
        .bind(task_group.board_id)
        .bind(task_group.task_group_id)
        .execute(pool_ref)
        .await;
    return query_result;
}