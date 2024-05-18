use crate::libs::prelude::*;
use crate::libs::prelude_router::*;
use crate::libs::prelude_entity::*;
use crate::entity::users::{Users, Role, self};
use crate::entity::orders::{Orders, Address, self};
use crate::entity::tracings::{Tracings, self};
use crate::entity::manifests::{Manifests, self};

mod create_order;
mod list_orders;
mod gateout;
mod gatein;
mod complete;
mod find_order;

pub fn routes() -> Router {
    Router::new()
        .nest("/v1", v1())
        .nest("/v2", v2())
}

fn v1() -> Router {
    Router::new()
        .route("/orders", get(list_orders::handle))
        .route("/orders", post(create_order::handle))
        .route("/orders/:id", get(find_order::handle))
        .route("/gateout", post(gateout::handle))
        .route("/gatein", post(gatein::handle))
        .route("/complete", post(complete::handle))
}

fn v2() -> Router {
    Router::new()
        .route("/orders", get(list_orders::handle))
}

async fn sudo<'r,E>(db: E, user: &Users) -> Result<()> where E: Executor<'r,Database = Postgres> {
    let Some(db_user) = users::find_by_id(db, user.user_id).await? else {
        return Err(Error::TokenExpired);
    };

    if user.updated_at != db_user.updated_at {
        return Err(Error::TokenExpired);
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Snapshot {
    Id { user_id: i32 },
    Anon { name: String, phone: String },
}

async fn snapshot<'r,E>(db: E, snapshot: &Snapshot, role: Role) -> sqlx::Result<(i32,String)> where E: Executor<'r, Database = Postgres> {
    match snapshot {
        Snapshot::Anon { name, phone } => users::snapshot_anon(db , &name, &phone, &role).await,
        Snapshot::Id { user_id } => users::snapshot_user(db, user_id).await,
    }
}




