//    IMPORTS    \\
/*use crate::rail_network;*/
use crate::rail_network::{self, City, Itinerary, RailNetwork, Route, SearchFunctionality, SortBy, TicketClass, Train};
use crate::domain::{ItineraryResponse, Route as DomainRoute, Day, Train as DomainTrain};
//     USES      \\
/*
use std::sync::Arc;
*/
use axum::{extract::{State, Json}, http::StatusCode, response::Json as ResponseJson};
use chrono::{NaiveTime};
use serde::{Deserialize};
use std::str::FromStr;
//  HANDLER  \\
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
    arrival_time_from: Option<String>,
    arrival_time_to: Option<String>,    
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
) -> Result<ResponseJson<Vec<ItineraryResponse>>, StatusCode> {
    //println!("{:?}", payload.sorter);
    let earliest_departure = match &payload.filters.earliest_departure {
        Some(time_str) => match NaiveTime::parse_from_str(time_str, "%H:%M:%S") {
            Ok(t) => Some(t),
            Err(_) => None,
        },
        None => None,
    };
    let arrival_time_from = parse_time(&payload.filters.arrival_time_from);
    let arrival_time_to = parse_time(&payload.filters.arrival_time_to);

    // Map price_range string to TicketClass enum
    let price_range = match payload.filters.price_range.as_deref() {
        Some("First") => Some(TicketClass::FirstClass),
        Some("Second") => Some(TicketClass::SecondClass),
        _ => Some(TicketClass::SecondClass),
    };

    // Map sort_by string to SortBy enum
    let sort_by = payload.sorter.as_ref().and_then(|s| s.sort_by.as_ref()).and_then(|s|match s.as_str() {
        "Duration" => Some(SortBy::Duration),
        "PriceAscendant1" => Some(SortBy::PriceAscendant(price_range.unwrap())),
        "PriceAscendant2" => Some(SortBy::PriceAscendant(price_range.unwrap())),
        "PriceDescendant1" => Some(SortBy::PriceDescendant(price_range.unwrap())),
        "PriceDescendant2" => Some(SortBy::PriceDescendant(price_range.unwrap())),
        "TimeAscendant" => Some(SortBy::DepartureTimeAscendant),
        _ => None,
    });
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
        arrival_time_from,
        arrival_time_to,
        train_type: tr_type,
        day_of_week: payload.filters.day_of_week.as_deref(),
        price_range,
        max_price: payload.filters.max_price,
        allowed_transfers: payload.filters.allowed_transfers.unwrap_or(true),
        min_transfer_minutes: payload.filters.min_transfer_minutes.unwrap_or(5),
        sort_by,
    };
    let results: Vec<Itinerary> = rn.search(&q);
    let response_list: Vec<ItineraryResponse> = results
        .iter()
        .map(|it| convert_itinerary_to_domain(it, &rn))
        .collect();
    
    Ok(ResponseJson(response_list))
}

pub async fn start_handler(State(rn): State<std::sync::Arc<RailNetwork>>) -> Result<ResponseJson<Vec<DomainRoute>>, StatusCode> {
    let routes : Vec<DomainRoute> = rn
    .get_all_routes()
    .iter()
    .map(convert_route_to_domain)
    .collect(); 
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
fn map_weekday_to_enum(day: &str) -> Option<Day> {
    match day {
        "Monday" => Some(Day::Monday),
        "Tuesday" => Some(Day::Tuesday),
        "Wednesday" => Some(Day::Wednesday),
        "Thursday" => Some(Day::Thursday),
        "Friday" => Some(Day::Friday),
        "Saturday" => Some(Day::Saturday),
        "Sunday" => Some(Day::Sunday),
        _ => None,
    }
}
fn map_train_to_enum(train: &str) -> Option<DomainTrain> {
    match train {
        "AVE" => Some(DomainTrain::AVE),
        "EuroCity" => Some(DomainTrain::EuroCity),
        "Eurostar" => Some(DomainTrain::Eurostar),
        "Frecciarossa" => Some(DomainTrain::Frecciarossa),
        "IC" => Some(DomainTrain::IC),
        "ICE" => Some(DomainTrain::ICE),
        "InterCity" => Some(DomainTrain::InterCity),
        "Intercites" => Some(DomainTrain::Intercites),
        "Italo" => Some(DomainTrain::Italo),
        "Nightjet" => Some(DomainTrain::Nightjet),
        "RE" => Some(DomainTrain::RE),
        "RJX" => Some(DomainTrain::RJX),
        "Railjet" => Some(DomainTrain::Railjet),
        "RegioExpress" => Some(DomainTrain::RegioExpress),
        "TER" => Some(DomainTrain::TER),
        "TGV" => Some(DomainTrain::TGV),
        "Thalys" => Some(DomainTrain::Thalys),
        _ => None,
    }
}

fn convert_itinerary_to_domain(it: &Itinerary, rn: &RailNetwork) -> ItineraryResponse {
    let routes: Vec<DomainRoute> = it
        .connections
        .iter()
        .filter_map(|&idxd| rn.get_all_routes().get(idxd))
        .map(convert_route_to_domain)
        .collect();

        ItineraryResponse {
            total_duration: it.total_duration.num_minutes() as u32,
            total_price_first: it.total_first_price,
            total_price_second: it.total_second_price,
            total_transfers: (it.connections.len().saturating_sub(1)) as u32,
            routes,
        }
}
fn convert_route_to_domain(r: &Route) -> DomainRoute {
    let days: Vec<String> = r.days_of_operation
            .iter()
            .map(|d| d.as_str().to_string())
            .collect();

    DomainRoute {
        idx: r.idx,
        departure_city: r.departure_city.clone(),
        arrival_city: r.arrival_city.clone(),
        departure_time: r.departure_time.to_string(),
        arrival_time: r.arrival_time.to_string(),
        train_type: r.train_type.clone(),
        days_of_operation: r
            .days_of_operation
            .iter()
            .map(|d| d.as_str().to_string())
            .collect(),
    }
}

fn parse_time(time_str: &Option<String>) -> Option<NaiveTime> {
    time_str.as_deref().and_then(parse_time_str)
}

fn parse_time_str(time_str: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(time_str, "%H:%M:%S")
    .or_else(|_| NaiveTime::parse_from_str(time_str, "%H:%M"))
    .ok()
}