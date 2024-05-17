SELECT 
    task_group_id,
    group_name
FROM task_board.task_group
WHERE board_id = $1