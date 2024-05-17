UPDATE task_board.task
SET task_text = $1
WHERE task_group_id = $2
AND task_id = $3