#![allow(unused)]

pub async fn setup() -> sqlx::PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&dotenvy::var("DATABASE_URL").expect("DATABASE_URL env is required")).await
        .expect("failed to connect database")
}

pub fn setup_lazy() -> sqlx::PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&dotenvy::var("DATABASE_URL").expect("DATABASE_URL env is required"))
        .expect("failed to connect database")
}


impl From<sqlx::Error> for crate::libs::errors::Error {
    fn from(value: sqlx::Error) -> Self {
        let Some(constraint) = value.as_database_error().and_then(|db_err|db_err.constraint()) else {
            return Self::fatal(value);
        };

        if false == constraint.ends_with("_key") {
            return Self::fatal(value);
        }

        let ct = constraint.split("_").take(2).collect::<Vec<&str>>();

        // NOTE: bad format with underscored field
        return Self::Duplicate(ct[1].to_string());
    }
}


