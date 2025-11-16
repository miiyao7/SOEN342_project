use serde::{Deserialize, Serialize};
use csv::Reader;
use std::{collections::HashMap, env, error::Error, ptr::null, str::FromStr};
use chrono::{NaiveTime, Duration, Timelike, NaiveDate, Local};
use strum_macros::{EnumIter, AsRefStr};
use uuid::Uuid;
use dotenvy::dotenv;
use sqlx::{PgPool, Row};


// -- RAIL NETWORK -- \\

#[derive(Debug)]
pub struct RailNetwork {
    pub routes: Vec<Route>,
    pub reservations: Vec<Trip>,
    pub pool: PgPool
} impl RailNetwork {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let routes = parse_CSV()?;
        let mut bookings = Vec::<Trip>::new();
        dotenv().ok();
        let pool = PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await?;
        let trips = sqlx::query(r#"SELECT id, date, route_id FROM "Trips";"#)
            .persistent(false)
            .fetch_all(&pool)
            .await?;
        for trip in trips {
            let mut route = Route::default();
            for route_s in routes.clone() {
                if route_s.get_id()[2..].parse().unwrap_or(0) == trip.get::<i16, &str>("route_id") {
                    route = route_s;
                    break;
                }
            }
            let tickets_s = sqlx::query(r#"
                SELECT  tickets.id, 
                        tickets.trip_id, 
                        tickets.person_id, 
                        persons.first_name, 
                        persons.last_name, 
                        persons.age 
                FROM        "Tickets"   AS tickets 
                LEFT JOIN   "Persons"   AS persons 
                    ON tickets.person_id = persons.id 
                WHERE trip_id = $1;
                "#)
                .persistent(false)
                .bind::<Uuid>(trip.get("id"))
                .fetch_all(&pool)
                .await?;
            
            
            let mut tickets: Vec<Ticket> = Vec::new();
            for ticket in tickets_s {
                let person = Person {
                    id: ticket.get("person_id"),
                    first_name: ticket.get("first_name"),
                    last_name: ticket.get("last_name"),
                    age: ticket.get::<i16, _>("age") as u8
                };
                tickets.push(Ticket {id: ticket.get("id"), traveler: person});
            }
            bookings.push(Trip {id: trip.get("id"), tickets: tickets, date: trip.get("date"), route: route});
        }
        Ok(RailNetwork {routes: routes, reservations: bookings, pool: pool})
    }

    // -- MAIN SEARCH FUNCTION -- \\

    pub fn search(&self, q: &SearchFunctionality) -> Vec<Itinerary> {
        let routes = &self.routes;
        let idx = IndexSet::build(routes);
    
        let mut itineraries = Self::direct_routes(routes, &idx, q);
        if q.allowed_transfers {
            itineraries.extend(Self::one_stop_itineraries(routes, &idx, q));
            itineraries.extend(Self::two_stop_itineraries(routes, &idx, q));
        }
        itineraries.retain(|it| self.passes_arrival_time_filter(it, q));
    
        self.sort(&mut itineraries, q);
        itineraries
    }

    fn passes_arrival_time_filter(&self, it: &Itinerary, q: &SearchFunctionality) -> bool {
        if q.arrival_time_from.is_none() && q.arrival_time_to.is_none() {
            return true; 
        }
        let last_route = match it.connections.last() {
            Some(route) => route,
            None => return false, 
        };
        if let Some(from) = q.arrival_time_from {
            if last_route.arrival_time < from {
                return false; 
            }
        }
        if let Some(to) = q.arrival_time_to {
            if last_route.arrival_time > to {
                return false; 
            }
        }
        true
    }


    // -- SORTING FUNCTION -- \\

    fn sort(&self, itineraries: &mut Vec<Itinerary>, q: &SearchFunctionality) {
        // If max_price is set, filter itineraries by both classes
        if let Some(max_price) = q.max_price {
            itineraries.retain(|it| {
                // Check if price_for for both classes <= max_price (depending on logic)
                it.price_for(TicketClass::FirstClass) <= max_price &&
                it.price_for(TicketClass::SecondClass) <= max_price
            });
        }
        match q.sort_by.unwrap_or(SortBy::DepartureTimeAscendant) {
            SortBy::Duration => itineraries.sort_by_key(|it| it.total_duration.num_minutes()),
            SortBy::PriceAscendant(cls) =>
                itineraries.sort_by(|a, b| a.price_for(cls).cmp(&b.price_for(cls))),
            SortBy::PriceDescendant(cls) =>
                itineraries.sort_by(|a, b| b.price_for(cls).cmp(&a.price_for(cls))),
            SortBy::DepartureTimeAscendant =>
                itineraries.sort_by_key(|it| it.connections[0].departure_time)
        }
    }


    // -- BOOKING TRIP FUNCTION -- \\

    pub async fn book_trip(&mut self, travelers: Vec<Person>, date: NaiveDate, route: Route) -> Trip {
        let reservation = Trip::new(travelers, date, route);
        Self::add_reservation(self, reservation.clone()).await.expect("Failed to add the trip to the database.");
        reservation
    }
    
    // -- FILTERING BOOKINGS FUNCTION -- \\
    pub async fn filter_bookings(&self, is_ongoing: bool, last_name: String, id: String) -> Result<Vec<Trip>, Box<dyn Error>> {
        let today = Local::now().date_naive();
        
        let trips_query = sqlx::query(r#"SELECT DISTINCT t.id AS trip_id, t.date AS date, t.route_id AS route_id FROM "Trips" t JOIN "Tickets" tk ON t.id = tk.trip_id JOIN "Persons" p ON tk.person_id = p.id WHERE LOWER(p.last_name) = LOWER($1) AND p.id = $2"#)
        .persistent(false)
        .bind(&last_name)
        .bind(&id)
        .fetch_all(&self.pool)
        .await?;

        let mut trips = Vec::new();
        
        for trip_row in trips_query {
            let trip_id: Uuid = trip_row.get("trip_id");

            let trip_date: chrono::NaiveDate = trip_row.get("date");
            let trip_date_naive = trip_date;
            let is_trip_ongoing = trip_date_naive >= today;

            let route_id: i16 = trip_row.get("route_id");
            let route = self.routes.iter().find(|r| r.id.id == route_id as u16).unwrap();
            
            if (is_ongoing && is_trip_ongoing) || (!is_ongoing && !is_trip_ongoing) {

                let tickets_query = sqlx::query(r#"SELECT tk.id AS ticket_id, p.id AS person_id, p.first_name AS first_name, p.last_name AS last_name, p.age AS age FROM "Tickets" AS tk JOIN "Persons" AS p ON tk.person_id = p.id WHERE tk.trip_id = $1"#)
                .persistent(false)
                .bind(&trip_id)
                .fetch_all(&self.pool)
                .await?;
                
                let mut tickets = Vec::new();
                for ticket_row in tickets_query {
                    let ticket_id: Uuid = ticket_row.get("ticket_id");
                    let person_id: String = ticket_row.get("person_id");
                    let first_name: String = ticket_row.get("first_name");
                    let last_name: String = ticket_row.get("last_name");
                    let age: i16 = ticket_row.get("age");
                    
                    tickets.push(Ticket {
                        id: ticket_id,
                        traveler: Person {
                            id: person_id,
                            first_name,
                            last_name,
                            age: age as u8,
                        },
                    });
                }
                
                trips.push(Trip {
                    id: trip_id,
                    tickets,
                    date: trip_date,
                    route: route.clone()
                });
            }
        }

        if is_ongoing {
            trips.sort_by_key(|trip| trip.date);
        } else {
            trips.sort_by_key(|trip| std::cmp::Reverse(trip.date));
        }
        
        Ok(trips)
    }
    
    // -- DATABASE FUNCTIONS -- \\

    pub async fn add_reservation(&mut self, booking: Trip) -> Result<(), Box<dyn Error>> {
        self.reservations.push(booking.clone());        
        let _ = sqlx::query(r#"INSERT INTO "Trips" (id, date, route_id) VALUES ($1, $2, $3);"#)
            .persistent(false)
            .bind(booking.id)
            .bind(booking.date)
            .bind(booking.route.id.id as i16)
            .execute(&self.pool)
            .await?;
        for ticket in booking.tickets {
            
            let person_row = sqlx::query(r#"
                SELECT id, first_name, last_name, age
                FROM "Persons"
                WHERE id = $1;
            "#)
            .persistent(false)
            .bind::<String>(ticket.traveler.id.clone())
            .fetch_optional(&self.pool)
            .await?;
                
            if person_row.is_none() {
                let _ = sqlx::query(r#"INSERT INTO "Persons" (id, first_name, last_name, age) VALUES ($1, $2, $3, $4);"#)
                .persistent(false)
                .bind(ticket.traveler.id.clone())
                .bind(ticket.traveler.first_name)
                .bind(ticket.traveler.last_name)
                .bind(ticket.traveler.age as i16)
                .execute(&self.pool)
                .await?;
            }
            let _ = sqlx::query(r#"INSERT INTO "Tickets" (id, trip_id, person_id) VALUES ($1, $2, $3);"#)
                .persistent(false)
                .bind(ticket.id)
                .bind(booking.id)
                .bind(ticket.traveler.id)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }


    // -- FILTERING AND MATCHING -- \\

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
        Self::matches_train(r, q.train_type) && Self::matches_day(r, q.day_of_week)
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
    fn possible_transfer(prev_arr: NaiveTime, next_dep: NaiveTime, min_transfer_min: i64, max_transfer_min: i64) -> Option<i64> {
        let w = Self::wait_minutes(prev_arr, next_dep);
        (w >= min_transfer_min && w < max_transfer_min).then_some(w)
    }
    fn sum_chain(routes: &[Route], connections: &[usize], waits: &[i64]) -> Duration {
        let connections_total = connections.iter()
            .map(|&i| Self::per_connection_duration(&routes[i]))
            .fold(Duration::zero(), |a, d| a + d);
        let wait_total = waits.iter()
            .map(|m| Duration::minutes(*m))
            .fold(Duration::zero(), |a, d| a + d);
        connections_total + wait_total
    }


    // -- ITINERARY BUILDERS -- \\

    pub fn get_all_routes(&self) -> &Vec<Route> {&self.routes}

    fn direct_routes(routes: &[Route], idx: &IndexSet, q: &SearchFunctionality) -> Vec<Itinerary> {
        let mut out = Vec::new();

        match (q.departure_city, q.arrival_city) {
            (Some(dep), Some(arr)) => {
                for i in idx.get_departures(dep) {
                    let r = &routes[i];
                    if !r.arrival_city.as_str().eq_ignore_ascii_case(arr) { continue; }
                    if !Self::matches_common(r, q) || !Self::depart_after_first_connection(r, q.earliest_departure) { continue; }

                    let mut it = Itinerary::default();
                    it.addRoute(r.clone());
                    out.push(it);
                }
            }
            (Some(dep), None) => {
                for i in idx.get_departures(dep) {
                    let r = &routes[i];
                    if !Self::matches_common(r, q) || !Self::depart_after_first_connection(r, q.earliest_departure) { continue; }
                    let mut it = Itinerary::default();
                    it.addRoute(r.clone());
                    out.push(it);
                }
            }
            (None, Some(arr)) => {
                for i in idx.get_arrivals(arr) {
                    let r = &routes[i];
                    if !Self::matches_common(r, q) || !Self::depart_after_first_connection(r, q.earliest_departure) { continue; }
                    let mut it = Itinerary::default();
                    it.addRoute(r.clone());
                    out.push(it);
                }
            }
            (None, None) => {
                for (i, r) in routes.iter().enumerate() {
                    if !Self::matches_common(r, q) || !Self::depart_after_first_connection(r, q.earliest_departure) { continue; }
                    let mut it = Itinerary::default();
                    it.addRoute(r.clone());
                    out.push(it);
                }
            }
        }

        out
    }

    fn one_stop_itineraries(routes: &[Route], idx: &IndexSet, q: &SearchFunctionality) -> Vec<Itinerary> {
        let mut out = Vec::new();

        let a_indices: Vec<usize> = match q.departure_city {
            Some(dep) => idx.get_departures(dep).collect(),
            None      => (0..routes.len()).collect(),
        };

        for ia in a_indices {
            let a = &routes[ia];
            if !Self::matches_common(a, q) || !Self::depart_after_first_connection(a, q.earliest_departure) { continue; }

            let b_indices: Vec<usize> = match q.arrival_city {
                Some(arr) => idx.get_arrivals(arr).collect(),
                None      => (0..routes.len()).collect(),
            };

            for ib in b_indices.iter().copied() {
                let b = &routes[ib];
                if !Self::matches_common(b, q) { continue; }
                if !a.arrival_city.as_str().eq_ignore_ascii_case(b.departure_city.as_str()) { continue; }

                if let Some(_wait) = Self::possible_transfer(a.arrival_time, b.departure_time, q.min_transfer_minutes, if b.arrival_time >= NaiveTime::from_hms_opt(8, 0, 0).unwrap() && b.arrival_time <= NaiveTime::from_hms_opt(20, 0, 0).unwrap() {240} else {60}) {
                    let mut it = Itinerary::default();
                    it.addRoute(a.clone());
                    it.addRoute(b.clone());
                    out.push(it);
                }
            }
        }

        out
    }

    fn two_stop_itineraries(routes: &[Route], idx: &IndexSet, q: &SearchFunctionality) -> Vec<Itinerary> {
        let mut out = Vec::new();

        let a_indices: Vec<usize> = match q.departure_city {
            Some(dep) => idx.get_departures(dep).collect(),
            None      => (0..routes.len()).collect(),
        };

        for ia in a_indices {
            let a = &routes[ia];
            if !Self::matches_common(a, q) || !Self::depart_after_first_connection(a, q.earliest_departure) { continue; }

            let b_indices: Vec<usize> = idx.get_departures(a.arrival_city.as_str()).collect();

            for ib in b_indices {
                let b = &routes[ib];
                if !Self::matches_common(b, q) { continue; }
                if norm(b.arrival_city.as_str()) == norm(a.departure_city.as_str()) { continue; }

                let _w1 = match Self::possible_transfer(a.arrival_time, b.departure_time, q.min_transfer_minutes, if b.arrival_time >= NaiveTime::from_hms_opt(8, 0, 0).unwrap() && b.arrival_time <= NaiveTime::from_hms_opt(20, 0, 0).unwrap() {240} else {60}) {
                    Some(m) => m,
                    None => continue,
                };

                let c_indices: Vec<usize> = match q.arrival_city {
                    Some(arr) => idx.get_arrivals(arr).collect(),
                    None      => (0..routes.len()).collect(),
                };

                for ic in c_indices.iter().copied() {
                    let c = &routes[ic];
                    if !Self::matches_common(c, q) { continue; }
                    if !b.arrival_city.as_str().eq_ignore_ascii_case(c.departure_city.as_str()) { continue; }

                    if let Some(_w2) = Self::possible_transfer(b.arrival_time, c.departure_time, q.min_transfer_minutes, if b.arrival_time >= NaiveTime::from_hms_opt(8, 0, 0).unwrap() && b.arrival_time <= NaiveTime::from_hms_opt(20, 0, 0).unwrap() {240} else {60}) {
                        let mut it = Itinerary::default();
                        it.addRoute(a.clone());
                        it.addRoute(b.clone());
                        it.addRoute(c.clone());
                        out.push(it);
                    }
                }
            }
        }

        out
    }
}


// -- SEARCH FUNCTIONALITY -- \\

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TicketClass { FirstClass, SecondClass }

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SortBy {
    Duration,
    PriceAscendant(TicketClass),
    PriceDescendant(TicketClass),
    DepartureTimeAscendant,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SearchFunctionality<'a> {
    pub departure_city: Option<&'a str>,
    pub arrival_city:   Option<&'a str>,
    pub earliest_departure: Option<NaiveTime>,
    pub arrival_time_from: Option<NaiveTime>,
    pub arrival_time_to:   Option<NaiveTime>,
    pub train_type:     Option<&'a str>,   
    pub day_of_week:    Option<&'a str>,   
    pub price_range:    Option<TicketClass>,
    pub max_price:      Option<u16>,      
    pub allowed_transfers: bool,        
    pub min_transfer_minutes: i64,         
    pub sort_by:        Option<SortBy>,    
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Itinerary {
    pub connections: Vec<Route>,
    pub total_duration: Duration,   
    pub total_first_price: u16,
    pub total_second_price: u16,
    pub transfer_duration: Vec<u64>
}
impl Itinerary {
    pub fn price_for(&self, cls: TicketClass) -> u16 {
        match cls {
            TicketClass::FirstClass => self.total_first_price,
            TicketClass::SecondClass => self.total_second_price,
        }
    }
    pub fn addRoute(&mut self, route: Route) {
        if let Some(prev) = self.connections.last() {
            let secs_opt = (route.departure_time.num_seconds_from_midnight() as i64)
                        .checked_sub(prev.arrival_time.num_seconds_from_midnight() as i64);
            /*if secs < 0 {secs += 24*60*60;}*/
            let secs = match secs_opt {
                Some(s) if s >= 0 => s,
                _ => (route.departure_time.num_seconds_from_midnight() as i64)
                    + 24 * 60 * 60
                    - (prev.arrival_time.num_seconds_from_midnight() as i64),
            };
            let wait = secs/60 as i64;
            self.transfer_duration.push(wait as u64);
            self.total_duration = self.total_duration + Duration::minutes(wait as i64);
        }
        self.total_duration = self.total_duration + route.duration();
        self.total_first_price = self.total_first_price.saturating_add(route.first_class_ticket_rate.into());
        self.total_second_price = self.total_second_price.saturating_add(route.second_class_ticket_rate.into());
        self.connections.push(route);
    }
}


// -- INDEXING -- \\

struct IndexSet {
    by_departure: HashMap<String, Vec<usize>>, 
    by_arrival:   HashMap<String, Vec<usize>>,
} impl IndexSet {
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

fn norm(s: &str) -> String {s.to_ascii_lowercase()}


// -- PARSE CSV -- \\

pub fn parse_CSV() -> Result<Vec<Route>, Box<dyn Error>> {
    let mut routes: Vec<Route> = Vec::new();
    for result in Reader::from_path("eu_rail_network.csv")?.deserialize() {
        let row: CSVRoute = result?;
        let days: Vec<Day> = if row.days_of_operation == "Daily" {
            Day::daily()
        } else {
            row.days_of_operation.split(|c| c == ',' || c == '-').filter_map(|s| s.parse().ok()).collect()
        };
        let route = Route {
            id: {RouteID::new(row.id)},
            departure_city: row.departure_city.parse()?,
            arrival_city: row.arrival_city.parse()?,
            departure_time: NaiveTime::parse_from_str(&row.departure_time, "%H:%M")?,
            arrival_time: NaiveTime::parse_from_str(&row.arrival_time.split_whitespace().next().unwrap(), "%H:%M")?,
            train_type: row.train_type.parse()?,
            days_of_operation: days,
            first_class_ticket_rate: row.first_class_ticket_rate,
            second_class_ticket_rate: row.second_class_ticket_rate
        };
        routes.push(route);
    }
    Ok(routes)
}
#[derive(Debug, Deserialize)] struct CSVRoute {#[serde(rename = "Route ID")] id: String, #[serde(rename = "Departure City")] departure_city: String, #[serde(rename = "Arrival City")] arrival_city: String, #[serde(rename = "Departure Time")] departure_time: String, #[serde(rename = "Arrival Time")] arrival_time: String, #[serde(rename = "Train Type")] train_type: String, #[serde(rename = "Days of Operation")] days_of_operation: String, #[serde(rename = "First Class ticket rate (in euro)")] first_class_ticket_rate: u16, #[serde(rename = "Second Class ticket rate (in euro)")] second_class_ticket_rate: u16}


// -- TRIP -- \\

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trip {
    id: Uuid,
    pub tickets: Vec<Ticket>,
    pub date: NaiveDate,
    pub route: Route
} impl Trip {
    pub fn new(travelers: Vec<Person>, date: NaiveDate, route: Route) -> Self {
        let mut tickets = Vec::<Ticket>::new();
        for traveler in travelers {
            tickets.push(Ticket::new(traveler))
        }
        Self {id: Uuid::new_v4(), tickets, date, route}
    }
    pub fn is_correct_date(&self, is_ongoing: bool) -> bool {
        self.date < Local::now().naive_local().date() || is_ongoing
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
    id: Uuid,
    pub traveler: Person
} impl Ticket {pub fn new(traveler: Person) -> Self {Self {id: Uuid::new_v4(), traveler}}}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8
}


// -- ROUTE -- \\

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Route {
    id: RouteID,
    pub departure_city: City,
    pub arrival_city: City,
    pub departure_time: NaiveTime,
    pub arrival_time: NaiveTime,
    pub train_type: Train,
    pub days_of_operation: Vec<Day>,
    pub first_class_ticket_rate: u16,
    pub second_class_ticket_rate: u16
} impl Route {
    pub fn default() -> Self {
        Route {id: RouteID {id: 0},
            departure_city: City::ACoruna,
            arrival_city: City::ACoruna,
            departure_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            arrival_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            train_type: Train::AVE,
            days_of_operation: Vec::new(),
            first_class_ticket_rate: 0,
            second_class_ticket_rate: 0
        }
    }
    pub fn arrival_is_next_day(&self) -> bool {self.arrival_time.signed_duration_since(self.departure_time) < Duration::zero()}
    pub fn duration(&self) -> Duration {
        let duration = self.arrival_time.signed_duration_since(self.departure_time);
        if duration < Duration::zero() {Duration::hours(24) + duration} else {duration}
    }
    pub fn get_id(&self) -> String {self.id.get_id()}
    pub fn set_id(&mut self, route_id: String) {self.id.set_id(route_id)}
}


// -- ROUTE ID -- \\

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RouteID {id: u16} impl RouteID {
    pub fn new(id: String) -> Self {RouteID {id: id[2..].parse().expect("Error: not a number.")}}
    pub fn get_id(&self) -> String {format!("{}{}{}", 'R', 0, self.id)}
    pub fn set_id(&mut self, route_id: String) {self.id = route_id[2..].parse().expect("Error: not a number.")}
}


// -- CITY -- \\

#[derive(Clone, Debug, EnumIter, AsRefStr, Serialize, Deserialize)]
pub enum City {ACoruna, Aalborg, Aarhus, Alicante, Almeria, Amiens, Amsterdam, Ancona, Angers, Annecy, Antwerp, Arezzo, Ashford, Augsburg, Avignon, Badajoz, Barcelona, Bari, Basel, Belgrade, Bergamo, Bergen, Berlin, Bern, Besancon, Bilbao, Birmingham, Bochum, Bologna, Bolzano, Bonn, Bordeaux, Bratislava, Brasov, Bremen, Brescia, Brest, Brighton, Brindisi, Bristol, Brno, Bruges, Brussels, Bucharest, Budapest, Burgas, Burgos, Calais, Cambridge, Cardiff, Cartagena, Catania, Chambery, ClermontFerrand, ClujNapoca, Cologne, Como, Copenhagen, Cork, Cuenca, Cadiz, Cordoba, Debrecen, Derby, Dijon, Dortmund, Drammen, Dresden, Dublin, Dusseldorf, Edinburgh, Eindhoven, Essen, Exeter, Ferrara, Florence, Forli, Frankfurt, Galway, Gdansk, Gdynia, Geneva, Genoa, Ghent, Glasgow, Gothenburg, Granada, Graz, Grenoble, Hamburg, Hannover, Heidelberg, Helsingborg, Helsinki, Iasi, Innsbruck, Karlsruhe, Katowice, Kiel, Kosice, Krakow, LAquila, LaRochelle, LaSpezia, Lausanne, LeMans, Leeds, Leicester, Leipzig, Lille, Limerick, Limoges, Linkoping, Linz, Lisbon, Liverpool, Livorno, Liege, Ljubljana, Logrono, London, Lublin, Lucerne, Lugano, Lund, Lyon, Madrid, Malmo, Manchester, Mannheim, Maribor, Marseille, Messina, Metz, Milan, Modena, Montpellier, Mostar, Mulhouse, Munich, Murcia, Malaga, Nancy, Nantes, Naples, Narbonne, Newcastle, Nice, Nis, Norrkoping, Nottingham, NoviSad, Nuremberg, Nimes, Odense, Oslo, Ostrava, Oulu, Oviedo, Oxford, Padua, Palermo, Pamplona, Paris, Parma, Perpignan, Perugia, Piacenza, Pisa, Plovdiv, Plymouth, Plzen, Poitiers, Porto, Portsmouth, Potsdam, Poznan, Prague, Pecs, Ravenna, Reading, Regensburg, ReggioCalabria, ReggioEmilia, Reims, Rennes, Rijeka, Rimini, Rome, Rostock, Rotterdam, Rouen, Salamanca, Salerno, Salzburg, SanSebastian, Santander, SantiagoDeCompostela, Sarajevo, Seville, Sheffield, Sofia, Sopot, Southampton, Split, StGallen, Stavanger, Stockholm, Strasbourg, Stuttgart, Swansea, Szeged, Tampere, Taranto, Terni, TheHague, Thessaloniki, Timisoara, Toledo, Toulouse, Tours, Trento, Trieste, Trondheim, Turin, Turku, Uppsala, Utrecht, Valencia, Valladolid, Varna, Venice, Verona, Versailles, Vicenza, Vienna, Vigo, Vasteras, Warsaw, Waterford, Wroclaw, Wurzburg, York, Zagreb, Zaragoza, Zurich, Orebro, CeskeBudejovice, Lodz} 
impl City {pub fn as_str(&self) -> &'static str {match self {City::ACoruna => "A Coruña", City::Aalborg => "Aalborg", City::Aarhus => "Aarhus", City::Alicante => "Alicante", City::Almeria => "Almería", City::Amiens => "Amiens", City::Amsterdam => "Amsterdam", City::Ancona => "Ancona", City::Angers => "Angers", City::Annecy => "Annecy", City::Antwerp => "Antwerp", City::Arezzo => "Arezzo", City::Ashford => "Ashford", City::Augsburg => "Augsburg", City::Avignon => "Avignon", City::Badajoz => "Badajoz", City::Barcelona => "Barcelona", City::Bari => "Bari", City::Basel => "Basel", City::Belgrade => "Belgrade", City::Bergamo => "Bergamo", City::Bergen => "Bergen", City::Berlin => "Berlin", City::Bern => "Bern", City::Besancon => "Besançon", City::Bilbao => "Bilbao", City::Birmingham => "Birmingham", City::Bochum => "Bochum", City::Bologna => "Bologna", City::Bolzano => "Bolzano", City::Bonn => "Bonn", City::Bordeaux => "Bordeaux", City::Bratislava => "Bratislava", City::Brasov => "Brașov", City::Bremen => "Bremen", City::Brescia => "Brescia", City::Brest => "Brest", City::Brighton => "Brighton", City::Brindisi => "Brindisi", City::Bristol => "Bristol", City::Brno => "Brno", City::Bruges => "Bruges", City::Brussels => "Brussels", City::Bucharest => "Bucharest", City::Budapest => "Budapest", City::Burgas => "Burgas", City::Burgos => "Burgos", City::Calais => "Calais", City::Cambridge => "Cambridge", City::Cardiff => "Cardiff", City::Cartagena => "Cartagena", City::Catania => "Catania", City::Chambery => "Chambéry", City::ClermontFerrand => "Clermont-Ferrand", City::ClujNapoca => "Cluj-Napoca", City::Cologne => "Cologne", City::Como => "Como", City::Copenhagen => "Copenhagen", City::Cork => "Cork", City::Cuenca => "Cuenca", City::Cadiz => "Cádiz", City::Cordoba => "Córdoba", City::Debrecen => "Debrecen", City::Derby => "Derby", City::Dijon => "Dijon", City::Dortmund => "Dortmund", City::Drammen => "Drammen", City::Dresden => "Dresden", City::Dublin => "Dublin", City::Dusseldorf => "Düsseldorf", City::Edinburgh => "Edinburgh", City::Eindhoven => "Eindhoven", City::Essen => "Essen", City::Exeter => "Exeter", City::Ferrara => "Ferrara", City::Florence => "Florence", City::Forli => "Forlì", City::Frankfurt => "Frankfurt", City::Galway => "Galway", City::Gdansk => "Gdańsk", City::Gdynia => "Gdynia", City::Geneva => "Geneva", City::Genoa => "Genoa", City::Ghent => "Ghent", City::Glasgow => "Glasgow", City::Gothenburg => "Gothenburg", City::Granada => "Granada", City::Graz => "Graz", City::Grenoble => "Grenoble", City::Hamburg => "Hamburg", City::Hannover => "Hannover", City::Heidelberg => "Heidelberg", City::Helsingborg => "Helsingborg", City::Helsinki => "Helsinki", City::Iasi => "Iași", City::Innsbruck => "Innsbruck", City::Karlsruhe => "Karlsruhe", City::Katowice => "Katowice", City::Kiel => "Kiel", City::Kosice => "Košice", City::Krakow => "Krakow", City::LAquila => "L'Aquila", City::LaRochelle => "La Rochelle", City::LaSpezia => "La Spezia", City::Lausanne => "Lausanne", City::LeMans => "Le Mans", City::Leeds => "Leeds", City::Leicester => "Leicester", City::Leipzig => "Leipzig", City::Lille => "Lille", City::Limerick => "Limerick", City::Limoges => "Limoges", City::Linkoping => "Linköping", City::Linz => "Linz", City::Lisbon => "Lisbon", City::Liverpool => "Liverpool", City::Livorno => "Livorno", City::Liege => "Liège", City::Ljubljana => "Ljubljana", City::Logrono => "Logroño", City::London => "London", City::Lublin => "Lublin", City::Lucerne => "Lucerne", City::Lugano => "Lugano", City::Lund => "Lund", City::Lyon => "Lyon", City::Madrid => "Madrid", City::Malmo => "Malmö", City::Manchester => "Manchester", City::Mannheim => "Mannheim", City::Maribor => "Maribor", City::Marseille => "Marseille", City::Messina => "Messina", City::Metz => "Metz", City::Milan => "Milan", City::Modena => "Modena", City::Montpellier => "Montpellier", City::Mostar => "Mostar", City::Mulhouse => "Mulhouse", City::Munich => "Munich", City::Murcia => "Murcia", City::Malaga => "Málaga", City::Nancy => "Nancy", City::Nantes => "Nantes", City::Naples => "Naples", City::Narbonne => "Narbonne", City::Newcastle => "Newcastle", City::Nice => "Nice", City::Nis => "Niš", City::Norrkoping => "Norrköping", City::Nottingham => "Nottingham", City::NoviSad => "Novi Sad", City::Nuremberg => "Nuremberg", City::Nimes => "Nîmes", City::Odense => "Odense", City::Oslo => "Oslo", City::Ostrava => "Ostrava", City::Oulu => "Oulu", City::Oviedo => "Oviedo", City::Oxford => "Oxford", City::Padua => "Padua", City::Palermo => "Palermo", City::Pamplona => "Pamplona", City::Paris => "Paris", City::Parma => "Parma", City::Perpignan => "Perpignan", City::Perugia => "Perugia", City::Piacenza => "Piacenza", City::Pisa => "Pisa", City::Plovdiv => "Plovdiv", City::Plymouth => "Plymouth", City::Plzen => "Plzeň", City::Poitiers => "Poitiers", City::Porto => "Porto", City::Portsmouth => "Portsmouth", City::Potsdam => "Potsdam", City::Poznan => "Poznań", City::Prague => "Prague", City::Pecs => "Pécs", City::Ravenna => "Ravenna", City::Reading => "Reading", City::Regensburg => "Regensburg", City::ReggioCalabria => "Reggio Calabria", City::ReggioEmilia => "Reggio Emilia", City::Reims => "Reims", City::Rennes => "Rennes", City::Rijeka => "Rijeka", City::Rimini => "Rimini", City::Rome => "Rome", City::Rostock => "Rostock", City::Rotterdam => "Rotterdam", City::Rouen => "Rouen", City::Salamanca => "Salamanca", City::Salerno => "Salerno", City::Salzburg => "Salzburg", City::SanSebastian => "San Sebastián", City::Santander => "Santander", City::SantiagoDeCompostela => "Santiago de Compostela", City::Sarajevo => "Sarajevo", City::Seville => "Seville", City::Sheffield => "Sheffield", City::Sofia => "Sofia", City::Sopot => "Sopot", City::Southampton => "Southampton", City::Split => "Split", City::StGallen => "St. Gallen", City::Stavanger => "Stavanger", City::Stockholm => "Stockholm", City::Strasbourg => "Strasbourg", City::Stuttgart => "Stuttgart", City::Swansea => "Swansea", City::Szeged => "Szeged", City::Tampere => "Tampere", City::Taranto => "Taranto", City::Terni => "Terni", City::TheHague => "The Hague", City::Thessaloniki => "Thessaloniki", City::Timisoara => "Timișoara", City::Toledo => "Toledo", City::Toulouse => "Toulouse", City::Tours => "Tours", City::Trento => "Trento", City::Trieste => "Trieste", City::Trondheim => "Trondheim", City::Turin => "Turin", City::Turku => "Turku", City::Uppsala => "Uppsala", City::Utrecht => "Utrecht", City::Valencia => "Valencia", City::Valladolid => "Valladolid", City::Varna => "Varna", City::Venice => "Venice", City::Verona => "Verona", City::Versailles => "Versailles", City::Vicenza => "Vicenza", City::Vienna => "Vienna", City::Vigo => "Vigo", City::Vasteras => "Västerås", City::Warsaw => "Warsaw", City::Waterford => "Waterford", City::Wroclaw => "Wrocław", City::Wurzburg => "Würzburg", City::York => "York", City::Zagreb => "Zagreb", City::Zaragoza => "Zaragoza", City::Zurich => "Zurich", City::Orebro => "Örebro", City::CeskeBudejovice => "České Budějovice", City::Lodz => "Łódź"}}} 
impl FromStr for City {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {match s {"A Coruña" => Ok(City::ACoruna), "Aalborg" => Ok(City::Aalborg), "Aarhus" => Ok(City::Aarhus), "Alicante" => Ok(City::Alicante), "Almería" => Ok(City::Almeria), "Amiens" => Ok(City::Amiens), "Amsterdam" => Ok(City::Amsterdam), "Ancona" => Ok(City::Ancona), "Angers" => Ok(City::Angers), "Annecy" => Ok(City::Annecy), "Antwerp" => Ok(City::Antwerp), "Arezzo" => Ok(City::Arezzo), "Ashford" => Ok(City::Ashford), "Augsburg" => Ok(City::Augsburg), "Avignon" => Ok(City::Avignon), "Badajoz" => Ok(City::Badajoz), "Barcelona" => Ok(City::Barcelona), "Bari" => Ok(City::Bari), "Basel" => Ok(City::Basel), "Belgrade" => Ok(City::Belgrade), "Bergamo" => Ok(City::Bergamo), "Bergen" => Ok(City::Bergen), "Berlin" => Ok(City::Berlin), "Bern" => Ok(City::Bern), "Besançon" => Ok(City::Besancon), "Bilbao" => Ok(City::Bilbao), "Birmingham" => Ok(City::Birmingham), "Bochum" => Ok(City::Bochum), "Bologna" => Ok(City::Bologna), "Bolzano" => Ok(City::Bolzano), "Bonn" => Ok(City::Bonn), "Bordeaux" => Ok(City::Bordeaux), "Bratislava" => Ok(City::Bratislava), "Brașov" => Ok(City::Brasov), "Bremen" => Ok(City::Bremen), "Brescia" => Ok(City::Brescia), "Brest" => Ok(City::Brest), "Brighton" => Ok(City::Brighton), "Brindisi" => Ok(City::Brindisi), "Bristol" => Ok(City::Bristol), "Brno" => Ok(City::Brno), "Bruges" => Ok(City::Bruges), "Brussels" => Ok(City::Brussels), "Bucharest" => Ok(City::Bucharest), "Budapest" => Ok(City::Budapest), "Burgas" => Ok(City::Burgas), "Burgos" => Ok(City::Burgos), "Calais" => Ok(City::Calais), "Cambridge" => Ok(City::Cambridge), "Cardiff" => Ok(City::Cardiff), "Cartagena" => Ok(City::Cartagena), "Catania" => Ok(City::Catania), "Chambéry" => Ok(City::Chambery), "Clermont-Ferrand" => Ok(City::ClermontFerrand), "Cluj-Napoca" => Ok(City::ClujNapoca), "Cologne" => Ok(City::Cologne), "Como" => Ok(City::Como), "Copenhagen" => Ok(City::Copenhagen), "Cork" => Ok(City::Cork), "Cuenca" => Ok(City::Cuenca), "Cádiz" => Ok(City::Cadiz), "Córdoba" => Ok(City::Cordoba), "Debrecen" => Ok(City::Debrecen), "Derby" => Ok(City::Derby), "Dijon" => Ok(City::Dijon), "Dortmund" => Ok(City::Dortmund), "Drammen" => Ok(City::Drammen), "Dresden" => Ok(City::Dresden), "Dublin" => Ok(City::Dublin), "Düsseldorf" => Ok(City::Dusseldorf), "Edinburgh" => Ok(City::Edinburgh), "Eindhoven" => Ok(City::Eindhoven), "Essen" => Ok(City::Essen), "Exeter" => Ok(City::Exeter), "Ferrara" => Ok(City::Ferrara), "Florence" => Ok(City::Florence), "Forlì" => Ok(City::Forli), "Frankfurt" => Ok(City::Frankfurt), "Galway" => Ok(City::Galway), "Gdańsk" => Ok(City::Gdansk), "Gdynia" => Ok(City::Gdynia), "Geneva" => Ok(City::Geneva), "Genoa" => Ok(City::Genoa), "Ghent" => Ok(City::Ghent), "Glasgow" => Ok(City::Glasgow), "Gothenburg" => Ok(City::Gothenburg), "Granada" => Ok(City::Granada), "Graz" => Ok(City::Graz), "Grenoble" => Ok(City::Grenoble), "Hamburg" => Ok(City::Hamburg), "Hannover" => Ok(City::Hannover), "Heidelberg" => Ok(City::Heidelberg), "Helsingborg" => Ok(City::Helsingborg), "Helsinki" => Ok(City::Helsinki), "Iași" => Ok(City::Iasi), "Innsbruck" => Ok(City::Innsbruck), "Karlsruhe" => Ok(City::Karlsruhe), "Katowice" => Ok(City::Katowice), "Kiel" => Ok(City::Kiel), "Košice" => Ok(City::Kosice), "Krakow" => Ok(City::Krakow), "L'Aquila" => Ok(City::LAquila), "La Rochelle" => Ok(City::LaRochelle), "La Spezia" => Ok(City::LaSpezia), "Lausanne" => Ok(City::Lausanne), "Le Mans" => Ok(City::LeMans), "Leeds" => Ok(City::Leeds), "Leicester" => Ok(City::Leicester), "Leipzig" => Ok(City::Leipzig), "Lille" => Ok(City::Lille), "Limerick" => Ok(City::Limerick), "Limoges" => Ok(City::Limoges), "Linköping" => Ok(City::Linkoping), "Linz" => Ok(City::Linz), "Lisbon" => Ok(City::Lisbon), "Liverpool" => Ok(City::Liverpool), "Livorno" => Ok(City::Livorno), "Liège" => Ok(City::Liege), "Ljubljana" => Ok(City::Ljubljana), "Logroño" => Ok(City::Logrono), "London" => Ok(City::London), "Lublin" => Ok(City::Lublin), "Lucerne" => Ok(City::Lucerne), "Lugano" => Ok(City::Lugano), "Lund" => Ok(City::Lund), "Lyon" => Ok(City::Lyon), "Madrid" => Ok(City::Madrid), "Malmö" => Ok(City::Malmo), "Manchester" => Ok(City::Manchester), "Mannheim" => Ok(City::Mannheim), "Maribor" => Ok(City::Maribor), "Marseille" => Ok(City::Marseille), "Messina" => Ok(City::Messina), "Metz" => Ok(City::Metz), "Milan" => Ok(City::Milan), "Modena" => Ok(City::Modena), "Montpellier" => Ok(City::Montpellier), "Mostar" => Ok(City::Mostar), "Mulhouse" => Ok(City::Mulhouse), "Munich" => Ok(City::Munich), "Murcia" => Ok(City::Murcia), "Málaga" => Ok(City::Malaga), "Nancy" => Ok(City::Nancy), "Nantes" => Ok(City::Nantes), "Naples" => Ok(City::Naples), "Narbonne" => Ok(City::Narbonne), "Newcastle" => Ok(City::Newcastle), "Nice" => Ok(City::Nice), "Niš" => Ok(City::Nis), "Norrköping" => Ok(City::Norrkoping), "Nottingham" => Ok(City::Nottingham), "Novi Sad" => Ok(City::NoviSad), "Nuremberg" => Ok(City::Nuremberg), "Nîmes" => Ok(City::Nimes), "Odense" => Ok(City::Odense), "Oslo" => Ok(City::Oslo), "Ostrava" => Ok(City::Ostrava), "Oulu" => Ok(City::Oulu), "Oviedo" => Ok(City::Oviedo), "Oxford" => Ok(City::Oxford), "Padua" => Ok(City::Padua), "Palermo" => Ok(City::Palermo), "Pamplona" => Ok(City::Pamplona), "Paris" => Ok(City::Paris), "Parma" => Ok(City::Parma), "Perpignan" => Ok(City::Perpignan), "Perugia" => Ok(City::Perugia), "Piacenza" => Ok(City::Piacenza), "Pisa" => Ok(City::Pisa), "Plovdiv" => Ok(City::Plovdiv), "Plymouth" => Ok(City::Plymouth), "Plzeň" => Ok(City::Plzen), "Poitiers" => Ok(City::Poitiers), "Porto" => Ok(City::Porto), "Portsmouth" => Ok(City::Portsmouth), "Potsdam" => Ok(City::Potsdam), "Poznań" => Ok(City::Poznan), "Prague" => Ok(City::Prague), "Pécs" => Ok(City::Pecs), "Ravenna" => Ok(City::Ravenna), "Reading" => Ok(City::Reading), "Regensburg" => Ok(City::Regensburg), "Reggio Calabria" => Ok(City::ReggioCalabria), "Reggio Emilia" => Ok(City::ReggioEmilia), "Reims" => Ok(City::Reims), "Rennes" => Ok(City::Rennes), "Rijeka" => Ok(City::Rijeka), "Rimini" => Ok(City::Rimini), "Rome" => Ok(City::Rome), "Rostock" => Ok(City::Rostock), "Rotterdam" => Ok(City::Rotterdam), "Rouen" => Ok(City::Rouen), "Salamanca" => Ok(City::Salamanca), "Salerno" => Ok(City::Salerno), "Salzburg" => Ok(City::Salzburg), "San Sebastián" => Ok(City::SanSebastian), "Santander" => Ok(City::Santander), "Santiago de Compostela" => Ok(City::SantiagoDeCompostela), "Sarajevo" => Ok(City::Sarajevo), "Seville" => Ok(City::Seville), "Sheffield" => Ok(City::Sheffield), "Sofia" => Ok(City::Sofia), "Sopot" => Ok(City::Sopot), "Southampton" => Ok(City::Southampton), "Split" => Ok(City::Split), "St. Gallen" => Ok(City::StGallen), "Stavanger" => Ok(City::Stavanger), "Stockholm" => Ok(City::Stockholm), "Strasbourg" => Ok(City::Strasbourg), "Stuttgart" => Ok(City::Stuttgart), "Swansea" => Ok(City::Swansea), "Szeged" => Ok(City::Szeged), "Tampere" => Ok(City::Tampere), "Taranto" => Ok(City::Taranto), "Terni" => Ok(City::Terni), "The Hague" => Ok(City::TheHague), "Thessaloniki" => Ok(City::Thessaloniki), "Timișoara" => Ok(City::Timisoara), "Toledo" => Ok(City::Toledo), "Toulouse" => Ok(City::Toulouse), "Tours" => Ok(City::Tours), "Trento" => Ok(City::Trento), "Trieste" => Ok(City::Trieste), "Trondheim" => Ok(City::Trondheim), "Turin" => Ok(City::Turin), "Turku" => Ok(City::Turku), "Uppsala" => Ok(City::Uppsala), "Utrecht" => Ok(City::Utrecht), "Valencia" => Ok(City::Valencia), "Valladolid" => Ok(City::Valladolid), "Varna" => Ok(City::Varna), "Venice" => Ok(City::Venice), "Verona" => Ok(City::Verona), "Versailles" => Ok(City::Versailles), "Vicenza" => Ok(City::Vicenza), "Vienna" => Ok(City::Vienna), "Vigo" => Ok(City::Vigo), "Västerås" => Ok(City::Vasteras), "Warsaw" => Ok(City::Warsaw), "Waterford" => Ok(City::Waterford), "Wrocław" => Ok(City::Wroclaw), "Würzburg" => Ok(City::Wurzburg), "York" => Ok(City::York), "Zagreb" => Ok(City::Zagreb), "Zaragoza" => Ok(City::Zaragoza), "Zurich" => Ok(City::Zurich), "Örebro" => Ok(City::Orebro), "České Budějovice" => Ok(City::CeskeBudejovice), "Łódź" => Ok(City::Lodz), _ => Err(format!("Unknown city: {}", s))}}
}
pub fn get_all_city_names() -> Vec<&'static str> {
    use strum::IntoEnumIterator;
    City::iter().map(|city| city.as_str()).collect()
}

// -- TRAIN -- \\

#[derive(Clone, Debug, EnumIter, AsRefStr, Serialize, Deserialize)]
pub enum Train {AVE, EuroCity, Eurostar, Frecciarossa, IC, ICE, InterCity, Intercites, Italo, Nightjet, RE, RJX, Railjet, RegioExpress, TER, TGV, Thalys} 
impl Train {pub fn as_str(&self) -> &'static str {match self {Train::AVE => "AVE", Train::EuroCity => "EuroCity", Train::Eurostar => "Eurostar", Train::Frecciarossa => "Frecciarossa", Train::IC => "IC", Train::ICE => "ICE", Train::InterCity => "InterCity", Train::Intercites => "Intercités", Train::Italo => "Italo", Train::Nightjet => "Nightjet", Train::RE => "RE", Train::RJX => "RJX", Train::Railjet => "Railjet", Train::RegioExpress => "RegioExpress", Train::TER => "TER", Train::TGV => "TGV", Train::Thalys => "Thalys"}}} 
impl FromStr for Train {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {match s {"AVE" => Ok(Train::AVE), "EuroCity" => Ok(Train::EuroCity), "Eurostar" => Ok(Train::Eurostar), "Frecciarossa" => Ok(Train::Frecciarossa), "IC" => Ok(Train::IC), "ICE" => Ok(Train::ICE), "InterCity" => Ok(Train::InterCity), "Intercités" => Ok(Train::Intercites), "Italo" => Ok(Train::Italo), "Nightjet" => Ok(Train::Nightjet), "RE" => Ok(Train::RE), "RJX" => Ok(Train::RJX), "Railjet" => Ok(Train::Railjet), "RegioExpress" => Ok(Train::RegioExpress), "TER" => Ok(Train::TER), "TGV" => Ok(Train::TGV), "Thalys" => Ok(Train::Thalys), _ => Err(format!("Unknown train type: {}", s))}}
}

pub fn get_all_train_names() -> Vec<&'static str> {
    use strum::IntoEnumIterator;
    Train::iter().map(|train| train.as_str()).collect()
}

// -- DAY -- \\

#[derive(Clone, Debug, EnumIter, AsRefStr, Serialize, Deserialize)]
pub enum Day {Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday} 
impl Day {
    pub fn as_str(&self) -> &'static str {match self {Day::Monday => "Monday", Day::Tuesday => "Tuesday", Day::Wednesday => "Wednesday", Day::Thursday => "Thursday", Day::Friday => "Friday", Day::Saturday => "Saturday", Day::Sunday => "Sunday"}}
    pub fn daily() -> Vec<Day> {vec![Day::Monday, Day::Tuesday, Day::Wednesday, Day::Thursday, Day::Friday, Day::Saturday, Day::Sunday]}
} impl FromStr for Day {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {match s {"Mon" => Ok(Day::Monday), "Monday" => Ok(Day::Monday), "Tue" => Ok(Day::Tuesday), "Tuesday" => Ok(Day::Tuesday), "Wed" => Ok(Day::Wednesday), "Wednesday" => Ok(Day::Wednesday), "Thu" => Ok(Day::Thursday), "Thursday" => Ok(Day::Thursday), "Fri" => Ok(Day::Friday), "Friday" => Ok(Day::Friday), "Sat" => Ok(Day::Saturday), "Saturday" => Ok(Day::Saturday), "Sun" => Ok(Day::Sunday), "Sunday" => Ok(Day::Sunday), _ => Err(format!("Unknown day: {}", s))}}
}
