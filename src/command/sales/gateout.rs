use super::*;

#[derive(Debug, Deserialize)]
pub struct GateoutParam {
    driver: Snapshot,
    order_ids: Vec<i32>,
    wh_to_id: i32,
}

pub async fn handle(
    Extension(db): Extension<Arc<PgPool>>,
    Auth(sales): Auth<Sales>,
    Json(data): Json<GateoutParam>
) -> Result<Json<Manifests>> {
    sudo(&*db, &sales.0.user).await?;

    let wh = sales.data()?;

    let mut tx = db.begin().await?;

    let (driver_snapshot,driver_name) = snapshot(&mut *tx, &data.driver, Role::Driver).await?;

    for order_id in data.order_ids.clone() {
        Tracings::archive(&mut *tx, order_id).await?;

        let data = tracings::Data {
            order_id,
            status: tracings::Status::Driver,
            subject_id: driver_snapshot,
            subject_name: driver_name.clone(),
        };

        Tracings::create(&mut *tx, &data).await?;
    }

    let manifest = manifests::Data {
        sales_id: sales.0.user.user_id,
        driver_id: driver_snapshot,
        wh_from_id: wh.wh_id,
        wh_to_id: data.wh_to_id,
    };

    let manifest = Manifests::create(&mut *tx, &manifest).await?;

    for order_id in data.order_ids {
        manifests::ManifestOrders::create(&mut *tx, order_id, manifest.manifest_id).await?;
    }

    tx.commit().await?;

    Ok(Json(manifest))
}


