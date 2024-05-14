use crate::models::board::Board;
use crate::models::task_group::TaskGroup;
use crate::models::task::Task;
use crate::models::account::Account;
use postgres::{Client, NoTls};

pub fn init_db() -> Client {
    let client = Client::connect("postgres://username:password@localhost/posrgres-test", NoTls)
        .expect("failed to connect to database");
    return client;
}

pub fn get_accounts() -> Vec<Account> {
    let mut client = init_db();
    let mut accounts = Vec::new();
    for row in client
        .query("SELECT id, user_name FROM account", &[])
        .expect("query failed") {

            accounts.push(
                Account {
                    id: row.get(0),
                    user_name: row.get(1),
                }
            );
        }
    return accounts;
}

pub fn get_board() -> Board {
    Board {
        board_id: 1,
        user_id: 1,
        task_groups: get_task_groups(),
        tasks: get_tasks(),
    }
}

pub fn get_task_groups() -> Vec<TaskGroup> {
    vec![
        TaskGroup {
            task_group_id: 1,
            board_id: 1,
            text: String::from("To Do"),
        },
        TaskGroup {
            task_group_id: 2,
            board_id: 1,
            text: String::from("In Progress"),
        },
        TaskGroup {
            task_group_id: 2,
            board_id: 1,
            text: String::from("Complete"),
        },
    ]
}

pub fn get_tasks() -> Vec<Task> {
    vec![
        Task {
            task_group_id: 1,
            task_id: 1,
            text: String::from("do stuff"),
        },
        Task {
            task_group_id: 2,
            task_id: 1,
            text: String::from("do more stuff"),
        },
    ]
}
