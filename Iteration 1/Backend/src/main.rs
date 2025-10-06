//    IMPORTS    \\
mod rail_network;
mod handler;
mod search_functionality;

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
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::net::SocketAddr;
use handler::{start_handler, search_handler};
use rail_network::parse_CSV;
use crate::search_functionality::{RailNetwork, SearchFunctionality, TicketClass, SortBy};


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

    // parse CSV and create RailNetwork instance once (example)
    let routes = parse_CSV().expect("Failed to parse CSV");
    let rn = RailNetwork::new(routes);

    // share RailNetwork via Arc for handler state
    let shared_rn = std::sync::Arc::new(rn);

    let app = Router::new()
        .route("/handler/search", post(search_handler))
        .route("/handler/get", get(start_handler))
        .with_state(shared_rn)
        .layer(ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
