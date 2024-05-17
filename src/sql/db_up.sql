DROP SCHEMA IF EXISTS task_board CASCADE;
CREATE SCHEMA task_board;

CREATE TABLE task_board.account (
    account_id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    pass TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE task_board.board (
    board_id SERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES task_board.account(account_id),
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE task_board.task_group (
    task_group_id SERIAL PRIMARY KEY,
    board_id INTEGER REFERENCES task_board.board(board_id),
    group_name TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE task_board.task (
    task_id SERIAL PRIMARY KEY,
    task_group_id INTEGER REFERENCES task_board.task_group(task_group_id),
    task_text TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);