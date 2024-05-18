use crate::libs::prelude_router::*;
// use tower_http::cors;
// use axum::http::{method::Method, header};

mod auth;
mod sales;

pub fn routes() -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/sales", sales::routes())
        .layer(axum::middleware::from_fn(cors_layer))
        // .layer(cors::CorsLayer::new()
        //        .allow_methods([Method::GET,Method::POST])
        //        .allow_credentials(true)
        //        .allow_origin(["http://localhost:5173".parse().unwrap()])
        //        .allow_headers([header::CONTENT_TYPE]))
}

async fn cors_layer(req: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let origin = req.headers().get(axum::http::header::ORIGIN).cloned();
    let mut res = next.run(req).await;
    let Some(origin) = origin else { return res };

    if origin == "http://localhost:5173" {
        res.headers_mut().append("Access-Control-Allow-Credentials","true".parse().unwrap());
        res.headers_mut().append("Access-Control-Allow-Origin","http://localhost:5173".parse().unwrap());
    }
    if origin == "https://staff.banter.id" {
        res.headers_mut().append("Access-Control-Allow-Credentials","true".parse().unwrap());
        res.headers_mut().append("Access-Control-Allow-Origin","https://staff.banter.id".parse().unwrap());
    }

    res
}
