//    IMPORTS    \\
mod rail_network;
mod handler;
mod domain;

//     USES      \\
use axum::{routing::post};
use http::Method;
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use std::net::SocketAddr;
use std::error::Error;
use handler::{start_handler, search_handler, get_cities, get_trains};
use rail_network::{RailNetwork};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;



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
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any);

    let app = axum::Router::new()
        .route("/handler/search", post(search_handler))
        .route("/handler/get", post(start_handler))
        .route("/handler/getCities", post(get_cities))
        .route("/handler/getTrains", post(get_trains))
        .with_state(shared_rn)
        .layer(TraceLayer::new_for_http())
        .layer(cors);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    println!("Listening on http://{}", addr);

    use tokio::net::TcpListener;

let listener = TcpListener::bind(&addr).await.unwrap();

axum::serve(listener, app.into_make_service())
    .with_graceful_shutdown(async {tokio::signal::ctrl_c().await.ok();})
    .await
    .unwrap();

    Ok(())
}
