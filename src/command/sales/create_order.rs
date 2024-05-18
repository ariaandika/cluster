use super::*;
use crate::entity::packages::{Packages, self};


#[derive(Debug, Deserialize)]
pub struct PackageDto {
    pub name: String,
    pub weight: f32,
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
    order_id: i32
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderParam {
    sender: Snapshot,
    receiver: Snapshot,
    destination: Address,
    packages: Vec<PackageDto>
}

pub async fn handle(
    Extension(db): Extension<Arc<PgPool>>,
    Auth(sales): Auth<Sales>,
    Json(data): Json<CreateOrderParam>
) -> Result<Json<CreateOrderResponse>> {
    sudo(&*db, &sales.0.user).await?;

    let wh = sales.data()?;

    let (sender_snapshot,_sender_name) = snapshot(&*db, &data.sender, Role::Customer).await?;
    let (receiver_snapshot,_receiver_name) = snapshot(&*db, &data.receiver, Role::Customer).await?;

    let mut tx = db.begin().await?;

    let order = orders::Data {
        sender_id: sender_snapshot,
        receiver_id: receiver_snapshot,
        destination: data.destination,
    };
    let order = Orders::create(&mut *tx, &order).await?;
    let order_id = order.order_id;

    let tracing = tracings::Data {
        order_id,
        status: tracings::Status::Warehouse,
        subject_id: wh.wh_id,
        subject_name: wh.wh_name,
    };

    Tracings::create(&mut *tx, &tracing).await?;

    for pkg in data.packages {
        Packages::create(&mut *tx, &pkg.into(order_id)).await?;
    }

    tx.commit().await?;

    Ok(Json(CreateOrderResponse { order_id }))
}

impl PackageDto {
    fn into(self, order_id: i32) -> packages::Data {
        packages::Data {
            order_id, name: self.name, width: self.width,
            weight: self.weight, length: self.length, height: self.height,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn test_create_order(db: PgPool) {

        let input = CreateOrderParam {
            sender: Snapshot::Anon {
                name: "John".into(),
                phone: "089636632567".into(),
            },
            receiver: Snapshot::Anon {
                name: "Alex".into(),
                phone: "089636632568".into(),
            },
            destination: Address {
                detail: "Jl jalan jalan".into(),
                kelurahan: "Sendangguwo".into(),
                kecamatan: "Tembalang".into(),
                kabupaten: "Semarang".into(),
                provinsi: "Jawa Tengah".into(),
                kodepos: 50703,
            },
            packages: vec![
                PackageDto {
                    name: "Book".into(),
                    weight: 20.,
                    height: 10.,
                    length: 13.,
                    width: 8.
                },
                PackageDto {
                    name: "Pen".into(),
                    weight: 4.,
                    height: 8.,
                    length: 6.,
                    width: 4.
                },
            ]
        };

        // let auth = Auth();

        // handle(Extension(Arc::new(db)), )
    }
}

