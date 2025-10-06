import { all } from 'axios';
import React, { use, useState } from 'react';
import { server } from 'typescript';

type Filters = {
  CityDeparture: string;
  CityArrival: string;
  DepartureTime: string;
  ArrivalTime: string;
  TrainType: string;
  SelectedDay: string;
  TicketClass: string;
  Price: Number;
  Transferring: boolean;
  Minutes: Number;
};

interface FilterTileProps {
  onFiltered: (data: {}) => void; // Callback prop type
  handler: (data: any) => void; // Callback prop type
}

const validTrains = ["AVE", "EuroCity", "Eurostar", "Frecciarossa", "IC", "ICE", "InterCity", "Intercites", "Italo", "Nightjet", "RE", "RJX", "Railjet", "RegioExpress", "TER", "TGV", "Thalys"];
const validCities = ["ACoruna", "Aalborg", "Aarhus", "Alicante", "Almeria", "Amiens", "Amsterdam", "Ancona", "Angers", "Annecy", "Antwerp", "Arezzo", "Ashford", "Augsburg", "Avignon", "Badajoz", "Barcelona", "Bari", "Basel", "Belgrade", "Bergamo", "Bergen", "Berlin", "Bern", "Besancon", "Bilbao", "Birmingham", "Bochum", "Bologna", "Bolzano", "Bonn", "Bordeaux", "Bratislava", "Brasov", "Bremen", "Brescia", "Brest", "Brighton", "Brindisi", "Bristol", "Brno", "Bruges", "Brussels", "Bucharest", "Budapest", "Burgas", "Burgos", "Calais", "Cambridge", "Cardiff", "Cartagena", "Catania", "Chambery", "ClermontFerrand", "ClujNapoca", "Cologne", "Como", "Copenhagen", "Cork", "Cuenca", "Cadiz", "Cordoba", "Debrecen", "Derby", "Dijon", "Dortmund", "Drammen", "Dresden", "Dublin", "Dusseldorf", "Edinburgh", "Eindhoven", "Essen", "Exeter", "Ferrara", "Florence", "Forli", "Frankfurt", "Galway", "Gdansk", "Gdynia", "Geneva", "Genoa", "Ghent", "Glasgow", "Gothenburg", "Granada", "Graz", "Grenoble", "Hamburg", "Hannover", "Heidelberg", "Helsingborg", "Helsinki", "Iasi", "Innsbruck", "Karlsruhe", "Katowice", "Kiel", "Kosice", "Krakow", "LAquila", "LaRochelle", "LaSpezia", "Lausanne", "LeMans", "Leeds", "Leicester", "Leipzig", "Lille", "Limerick", "Limoges", "Linkoping", "Linz", "Lisbon", "Liverpool", "Livorno", "Liege", "Ljubljana", "Logrono", "London", "Lublin", "Lucerne", "Lugano", "Lund", "Lyon", "Madrid", "Malmo", "Manchester", "Mannheim", "Maribor", "Marseille", "Messina", "Metz", "Milan", "Modena", "Montpellier", "Mostar", "Mulhouse", "Munich", "Murcia", "Malaga", "Nancy", "Nantes", "Naples", "Narbonne", "Newcastle", "Nice", "Nis", "Norrkoping", "Nottingham", "NoviSad", "Nuremberg", "Nimes", "Odense", "Oslo", "Ostrava", "Oulu", "Oviedo", "Oxford", "Padua", "Palermo", "Pamplona", "Paris", "Parma", "Perpignan", "Perugia", "Piacenza", "Pisa", "Plovdiv", "Plymouth", "Plzen", "Poitiers", "Porto", "Portsmouth", "Potsdam", "Poznan", "Prague", "Pecs", "Ravenna", "Reading", "Regensburg", "ReggioCalabria", "ReggioEmilia", "Reims", "Rennes", "Rijeka", "Rimini", "Rome", "Rostock", "Rotterdam", "Rouen", "Salamanca", "Salerno", "Salzburg", "SanSebastian", "Santander", "SantiagoDeCompostel", "Sarajevo", "Seville", "Sheffield", "Sofia", "Sopot", "Southampton", "Split", "StGallen", "Stavanger", "Stockholm", "Strasbourg", "Stuttgart", "Swansea", "Szeged", "Tampere", "Taranto", "Terni", "TheHague", "Thessaloniki", "Timisoara", "Toledo", "Toulouse", "Tours", "Trento", "Trieste", "Trondheim", "Turin", "Turku", "Uppsala", "Utrecht", "Valencia", "Valladolid", "Varna", "Venice", "Verona", "Versailles", "Vicenza", "Vienna", "Vigo", "Vasteras", "Warsaw", "Waterford", "Wroclaw", "Wurzburg", "York", "Zagreb", "Zaragoza", "Zurich", "Orebro", "CeskeBudejovice", "Lodz"];


const FilterTile: React.FC<FilterTileProps> = ({ onFiltered, handler }) => {
  const [allfilters, setAllFilters] = useState<Filters>({ CityDeparture: "", CityArrival: "", DepartureTime: "", ArrivalTime: "", TrainType: "", SelectedDay: "", TicketClass: "", Price: 0.00, Transferring: false, Minutes: 0});
  const [errors, setErrors] = useState({
    cityDeparture: "",
    cityArrival: "",
    trainType: "",
    minutes: "",
    price: "",
    general: "",
  });
  const [hasError, setHasError] = useState(Boolean);
  const [tooltip, setTooltip] = useState<String>("None");

 const setFilters = () =>{
    const activeFilters = Object.fromEntries(
      Object.entries(allfilters).filter(([key, value]) =>
        value !== "" && value !== null && !(Array.isArray(value) && value.length === 0)
      )
    );

    const isEmpty = Object.keys(activeFilters).length === 0;

    console.log("DEBUG {FILTER}:\r", activeFilters);

    if (!hasError) {
      onFiltered(activeFilters);
    }
  }
   const isInteger = (value: string) =>{
    if(/^\d+$/.test(value) && parseInt(value) > 0){ return true; }
    return false;
  }
  const validation = (name: any, value: any) => {    
    // Validation
    if(value.trim() == "" || value == null){
      setHasError(false);
      setErrors(prev => ({ ...prev, cityDeparture: "", cityArrival: "", trainType: "", price: "", minutes: "", general: "" }));
    } else if (name === "CityDeparture" && !validCities.includes(value)) {
      setErrors(prev => ({ ...prev, cityDeparture: "Invalid" }));
      setHasError(true);
    } else if (name === "CityArrival" && !validCities.includes(value)) {
      setErrors(prev => ({ ...prev, cityArrival: "Invalid" }));
      setHasError(true);
    } else if (name === "TrainType" && !validTrains.includes(value)) {
      setErrors(prev => ({ ...prev, trainType: "Invalid" }));
      setHasError(true);
    } else if (name === "Minutes" && !isInteger(value)) {
      setErrors(prev => ({ ...prev, minutes: "Invalid" }));
      setHasError(true);
    } else if (name === "Price" && !isInteger(value)) {
      setErrors(prev => ({ ...prev, price: "Invalid" }));
      setHasError(true);
    } else {
      setHasError(false);
      setErrors(prev => ({ ...prev, cityDeparture: "", cityArrival: "", trainType: "", price: "", minutes: "", general: "" }));
    }
  }
  const handleFilterChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;
    const isChecked = event.target.checked;
    if (name == "Transferring"){
        setAllFilters(prev => ({
          ...prev,
          [name]: isChecked,
        }));
    } else if(isInteger(value)) {
      setAllFilters(prev => ({
        ...prev,
        [name]: parseInt(value),
      }));
      validation(name, value);
    } else {
      setAllFilters(prev => ({
        ...prev,
        [name]: value,
      }));
      validation(name, value);
    }
  };
  const handleSelectChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    let selected = event.target.value;
    let name = event.target.name;
    setAllFilters(prev => ({
      ...prev,
      [name]: selected,
    }));
    /* - if(selected.length == 7){
      setTooltip("All Week");
    } else {
      setTooltip(
        selected.map(item => item.substring(0, 3)).join(", ")
      );
    }*/
    // console.log("Set: " + event.target.id + " "+ selected)
  };
    return (
      <div className="form-tile filters">
        <h2>Filters</h2>
        <form>
          <div className="form-group">
            <div className="cardContainer">
              <div className="card"><label htmlFor="CityDeparture" className={errors.cityDeparture ? "has-error" : ""}>Departure City</label>
                <input list="CityDeparture" className="optionSearch" name="CityDeparture" onChange={handleFilterChange}></input>
                <datalist id="CityDeparture">
                  {validCities.map(city => <option key={city} value={city} />)}
                </datalist></div>
              <div className="card"><label htmlFor="CityArrival" className={errors.cityArrival ? "has-error" : ""}>Arrival City</label>
                <input list="CityArrival" className="optionSearch" name="CityArrival" onChange={handleFilterChange}/>
                <datalist id="CityArrival">
                  {validCities.map(city => <option key={city} value={city} />)}
                </datalist></div>
              <div className="card"><label htmlFor="DepartureTime">Departure Time</label>
                <input list="DepartureTime" className="optionSearch" name="DepartureTime" onChange={handleFilterChange}></input>
                <datalist id="DepartureTime">
                  <option value="1:00"/>
                  <option value="2:00"/>
                  <option value="5:00"/>
                </datalist></div>
              <div className="card checkbox">
                <label className={errors.minutes ? "has-error" : ""}><input type="checkbox" name="Transferring" onChange={handleFilterChange} /> 
                 {allfilters.Transferring ? "Transfer Minutes:" : "Transferring?"}</label>
                 {allfilters.Transferring ? <input type="text" name="Minutes" onChange={handleFilterChange}></input> : <input type="text" disabled></input>}
              </div>
              <div className="card"><label htmlFor="SelectedDay">Week Day</label>
                <select id="SelectedDay" name="SelectedDay" onChange={handleSelectChange} defaultValue="None">
                  <option value="None">None</option>
                  <option value="Monday">Monday</option>
                  <option value="Tuesday">Tuesday</option>
                  <option value="Wednesday">Wednesday</option>
                  <option value="Thursday">Thursday</option>
                  <option value="Friday">Friday</option>
                  <option value="Saturday">Saturday</option>
                  <option value="Sunday">Sunday</option>
                </select></div>
              <div className="card"><label htmlFor="TrainType" className={errors.trainType ? "has-error" : ""}>Train Type</label>
                <input list="TrainType" className="optionSearch" name="TrainType" onChange={handleFilterChange}></input>
                <datalist id="TrainType">
                  {validTrains.map(train => <option key={train} value={train} />)}
                </datalist></div>
              <div className="card">
                <label htmlFor="TicketClass">Ticket Class</label>
                <select id="TicketClass" name="TicketClass" onChange={handleSelectChange} defaultValue="None">
                  <option value="None">None</option>
                  <option value="First">First Class</option>
                  <option value="Second">Second Class</option>
                </select>
              </div>
              <div className="card">
                <label htmlFor="Price" className={errors.price ? "has-error" : ""}>Rate (₤)</label>
                <input type="text" name="Price" onChange={handleFilterChange}></input>
              </div>
            </div>
          </div>
          <button type="button" className="filter-submit" onClick={setFilters}>FILTER</button>
        </form>
      </div>
    )
}
export default FilterTile;