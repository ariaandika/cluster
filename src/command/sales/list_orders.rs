use super::*;

#[derive(Debug, Serialize, FromRow)]
pub struct OrdersInWarehouse {
    #[sqlx(flatten)]
    pub tracings: tracings::Tracings,
    pub destination: Address,
}

const QUERY: &str = r#"
SELECT
    t.*,
    o.destination
FROM tracings t
JOIN orders o ON o.order_id = t.order_id
WHERE t.status = 'Warehouse' AND t.subject_id = $1
LIMIT $2 OFFSET $3;
"#;

pub async fn handle(
    Extension(db): Extension<Arc<PgPool>>,
    Auth(sales): Auth<Sales>,
) -> Result<JsonData<Vec<OrdersInWarehouse>>> {
    let wh = sales.data()?;
    let orders = sqlx::query_as(QUERY)
        .bind(wh.wh_id).bind(50).bind(0)
        .fetch_all(&*db).await?;
    Ok(JsonData(orders))
}



// (
//     SELECT json_agg(json_build_array(s.snapshot_id,s.name))
//     FROM users_snapshot s
//     WHERE s.snapshot_id = o.sender_id OR s.snapshot_id = o.receiver_id
// ) as names
