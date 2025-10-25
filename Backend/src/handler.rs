use crate::rail_network::{self, City, Itinerary, RailNetwork, Route, SearchFunctionality, SortBy, TicketClass, Train, Person, Trip};
use crate::domain::{ItineraryResponse, Route as DomainRoute, Day, Train as DomainTrain};
use axum::{extract::{State, Json}, http::StatusCode, response::Json as ResponseJson};
use chrono::{NaiveTime, NaiveDate};
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;

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
    max_price: Option<u16>,
    allowed_transfers: Option<bool>,
    min_transfer_minutes: Option<i64>,
}
#[derive(Debug, Deserialize)]
pub struct BookTripRequest {
    travelers: Vec<Person>,
    trip_date: NaiveDate,
    route_id: u16
}
#[derive(Debug, Deserialize)]
pub struct FilterBookingsRequest {
    is_ongoing: bool,
    last_name: String,
    id: String
}
pub async fn search_handler(
    State(rn): State<Arc<RwLock<RailNetwork>>>,
    Json(payload): Json<SearchRequest>,
) -> Result<ResponseJson<Vec<ItineraryResponse>>, StatusCode> {
    let earliest_departure = match &payload.filters.earliest_departure {
        Some(time_str) => match NaiveTime::parse_from_str(time_str, "%H:%M:%S") {
            Ok(t) => Some(t),
            Err(_) => None,
        },
        None => None,
    };
    let arrival_time_from = parse_time(&payload.filters.arrival_time_from);
    let arrival_time_to = parse_time(&payload.filters.arrival_time_to);

    let price_range = match payload.filters.price_range.as_deref() {
        Some("First") => Some(TicketClass::FirstClass),
        Some("Second") => Some(TicketClass::SecondClass),
        _ => Some(TicketClass::SecondClass),
    };

    let sort_by = payload.sorter.as_ref().and_then(|s| s.sort_by.as_ref()).and_then(|s|match s.as_str() {
        "Duration" => Some(SortBy::Duration),
        "PriceAscendant1" => Some(SortBy::PriceAscendant(TicketClass::FirstClass)),
        "PriceAscendant2" => Some(SortBy::PriceAscendant(TicketClass::SecondClass)),
        "PriceDescendant1" => Some(SortBy::PriceDescendant(TicketClass::FirstClass)),
        "PriceDescendant2" => Some(SortBy::PriceDescendant(TicketClass::SecondClass)),
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
    
    let q = SearchFunctionality {
        departure_city: dep_city,
        arrival_city: arr_city,
        earliest_departure: earliest_departure,
        arrival_time_from: arrival_time_from,
        arrival_time_to: arrival_time_to,
        train_type: tr_type,
        day_of_week: payload.filters.day_of_week.as_deref(),
        price_range: price_range,
        max_price: payload.filters.max_price,
        allowed_transfers: payload.filters.allowed_transfers.unwrap_or(true),
        min_transfer_minutes: payload.filters.min_transfer_minutes.unwrap_or(5),
        sort_by
    };
    let rn_read = rn.read().await;
    let results: Vec<Itinerary> = rn_read.search(&q);
    let response_list: Vec<ItineraryResponse> = results
        .iter()
        .map(|it| convert_itinerary_to_domain(it, &rn_read))
        .collect();
    Ok(ResponseJson(response_list))
}

pub async fn start_handler(State(rn): State<Arc<RwLock<RailNetwork>>>) -> Result<ResponseJson<Vec<DomainRoute>>, StatusCode> {
    let routes : Vec<DomainRoute> = rn
    .read()
    .await
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

pub async fn book_trip_handler(
    State(rn): State<Arc<RwLock<RailNetwork>>>,
    Json(payload): Json<BookTripRequest>
) -> Result<ResponseJson<Trip>, StatusCode> {
    let mut rn = rn.write().await;
    let mut route = Route::default();
    for route_s in rn.routes.clone() {
        if route_s.get_id()[2..].parse::<u16>().unwrap_or(0) == payload.route_id {
            route = route_s;
            break;
        }
    }
    //println!("{:?}", payload);
    Ok(ResponseJson(rn.book_trip(payload.travelers, payload.trip_date, route).await))
}

pub async fn filter_bookings_handler(
    State(rn): State<Arc<RwLock<RailNetwork>>>,
    Json(payload): Json<FilterBookingsRequest>,
) -> Result<Json<Vec<Trip>>, StatusCode> {
    let net = rn.read().await;

    let trips = net
        .filter_bookings(payload.is_ongoing, payload.last_name, payload.id)
        .await
        .map_err(|e| {
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(trips))
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

fn convert_itinerary_to_domain(it: &Itinerary, _rn: &RailNetwork) -> ItineraryResponse {
    let routes: Vec<DomainRoute> = it
        .connections
        .iter()
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
        idx: r.get_id()[2..].parse().unwrap_or(0),
        departure_city: r.departure_city.as_str().to_string(),
        arrival_city: r.arrival_city.as_str().to_string(),
        departure_time: r.departure_time.to_string(),
        arrival_time: r.arrival_time.to_string(),
        train_type: r.train_type.as_str().to_string(),
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