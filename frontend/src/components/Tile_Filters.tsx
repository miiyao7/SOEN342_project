import { all } from 'axios';
import React, { use, useState } from 'react';
import { server } from 'typescript';

type Filters = {
  CityDeparture: string;
  CityArrival: string;
  DepartureTime: string;
  ArrivalTime: string;
  ArrivalTimeFrom: string;
  TrainType: string;
  SelectedDay: string;
  TicketClass: string;
  Price: Number;
  Transferring: boolean;
  Minutes: Number;
};

interface FilterTileProps {
  onFiltered: (data: {}) => void; // Callback prop type
  validTrains: string[];
  validCities: string[];
}

const FilterTile: React.FC<FilterTileProps> = ({ onFiltered, validCities, validTrains }) => {
  const [allfilters, setAllFilters] = useState<Filters>({ CityDeparture: "", CityArrival: "", DepartureTime: "", ArrivalTime: "", ArrivalTimeFrom: "", TrainType: "", SelectedDay: "", TicketClass: "", Price: 0.00, Transferring: true, Minutes: 0});
  const [errors, setErrors] = useState({
    cityDeparture: "",
    cityArrival: "",
    departureTime: "",
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

    //console.log("DEBUG {FILTER}:\r", activeFilters);

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
      setErrors(prev => ({ ...prev, cityDeparture: "Invalid" })); setHasError(true);
    } else if (name === "CityArrival" && !validCities.includes(value)) {
      setErrors(prev => ({ ...prev, cityArrival: "Invalid" })); setHasError(true);
    } else if (name === "DepartureTime" && !isInteger(value)) {
      setErrors(prev => ({ ...prev, departureTime: "Invalid" })); setHasError(true);
    } else if (name === "TrainType" && !validTrains.includes(value)) {
      setErrors(prev => ({ ...prev, trainType: "Invalid" })); setHasError(true);
    } else if (name === "Minutes" && !isInteger(value)) {
      setErrors(prev => ({ ...prev, minutes: "Invalid" })); setHasError(true);
    } else if (name === "Price" && !isInteger(value)) {
      setErrors(prev => ({ ...prev, price: "Invalid" })); setHasError(true);
    } else {
      setHasError(false);
      setErrors(prev => ({ ...prev, cityDeparture: "", cityArrival: "", trainType: "", departureTime: "", price: "", minutes: "", general: "" }));
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
              <div className="card"><label htmlFor="DepartureTime" className={errors.departureTime ? "has-error" : ""}>Departure Time</label>
                <input type="text" className="DepartureTime" name="DepartureTime" onChange={handleFilterChange}></input>
              </div>
              <div className="card checkbox">
                <label className={errors.minutes ? "has-error" : ""}><input type="checkbox" name="Transferring" onChange={handleFilterChange} defaultChecked={true} /> 
                 {allfilters.Transferring ? "Transfer Minutes:" : "  Transferring?"}</label>
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
              <div className="card">
                <label htmlFor="TicketClass">Ticket Class</label>
                <select id="TicketClass" name="TicketClass" onChange={handleSelectChange} defaultValue="None">
                  <option value="None">None</option>
                  <option value="First">First Class</option>
                  <option value="Second">Second Class</option>
                </select>
              </div>
              <div className="card"><label htmlFor="TrainType" className={errors.trainType ? "has-error" : ""}>Train Type</label>
                <input list="TrainType" className="optionSearch" name="TrainType" onChange={handleFilterChange}></input>
                <datalist id="TrainType">
                  {validTrains.map(train => <option key={train} value={train} />)}
                </datalist></div>
              <div className="card">
                <label htmlFor="Price" className={errors.price ? "has-error" : ""}>Max Price (€)</label>
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