mod rail_network;
mod display;

// use rail_network::parse_CSV;
use crate::rail_network::{RailNetwork, SearchFunctionality, TicketClass, SortBy};
use display::print_itineraries;

//fn main() {println!("{:?}", rail_network::parse_CSV())}
// testing purposes only
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rn = RailNetwork::new()?;

    let q = SearchFunctionality {
        departure_city: Some("Lyon"),
        arrival_city:   Some("London"),
        earliest_departure: None,
        train_type:     None,
        day_of_week:    None,
        price_range:    Some(TicketClass::SecondClass),
        max_price:      None,
        allowed_transfers: true,
        min_transfer_minutes: 5,
        sort_by: Some(SortBy::Duration),
    };

    let itineraries = rn.search(&q);
    print_itineraries(&rn.routes, &itineraries);
    Ok(())
}