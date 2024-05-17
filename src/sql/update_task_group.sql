UPDATE task_board.task_group
SET group_name = $1
WHERE board_id = $2
AND task_group_id = $3