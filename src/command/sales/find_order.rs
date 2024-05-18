use super::*;
use crate::entity::packages;

#[derive(Debug, Serialize)]
pub struct Subject {
    name: String,
    phone: String,
}

#[derive(Debug, Serialize)]
pub struct OrderDetailResponse {
    pub order: orders::Orders,
    pub sender: Subject,
    pub receiver: Subject,
    pub tracings: Vec<tracings::Tracings>,
    pub packages: Vec<packages::Packages>,
}

// const QUERY: &str = r#"
// SELECT
//     t.*,
//     o.destination as destination,
//     p.*
// FROM orders o
// JOIN tracings t ON t.order_id = o.order_id
// JOIN packages p ON p.order_id = o.order_id
// JOIN users_snapshot u ON u.snapshot_id = o.sender_id OR u.snapshot_id = o.receiver_id
// WHERE o.order_id = $1;
// "#;
// (
//     SELECT json_agg(json_build_array(s.snapshot_id,s.name))
//     FROM users_snapshot s
//     WHERE s.snapshot_id = o.sender_id OR s.snapshot_id = o.receiver_id
// ) as names

pub async fn handle(
    Extension(db): Extension<Arc<PgPool>>,
    Auth(_): Auth<Sales>,
    Path(id): Path<i32>,
) -> Result<JsonData<Option<OrderDetailResponse>>> {
    let Some(order) = orders::Orders::find_by_id(&*db, id).await? else {
        return Ok(JsonData(None));
    };

    let tracings_data = tracings::Tracings::list_by_order_id(&*db, order.order_id).await?;
    let pkgs = packages::Packages::list_by_order_id(&*db, order.order_id).await?;
    let Some(sender) = users::Snapshot::find_by_id(&*db, order.sender_id).await? else {
        return Ok(JsonData(None));
    };
    let Some(receiver) = users::Snapshot::find_by_id(&*db, order.receiver_id).await? else {
        return Ok(JsonData(None));
    };

    let response = OrderDetailResponse {
        order,
        tracings: tracings_data,
        packages: pkgs,
        sender: sender.into(),
        receiver: receiver.into(),
    };

    Ok(JsonData(Some(response)))
}

impl From<users::Snapshot> for Subject {
    fn from(value: users::Snapshot) -> Self {
        Self { name: value.name, phone: value.phone }
    }
}

