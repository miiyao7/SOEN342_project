mod rail_network;
mod search_functionality;
mod display;

use rail_network::parse_CSV;
use crate::search_functionality::{search_itineraries, SearchFunctionality, TicketClass, SortBy};
use display::print_itineraries;

//fn main() {println!("{:?}", rail_network::parse_CSV())}
// testing purposes only
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let routes = parse_CSV()?;

    let q = SearchFunctionality {
        departure_city: Some("Paris"),
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

    let itineraries = search_itineraries(&routes, &q);
    print_itineraries(&routes, &itineraries);
    Ok(())
}