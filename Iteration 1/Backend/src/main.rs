//    IMPORTS    \\
mod rail_network;
mod handler;

//     USES      \\
use axum::{
    routing::post,    
    routing::get,
    extract::Multipart,
    http::Method,
    Json, Router,
};
use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use handler::{get_enum_cities, get_enum_trains, get_enum_days, upload_csv};

#[tokio::main]
async fn main() {
    // Configure CORS to allow requests from your frontend origin
    let cors = CorsLayer::new()
        // Allow only your frontend origin - replace with actual origin as needed
        .allow_origin(Any)
        // Allow POST and GET methods you use
        .allow_methods(vec![Method::GET, Method::POST])
        // Allow common headers you need
        .allow_headers(Any);

    let app = Router::new()
        .route("/upload", post(upload_csv))
        .route("/handler/cities", get(get_enum_cities))
        .route("/handler/trains", get(get_enum_trains))
        .route("/handler/days", get(get_enum_days))
        // Attach CORS middleware layer
        .layer(ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
