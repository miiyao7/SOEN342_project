use std::collections::HashMap;
use chrono::{Duration, NaiveTime};
use chrono::Timelike;
use crate::rail_network::Route;

// -- search functionality -- \\
#[derive(Clone, Copy, Debug)]
pub enum TicketClass { FirstClass, SecondClass }

#[derive(Clone, Copy, Debug)]
pub enum SortBy {
    Duration,
    PriceAscendant(TicketClass),
    PriceDescendant(TicketClass),
    DepartureTimeAscendant,
}

#[derive(Clone, Debug, Default)]
pub struct SearchFunctionality<'a> {
    pub departure_city: Option<&'a str>,
    pub arrival_city:   Option<&'a str>,
    pub earliest_departure: Option<NaiveTime>,
    pub train_type:     Option<&'a str>,   
    pub day_of_week:    Option<&'a str>,   
    pub price_range:    Option<TicketClass>,
    pub max_price:      Option<u32>,      
    pub allowed_transfers: bool,        
    pub min_transfer_minutes: i64,         
    pub sort_by:        Option<SortBy>,    
}

#[derive(Clone, Debug)]
pub struct Itinerary {
    pub connections: Vec<usize>,
    pub total_duration: Duration,   
    pub total_first_price: u32,
    pub total_second_price: u32,
    pub transfer_duration: Vec<i64>,
}


impl Itinerary {
    pub fn price_for(&self, class: TicketClass) -> u32 {
        match class {
            TicketClass::FirstClass  => self.total_first_price,
            TicketClass::SecondClass => self.total_second_price,
        }
    }
}

// --main search function--\\

pub fn search_itineraries(routes: &[Route], q: &SearchFunctionality) -> Vec<Itinerary> {
    let idx = IndexSet::build(routes);

    let mut results = direct_routes(routes, &idx, q);
    if q.allowed_transfers {
        results.extend(one_stop_itineraries(routes, &idx, q));
        results.extend(two_stop_itineraries(routes, &idx, q));
    }

    // sorting
    match q.sort_by.unwrap_or(SortBy::DepartureTimeAscendant) {
        SortBy::Duration => results.sort_by_key(|it| it.total_duration.num_minutes()),
        SortBy::PriceAscendant(cls) =>
            results.sort_by(|a, b| a.price_for(cls).cmp(&b.price_for(cls))),
        SortBy::PriceDescendant(cls) =>
            results.sort_by(|a, b| b.price_for(cls).cmp(&a.price_for(cls))),
        SortBy::DepartureTimeAscendant =>
            results.sort_by_key(|it| routes[it.connections[0]].departure_time),
    }

    results
}

// -- indexing -- \\

struct IndexSet {
    by_departure: HashMap<String, Vec<usize>>, 
    by_arrival:   HashMap<String, Vec<usize>>,
}

impl IndexSet {
    fn build(routes: &[Route]) -> Self {
        let mut by_departure: HashMap<String, Vec<usize>> = HashMap::new();
        let mut by_arrival:   HashMap<String, Vec<usize>> = HashMap::new();

        for (i, r) in routes.iter().enumerate() {
            by_departure.entry(norm(r.departure_city.as_str())).or_default().push(i);
            by_arrival.entry(norm(r.arrival_city.as_str())).or_default().push(i);
        }
        Self { by_departure, by_arrival }
    }

    fn get_departures<'a>(&'a self, city: &str) -> impl Iterator<Item = usize> + 'a {
        self.by_departure.get(&norm(city)).into_iter().flatten().copied()
    }
    fn get_arrivals<'a>(&'a self, city: &str) -> impl Iterator<Item = usize> + 'a {
        self.by_arrival.get(&norm(city)).into_iter().flatten().copied()
    }
}

fn norm(s: &str) -> String { s.to_ascii_lowercase() }

// -- filtering and matching -- \\

fn matches_train(r: &Route, t: Option<&str>) -> bool {
    match t { None => true, Some(tt) => r.train_type.as_str().eq_ignore_ascii_case(tt) }
}
fn matches_day(r: &Route, d: Option<&str>) -> bool {
    match d {
        None => true,
        Some(label) => r.days_of_operation.iter().any(|dy| dy.as_str().eq_ignore_ascii_case(label)),
    }
}
fn matches_common(r: &Route, q: &SearchFunctionality) -> bool {
    matches_train(r, q.train_type) && matches_day(r, q.day_of_week)
}
fn depart_after_first_connection(r: &Route, earliest: Option<NaiveTime>) -> bool {
    match earliest { None => true, Some(t0) => r.departure_time >= t0 }
}

fn per_connection_duration(r: &Route) -> Duration { r.duration() } 

fn wait_minutes(prev_arr: NaiveTime, next_dep: NaiveTime) -> i64 {
    let mut secs = (next_dep.num_seconds_from_midnight() as i64)
                 - (prev_arr.num_seconds_from_midnight() as i64);
    if secs < 0 { secs += 24 * 60 * 60; }
    secs / 60
}
fn possible_transfer(prev_arr: NaiveTime, next_dep: NaiveTime, min_transfer_min: i64) -> Option<i64> {
    let w = wait_minutes(prev_arr, next_dep);
    (w >= min_transfer_min).then_some(w)
}
fn sum_chain(routes: &[Route], connections: &[usize], waits: &[i64]) -> Duration {
    let connections_total = connections.iter()
        .map(|&i| per_connection_duration(&routes[i]))
        .fold(Duration::zero(), |a, d| a + d);
    let wait_total = waits.iter()
        .map(|m| Duration::minutes(*m))
        .fold(Duration::zero(), |a, d| a + d);
    connections_total + wait_total
}

// -- itinerary builders -- \\

// direct routes
fn direct_routes(routes: &[Route], idx: &IndexSet, q: &SearchFunctionality) -> Vec<Itinerary> {
    let mut out = Vec::new();

    match (q.departure_city, q.arrival_city) {
        (Some(dep), Some(arr)) => {
            for i in idx.get_departures(dep) {
                let r = &routes[i];
                if !r.arrival_city.as_str().eq_ignore_ascii_case(arr) { continue; }
                if !matches_common(r, q) || !depart_after_first_connection(r, q.earliest_departure) { continue; }

                out.push(Itinerary {
                    connections: vec![i],
                    total_duration: per_connection_duration(r),
                    total_first_price: r.first_class_ticket_rate as u32,
                    total_second_price: r.second_class_ticket_rate as u32,
                    transfer_duration: vec![],
                });
            }
        }
        (Some(dep), None) => {
            for i in idx.get_departures(dep) {
                let r = &routes[i];
                if !matches_common(r, q) || !depart_after_first_connection(r, q.earliest_departure) { continue; }
                out.push(Itinerary {
                    connections: vec![i],
                    total_duration: per_connection_duration(r),
                    total_first_price: r.first_class_ticket_rate as u32,
                    total_second_price: r.second_class_ticket_rate as u32,
                    transfer_duration: vec![],
                });
            }
        }
        (None, Some(arr)) => {
            for i in idx.get_arrivals(arr) {
                let r = &routes[i];
                if !matches_common(r, q) || !depart_after_first_connection(r, q.earliest_departure) { continue; }
                out.push(Itinerary {
                    connections: vec![i],
                    total_duration: per_connection_duration(r),
                    total_first_price: r.first_class_ticket_rate as u32,
                    total_second_price: r.second_class_ticket_rate as u32,
                    transfer_duration: vec![],
                });
            }
        }
        (None, None) => {
            for (i, r) in routes.iter().enumerate() {
                if !matches_common(r, q) || !depart_after_first_connection(r, q.earliest_departure) { continue; }
                out.push(Itinerary {
                    connections: vec![i],
                    total_duration: per_connection_duration(r),
                    total_first_price: r.first_class_ticket_rate as u32,
                    total_second_price: r.second_class_ticket_rate as u32,
                    transfer_duration: vec![],
                });
            }
        }
    }

    out
}
// one-stop routes
fn one_stop_itineraries(routes: &[Route], idx: &IndexSet, q: &SearchFunctionality) -> Vec<Itinerary> {
    let mut out = Vec::new();

    let a_indices: Vec<usize> = match q.departure_city {
        Some(dep) => idx.get_departures(dep).collect(),
        None      => (0..routes.len()).collect(),
    };

    for ia in a_indices {
        let a = &routes[ia];
        if !matches_common(a, q) || !depart_after_first_connection(a, q.earliest_departure) { continue; }


        let b_indices: Vec<usize> = match q.arrival_city {
            Some(arr) => idx.get_arrivals(arr).collect(),
            None      => (0..routes.len()).collect(),
        };

        for ib in b_indices.iter().copied() {
            let b = &routes[ib];
            if !matches_common(b, q) { continue; }
            if !a.arrival_city.as_str().eq_ignore_ascii_case(b.departure_city.as_str()) { continue; }

            if let Some(wait) = possible_transfer(a.arrival_time, b.departure_time, q.min_transfer_minutes) {
                let total = sum_chain(routes, &[ia, ib], &[wait]);
                out.push(Itinerary {
                    connections: vec![ia, ib],
                    total_duration: total,
                    total_first_price: (a.first_class_ticket_rate as u32) + (b.first_class_ticket_rate as u32),
                    total_second_price: (a.second_class_ticket_rate as u32) + (b.second_class_ticket_rate as u32),
                    transfer_duration: vec![wait],
                });
            }
        }
    }

    out
}
// two-stop routes
fn two_stop_itineraries(routes: &[Route], idx: &IndexSet, q: &SearchFunctionality) -> Vec<Itinerary> {
    let mut out = Vec::new();

    let a_indices: Vec<usize> = match q.departure_city {
        Some(dep) => idx.get_departures(dep).collect(),
        None      => (0..routes.len()).collect(),
    };

    for ia in a_indices {
        let a = &routes[ia];
        if !matches_common(a, q) || !depart_after_first_connection(a, q.earliest_departure) { continue; }

        let b_indices: Vec<usize> = idx.get_departures(a.arrival_city.as_str()).collect();

        for ib in b_indices {
            let b = &routes[ib];
            if !matches_common(b, q) { continue; }
            if norm(b.arrival_city.as_str()) == norm(a.departure_city.as_str()) { continue; }
            let w1 = match possible_transfer(a.arrival_time, b.departure_time, q.min_transfer_minutes) {
                Some(m) => m,
                None => continue,
            };

            let c_indices: Vec<usize> = match q.arrival_city {
                Some(arr) => idx.get_arrivals(arr).collect(),
                None      => (0..routes.len()).collect(),
            };

            for ic in c_indices.iter().copied() {
                let c = &routes[ic];
                if !matches_common(c, q) { continue; }
                if !b.arrival_city.as_str().eq_ignore_ascii_case(c.departure_city.as_str()) { continue; }

                if let Some(w2) = possible_transfer(b.arrival_time, c.departure_time, q.min_transfer_minutes) {
                    let total = sum_chain(routes, &[ia, ib, ic], &[w1, w2]);
                    out.push(Itinerary {
                        connections: vec![ia, ib, ic],
                        total_duration: total,
                        total_first_price: (a.first_class_ticket_rate as u32)
                                         + (b.first_class_ticket_rate as u32)
                                         + (c.first_class_ticket_rate as u32),
                        total_second_price: (a.second_class_ticket_rate as u32)
                                          + (b.second_class_ticket_rate as u32)
                                          + (c.second_class_ticket_rate as u32),
                        transfer_duration: vec![w1, w2],
                    });
                }
            }
        }
    }

    out
}
