//    IMPORTS    \\
mod rail_network;
mod handler;
mod domain;

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
use std::error::Error;
use handler::{start_handler, search_handler, get_cities, get_trains};
use crate::domain::{ItineraryResponse, Route as DomainRoute, Train as DomainTrain, Day};
use rail_network::{RailNetwork};



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    // parse CSV and create RailNetwork instance once (example)
    //let routes = parse_CSV().expect("Failed to parse CSV");
    let rn = RailNetwork::new()?;

    // share RailNetwork via Arc for handler state
    let shared_rn = Arc::new(rn);
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            // tweak levels to taste:
            "info,axum=debug,tower_http=debug",
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Configure CORS to allow requests from your frontend origin
    let cors = CorsLayer::new()
        // Allow only your frontend origin - replace with actual origin as needed
        .allow_origin(Any)
        // Allow POST and GET methods you use
        .allow_methods(vec![Method::GET, Method::POST])
        // Allow common headers you need
        .allow_headers(Any);

    let app = axum::Router::new()
        .route("/handler/search", post(search_handler))
        .route("/handler/get", post(start_handler))
        .route("/handler/getCities", post(get_cities))
        .route("/handler/getTrains", post(get_trains))
        .with_state(shared_rn)
        .layer(cors).layer(TraceLayer::new_for_http())
    let addr = SocketAddr::from(([127, 0, 0, 1], 4001));

    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .http1_max_buf_size(64 * 1024) 
        .serve(app.into_make_service())
        .await;

    Ok(())
}
