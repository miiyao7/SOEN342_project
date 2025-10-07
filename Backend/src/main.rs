//    IMPORTS    \\
mod rail_network;
mod handler;

//     USES      \\
use axum::{
    routing::post,    
    routing::get,
    http::Method,
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;
use std::sync::Arc;
use std::net::SocketAddr;
use handler::{start_handler, search_handler, get_cities, get_trains};
use rail_network::{RailNetwork};



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
    //let routes = parse_CSV().expect("Failed to parse CSV");
    let rn = RailNetwork::new();

    // share RailNetwork via Arc for handler state
    let shared_rn = Arc::new(rn.unwrap());

    let app = Router::new()
        .route("/handler/search", post(search_handler))
        .route("/handler/get", get(start_handler))
        .route("/handler/getCities", get(get_cities))
        .route("/handler/getTrains", get(get_trains))
        .with_state(shared_rn)
        .layer(ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
