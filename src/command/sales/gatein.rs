use super::*;

#[derive(Debug, Deserialize)]
pub struct GateinParam {
    manifest_id: i32,
}

pub async fn handle(
    Extension(db): Extension<Arc<PgPool>>,
    Auth(sales): Auth<Sales>,
    Json(data): Json<GateinParam>
) -> Result<()> {
    sudo(&*db, &sales.0.user).await?;

    let wh = sales.data()?;

    let mut tx = db.begin().await?;

    Manifests::complete(&*db, data.manifest_id).await?;

    let rels = manifests::ManifestOrders::list_by_manifest_id(&*db, data.manifest_id).await?;

    for rel in rels {
        let order_id = rel.order_id;

        Tracings::archive(&mut *tx, order_id).await?;

        let data = tracings::Data {
            order_id,
            status: tracings::Status::Warehouse,
            subject_id: wh.wh_id,
            subject_name: wh.wh_name.clone(),
        };

        Tracings::create(&mut *tx, &data).await?;
    }

    tx.commit().await?;

    Ok(())
}


