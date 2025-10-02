// TO USE THIS TO DISPLAY ITINERARIES TO REMOVE WHEN DONE TESTING
use chrono::{Duration, NaiveTime};
use crate::rail_network::Route;
use crate::search_functionality::Itinerary;

fn fmt_time(t: NaiveTime) -> String { t.format("%H:%M").to_string() }
fn fmt_duration(d: Duration) -> String {
    let m = d.num_minutes();
    format!("{}h {:02}m", m / 60, m % 60)
}

pub fn print_itineraries(routes: &[Route], itins: &[Itinerary]) {
    if itins.is_empty() {
        println!("No itineraries found.");
        return;
    }

    for (i, itin) in itins.iter().enumerate() {
        println!("================ Itinerary {} ================", i + 1);

        for (leg_idx, &ri) in itin.connections.iter().enumerate() {
            let r = &routes[ri];
            println!(
                "Connection {}: {} → {} | {}–{} | Train: {} | 1st: €{} | 2nd: €{}",
                leg_idx + 1,
                r.departure_city.as_str(),
                r.arrival_city.as_str(),
                fmt_time(r.departure_time),
                fmt_time(r.arrival_time),
                r.train_type.as_str(),
                r.first_class_ticket_rate,
                r.second_class_ticket_rate
            );

            if let Some(wait) = itin.transfer_duration.get(leg_idx) {
                println!("    Transfer wait: {} min", wait);
            }
        }

        println!(
            "TOTAL: {} | TOTAL 1st: €{} | TOTAL 2nd: €{}\n",
            fmt_duration(itin.total_duration),
            itin.total_first_price,
            itin.total_second_price
        );
    }
}
