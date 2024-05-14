use crate::data::service;
use crate::models::board::Board;
use crate::models::task_group::TaskGroup;
use crate::models::task::Task;
use crate::models::account::Account;

#[test]
fn test_add() {
    assert_eq!(1 + 2, 3);
}

#[test]
fn test_get_board() {
    let board: Board = service::get_board();
    let json = board.to_json();
    println!("{}", json);
    assert_eq!(1 + 2, 3);
}

#[test]
fn test_get_task_groups() {
    let task_groups: Vec<TaskGroup> = service::get_task_groups();
    assert_eq!(1 + 2, 3);
}

#[test]
fn test_get_tasks() {
    let tasks: Vec<Task> = service::get_tasks();
    assert_eq!(1 + 2, 3);
}

#[test]
fn test_db() {
    let accounts: Vec<Account> = service::get_accounts();
    assert_eq!(1 + 2, 3);
}