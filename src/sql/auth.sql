SELECT account_id, username
FROM task_board.account
WHERE username = $1
AND pass = $2