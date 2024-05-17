DELETE 
FROM task_board.task
WHERE task_group_id = $1
AND task_id = $2