//    IMPORTS    \\
/*use crate::rail_network;*/
use crate::rail_network::{self, City, Itinerary, RailNetwork, Route, SearchFunctionality, SortBy, TicketClass, Train};
//     USES      \\
/*
use std::sync::Arc;
*/
use axum::{extract::{State, Json}, http::StatusCode, response::Json as ResponseJson};
use chrono::{NaiveTime};
use serde::{Deserialize};
use std::str::FromStr;


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
pub async fn search_handler(
    State(rn): State<std::sync::Arc<RailNetwork>>,
    Json(payload): Json<SearchRequest>,
) -> Result<ResponseJson<Vec<Itinerary>>, StatusCode> {
    //println!("{:?}", payload.sorter);
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
        "PriceAscendant1" => Some(SortBy::PriceAscendant(TicketClass::FirstClass)),
        "PriceAscendant2" => Some(SortBy::PriceAscendant(TicketClass::SecondClass)),
        "PriceDescendant1" => Some(SortBy::PriceDescendant(TicketClass::FirstClass)),
        "PriceDescendant2" => Some(SortBy::PriceDescendant(TicketClass::SecondClass)),
        "TimeAscendant" => Some(SortBy::DepartureTimeAscendant),
        _ => None,
    },
        None => None,
    };
    let dep_city: Option<&str> = payload.filters.departure_city.as_ref()
    .and_then(|city_str| {
        match City::from_str(city_str) {
            Ok(city_enum) => Some(city_enum.as_str()),
            Err(_) => None,
        }
    });
    let arr_city: Option<&str> = payload.filters.arrival_city.as_ref()
    .and_then(|city_str| {
        match City::from_str(city_str) {
            Ok(city_enum) => Some(city_enum.as_str()),
            Err(_) => None,
        }
    });
    
    let tr_type: Option<&str> = payload.filters.train_type.as_ref()
    .and_then(|train_str| {
        match Train::from_str(train_str) {
            Ok(train_enum) => Some(train_enum.as_str()),
            Err(_) => None,
        }
    });
    // Build SearchFunctionality struct (use owned Strings or convert to &str safely)
    let q = SearchFunctionality {
        departure_city: dep_city,
        arrival_city: arr_city,
        earliest_departure,
        train_type: tr_type,
        day_of_week: payload.filters.day_of_week.as_deref(),
        price_range,
        max_price: payload.filters.max_price,
        allowed_transfers: payload.filters.allowed_transfers.unwrap_or(true),
        min_transfer_minutes: payload.filters.min_transfer_minutes.unwrap_or(5),
        sort_by,
    };
    
    println!("{:?}", &q);
    // Call the search function on RailNetwork
    let results = rn.search(&q);
    Ok(ResponseJson(results))
}

pub async fn start_handler(State(rn): State<std::sync::Arc<RailNetwork>>) -> Result<ResponseJson<Vec<Route>>, StatusCode> {
    let routes = rn.get_all_routes().clone();  
    Ok(ResponseJson(routes))
}

pub async fn get_trains() -> Result<ResponseJson<Vec<&'static str>>, StatusCode> {
    let trains = rail_network::get_all_train_names();
    Ok(ResponseJson(trains))
}

pub async fn get_cities() -> Result<ResponseJson<Vec<&'static str>>, StatusCode> {
    let cities = rail_network::get_all_city_names();
    Ok(ResponseJson(cities))
}