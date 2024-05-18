use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_|{
                "serve=trace,cluster=trace,sqlx=trace,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = cluster::libs::database::setup().await;
    let arc = std::sync::Arc::new(db);

    let root = cluster::command::routes().layer(axum::Extension(arc));
    let tcp = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("Listening in http://localhost:3000");
    axum::serve(tcp, root).await.unwrap();
}
