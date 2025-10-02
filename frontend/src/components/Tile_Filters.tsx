import React, { useState } from 'react';

interface FilterTileProps {
  data: JSON; // Callback prop type
  loading: (data: any) => void; // Callback prop type
}


const FilterTile: React.FC<FilterTileProps> = ({ data, loading }) => {
    const [filterCityDeparture, setFilterCityDeparture] = useState(null);
    const [filterCityArrival, setFilterCityArrival] = useState(null);
    const [filterDepartureTime, setFilterDepartureTime] = useState(null);
    const [filterArrivalTime, setFilterArrivalTime] = useState(null);
    const [filterTrainType, setFilterTrainType] = useState(null);
    const [filterOperationDays, setFilterOperationDays] = useState(null);
    const [filter1stRates, setFilter1stRates] = useState(null);
    const [filter2ndRates, setFilter2ndRates] = useState(null);

    return (    
      <div className="form-tile filters">     
        <h2>Filters</h2>             
        <form>
          <div className="form-group">  
            <div className="cardContainer">
              <div className="card"><label htmlFor="username">Departure City</label>
                <select id="CityDeparture" name="CityDeparture">
                  <option value="au">Australia</option>
                  <option value="ca">Canada</option>
                  <option value="usa">USA</option>
                </select></div>
              <div className="card"><label htmlFor="username">Arrival City</label>
                <select id="CityArrival" name="CityArrival">
                  <option value="au">Australia</option>
                  <option value="ca">Canada</option>
                  <option value="usa">USA</option>
                </select></div>
              <div className="card"><label htmlFor="username">Departure Time</label>
                <select id="DepartureTime" name="DepartureTime">
                  <option value="au">1:00</option>
                  <option value="ca">5:40</option>
                  <option value="usa">8:20</option>
                </select></div>
              <div className="card"><label htmlFor="username">Arrival Time</label>
                <select id="ArrivalTime" name="ArrivalTime">
                  <option value="au">1:00</option>
                  <option value="ca">5:40</option>
                  <option value="usa">8:20</option>
                </select></div>
              <div className="card"><label htmlFor="username">Train Type</label>
                <select id="TrainType" name="TrainType">
                  <option value="au">1:00</option>
                  <option value="ca">5:40</option>
                  <option value="usa">8:20</option>
                </select></div>
              <div className="card"><label htmlFor="username">Days of Operation</label>
                <select id="OperationDays" name="OperationDays">
                  <option value="au">10</option>
                  <option value="ca">540</option>
                  <option value="usa">82</option>
                </select></div>
              <div className="card">
                <label htmlFor="username">Ticket Rates (₤): 1st Class</label>
                <select id="1stRates" name="1stRates">
                  <option value="au">10</option>
                  <option value="ca">540</option>
                  <option value="usa">82</option>
                </select>
              </div>
              <div className="card">
                <label htmlFor="username">Ticket Rates (₤): 2nd Class</label>
                <select id="2ndRates" name="2ndRates">
                  <option value="au">10</option>
                  <option value="ca">540</option>
                  <option value="usa">82</option>
                </select>
              </div>
            </div>
          </div>
        </form>
      </div> 
    )
}
export default FilterTile;