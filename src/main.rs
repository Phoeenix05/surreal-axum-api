use axum::{routing::get, Router, extract::Query};
use utoipa::OpenApi;
use std::net::SocketAddr;

#[derive(OpenApi)]
#[openapi(paths(get_user))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
    
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = Router::new().route("/user/:name", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[utoipa::path(
    get,
    path = "/user/:name",
    responses(
        (status = 200, description = "User found succesfully"),
        (status = NOT_FOUND, description = "User not found")
    ),
    params(
        ("name" = String, description = "User's name")
    )
)]
async fn get_user(name: Option<Query<String>>) -> &'static str {
    "Hello, World!"
}
