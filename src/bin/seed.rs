use std::sync::Arc;
use sqlx::*;
use cluster::libs::errors::Result;
use serde_json::Value;
use cluster::entity::users::{Role, self};
use cluster::entity::warehouses::{WhType, self};
use cluster::entity::employees::{Employees, self};


#[tokio::main]
async fn main() {
    let db = cluster::libs::database::setup().await;
    seed(db).await;
}

async fn seed(db: PgPool) {
    let db = Arc::new(db);

    sqlx::query("DELETE FROM employees").execute(&*db).await.unwrap();
    sqlx::query("DELETE FROM warehouses").execute(&*db).await.unwrap();
    sqlx::query("DELETE FROM users").execute(&*db).await.unwrap();

    let _ = seed_users(db.clone()).await.map_err(log);
    let _ = seed_wh(db.clone()).await.map_err(log);
}

pub async fn seed_users(db: Arc<PgPool>) -> Result<()> {

    let passwd = cluster::libs::password::hash(b"passwd123")?;
    let f = vec![
        ("John".to_string(), "089636632566".to_string(), passwd.clone(), Role::Customer, Value::default()),
        ("Yeet".into(), "089636632565".into(), passwd.clone(), Role::Customer, Value::default()),
        ("Mark".into(), "089636632564".into(), passwd.clone(), Role::Customer, Value::default()),
        ("Morp".into(), "089636632563".into(), passwd.clone(), Role::Customer, Value::default()),

        ("Connor".into(), "089636632568".into(), passwd.clone(), Role::Sales, Value::default()),
        ("Robin".into(), "089636632567".into(), passwd.clone(), Role::Sales, Value::default()),
        ("Lilian".into(), "089636632569".into(), passwd.clone(), Role::Sales, Value::default()),
        ("Gengar".into(), "089636632570".into(), passwd.clone(), Role::Sales, Value::default()),
    ];

    for (name,phone,password,role,metadata) in f.into_iter() {
        let _ = users::create(&*db, &users::Data {
            name, phone, password, role, metadata,
        }).await?;
    }

    Ok(())
}

pub async fn seed_wh(db: Arc<PgPool>) -> Result<()> {

    let f = vec![
        ("Speed Warehouse".to_string(), WhType::Warehouse),
        ("Click Warehouse".into(), WhType::Warehouse),
        ("Rolling Warehouse".into(), WhType::Warehouse),
        ("Flip Counter".into(), WhType::Warehouse),
    ];

    let mut ids = vec![];

    for (name,wh_type) in f.into_iter() {
        let wh = warehouses::create(&*db, &warehouses::Data {
            name, wh_type
        }).await?;
        ids.push(wh.wh_id);
    }

    let slss = users::list(&*db, 10, 0)
        .await?
        .into_iter()
        .filter(|e|e.data.role == users::Role::Sales);

    let slss = slss.take(3).map(|e|e.user_id).collect::<Vec<i32>>();

    let data = Employees { user_id: slss[0], wh_id: ids[0] };
    let _ = employees::create(&*db, &data).await?;
    let data = Employees { user_id: slss[1], wh_id: ids[1] };
    let _ = employees::create(&*db, &data).await?;
    let data = Employees { user_id: slss[2], wh_id: ids[2] };
    let _ = employees::create(&*db, &data).await?;



    // slss


    Ok(())
}

fn log<D>(value: D) where D: std::fmt::Display {
    println!("Error: {value}");
}

