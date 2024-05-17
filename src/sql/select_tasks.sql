SELECT
    task_id,
    task.task_group_id,
    task_text
FROM task_board.task_group
JOIN task_board.task ON task_group.task_group_id = task.task_group_id
WHERE task_group.board_id = $1