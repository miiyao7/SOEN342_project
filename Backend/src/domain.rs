use serde::{Serialize, Deserialize};
#[derive(Clone, Debug, Deserialize)]
pub enum Train {AVE, EuroCity, Eurostar, Frecciarossa, IC, ICE, InterCity, Intercites, Italo, Nightjet, RE, RJX, Railjet, RegioExpress, TER, TGV, Thalys} 
#[derive(Clone, Debug, Deserialize)]
pub enum Day {Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday} 
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Route{
    pub idx: usize,
    pub departure_city: String,
    pub arrival_city: String,
    pub departure_time: String,
    pub arrival_time: String,
    pub train_type: String,
    pub days_of_operation: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItineraryResponse {
    pub total_duration: u32,
    pub total_price_first: u32,
    pub total_price_second: u32,
    pub total_transfers: u32,
    pub routes: Vec<Route>,
}
