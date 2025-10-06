//    IMPORTS    \\
/*use crate::rail_network;*/
use crate::rail_network::Route;
use crate::search_functionality::{RailNetwork, SearchFunctionality, TicketClass, SortBy};
use crate::search_functionality::TicketClass::*;
use crate::search_functionality::SortBy::*;

//     USES      \\
/*

use strum_macros::{EnumVariantNames, EnumIter};
use std::sync::Arc;
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;
use tokio::sync::Mutex;
*/
use axum::{extract::{State, Json}, http::StatusCode, response::Json as ResponseJson};
use chrono::{Duration, NaiveTime};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    filters: Filters,
    sorter: Option<Sorter>,
}
#[derive(Debug, Deserialize)]
pub struct Sorter {
    sort_by: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Filters {
    departure_city: Option<String>,
    arrival_city: Option<String>,
    earliest_departure: Option<String>, 
    train_type: Option<String>,         
    day_of_week: Option<String>,        
    price_range: Option<String>,        
    max_price: Option<u32>,
    allowed_transfers: Option<bool>,
    min_transfer_minutes: Option<i64>,
}
/*
    pub async fn call_parse_csv() -> impl IntoResponse { 
        let routes = parse_CSV().expect("Failed to parse CSV"); 
        let rn = RailNetwork::new(routes);
        let q = QUERY.lock().await;
        let itineraries = rn.search(&q);
        Json(itineraries).into_response()
    }
    pub async fn get_time_a() -> impl IntoResponse  {
        match parse_CSV() {
            Ok(routes) => Json(routes).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse CSV: {}", e),
            ).into_response(),
        }
    }
    pub async fn get_time_d() -> impl IntoResponse  {
        match parse_CSV() {
            Ok(routes) => Json(routes).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse CSV: {}", e),
            ).into_response(),
        }
    }
    pub async fn get_rate1() -> impl IntoResponse  {
        match parse_CSV() {
            Ok(routes) => Json(routes).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse CSV: {}", e),
            ).into_response(),
        }
    }
    pub async fn get_rate2() -> impl IntoResponse  {
        match parse_CSV() {
            Ok(routes) => Json(routes).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse CSV: {}", e),
            ).into_response(),
        }
    }

    pub async fn sort_by(sorter: String) -> impl IntoResponse {
        let mut query = QUERY.lock().await;  // async lock
        query.sort_by = Some(match sorter.as_str() {
            "duration"  => SortBy::Duration,
            "1stRate"   => SortBy::PriceAscendant(TicketClass::FirstClass),
            "2ndRate"   => SortBy::PriceAscendant(TicketClass::SecondClass),
            _ => SortBy::PriceDescendant(TicketClass::FirstClass),
        });
        Json(query.clone()).into_response()
    }    
*/
pub async fn search_handler(
    State(rn): State<std::sync::Arc<RailNetwork>>,
    Json(payload): Json<SearchRequest>,
) -> Result<ResponseJson<Vec<crate::search_functionality::Itinerary>>, StatusCode> {
    println!("{:?}", payload.sorter);
    let earliest_departure = match &payload.filters.earliest_departure {
        Some(time_str) => match NaiveTime::parse_from_str(time_str, "%H:%M:%S") {
            Ok(t) => Some(t),
            Err(_) => None,
        },
        None => None,
    };

    // Map price_range string to TicketClass enum
    let price_range = match payload.filters.price_range.as_deref() {
        Some("First") => Some(TicketClass::FirstClass),
        Some("Second") => Some(TicketClass::SecondClass),
        _ => Some(TicketClass::SecondClass),
    };

    // Map sort_by string to SortBy enum
    let sort_by = match &payload.sorter.unwrap().sort_by {
    Some(s) => match s.as_str() {
        "Duration" => Some(SortBy::Duration),
        "PriceAscendant" => Some(SortBy::PriceAscendant(price_range.unwrap())),
        "PriceDescendant" => Some(SortBy::PriceDescendant(price_range.unwrap())),
        "TimeAscendant" => Some(SortBy::DepartureTimeAscendant),
        _ => None,
    },
        None => None,
    };

    // Build SearchFunctionality struct (use owned Strings or convert to &str safely)
    let q = SearchFunctionality {
        departure_city: payload.filters.departure_city.as_deref(),
        arrival_city: payload.filters.arrival_city.as_deref(),
        earliest_departure,
        train_type: payload.filters.train_type.as_deref(),
        day_of_week: payload.filters.day_of_week.as_deref(),
        price_range,
        max_price: payload.filters.max_price,
        allowed_transfers: payload.filters.allowed_transfers.unwrap_or(true),
        min_transfer_minutes: payload.filters.min_transfer_minutes.unwrap_or(5),
        sort_by,
    };
    // Call the search function on RailNetwork
    let results = rn.search(&q);
    Ok(ResponseJson(results))
}

pub async fn start_handler(State(rn): State<std::sync::Arc<RailNetwork>>) -> Result<ResponseJson<Vec<Route>>, StatusCode> {
    let routes = rn.get_all_routes().clone();  
    Ok(ResponseJson(routes))
}