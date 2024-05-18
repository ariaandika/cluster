use super::*;

#[derive(Debug, Deserialize)]
pub struct CompleteParam {
    manifest_id: i32,
    courier: Snapshot,
}

pub async fn handle(
    Extension(db): Extension<Arc<PgPool>>,
    Auth(sales): Auth<Sales>,
    Json(data): Json<CompleteParam>
) -> Result<()> {
    sudo(&*db, &sales.0.user).await?;

    let mut tx = db.begin().await?;

    let (courier_snapshot,courier_name) = snapshot(&mut *tx, &data.courier, Role::Courier).await?;

    Manifests::complete(&*db, data.manifest_id).await?;

    let rels = manifests::ManifestOrders::list_by_manifest_id(&*db, data.manifest_id).await?;

    for rel in rels {
        let order_id = rel.order_id;

        Tracings::archive(&mut *tx, order_id).await?;

        let data = tracings::Data {
            order_id,
            status: tracings::Status::Complete,
            subject_id: courier_snapshot,
            subject_name: courier_name.clone(),
        };

        Tracings::create(&mut *tx, &data).await?;
    }

    tx.commit().await?;

    Ok(())
}


