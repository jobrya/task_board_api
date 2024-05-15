use std::fs;
use actix_web::web::Data;
use sqlx::{postgres, Postgres, Pool};
use crate::models::board::Board;
use crate::models::task_group::TaskGroup;
use crate::models::task::Task;
//use crate::models::account::Account;

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

pub async fn add_account(data_pool: Data<Pool<Postgres>>, username: String, password: String) 
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

pub fn get_board() -> Board {
    Board {
        board_id: 1,
        account_id: 1,
        task_groups: get_task_groups(),
        tasks: get_tasks(),
    }
}

pub fn get_task_groups() -> Vec<TaskGroup> {
    vec![
        TaskGroup {
            task_group_id: 1,
            board_id: 1,
            group_name: String::from("To Do"),
        },
        TaskGroup {
            task_group_id: 2,
            board_id: 1,
            group_name: String::from("In Progress"),
        },
        TaskGroup {
            task_group_id: 2,
            board_id: 1,
            group_name: String::from("Complete"),
        },
    ]
}

pub fn get_tasks() -> Vec<Task> {
    vec![
        Task {
            task_group_id: 1,
            task_id: 1,
            task_text: String::from("do stuff"),
        },
        Task {
            task_group_id: 2,
            task_id: 1,
            task_text: String::from("do more stuff"),
        },
    ]
}
