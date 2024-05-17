DELETE
FROM task_board.task_group
WHERE board_id = $1
AND task_group_id = $2