use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use utoipa::{OpenApi, ToSchema};

// use surrealdb::engine::remote::ws::Ws;
// use surrealdb::opt::auth::Root;
// use surrealdb::sql::Thing;
// use surrealdb::Surreal;

#[derive(OpenApi)]
#[openapi(paths(get_user, create_user), components(schemas(User, Success)))]
struct ApiDoc;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct User {
    email: String,
    name: Option<String>,
    password: String,
}

#[derive(Debug, Serialize, ToSchema)]
struct Success {
    status: String,
    message: Option<String>,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let apidoc = ApiDoc::openapi().to_pretty_json().unwrap();
    match std::fs::write("openapi.json", apidoc) {
        Ok(_) => println!("{}", "Saved ApiDoc".green().bold()),
        Err(err) => println!("{}: {}", "Failed to save ApiDoc".red(), err),
    }
    // println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    // db.signin(Root {
    //     username: "root",
    //     password: "root",
    // })
    // .await?;

    // db.use_ns("test").use_db("test").await?;

    let app = Router::new()
        .route("/user/:name", get(get_user))
        .route("/create/user", post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[utoipa::path(
    tag = "User",
    get,
    path = "/user/:name",
    responses(
        (status = 200, description = "User found successfully"),
        (status = NOT_FOUND, description = "User not found")
    ),
    params(
        ("name" = String, description = "User's name")
    )
)]
async fn get_user(Path(name): Path<String>) -> impl IntoResponse {
    Json(name.to_string())
}

#[utoipa::path(
    tag = "User",
    post,
    path = "/create/user",
    responses(
        (status = 200, description = "User created successfully", body = Success),
        (status = 400, description = "Creating new user failed", body = Success)
    ),
    request_body = User
)]
async fn create_user(Json(_data): Json<User>) -> impl IntoResponse {
    Json("ok")
}
