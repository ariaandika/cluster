SELECT
    t.*,
    o.sender_id,
    o.receiver_id,
    (
        SELECT
            json_agg(json_build_array(s.snapshot_id,s.name))
        FROM users_snapshot s
        WHERE
            s.snapshot_id = o.sender_id
            OR s.snapshot_id = o.receiver_id
    ) as names
FROM tracings t
JOIN orders o ON o.order_id = t.order_id
WHERE t.status = 'Warehouse' AND t.subject_id = $1
LIMIT $2 OFFSET $3;

