import React, { useState } from 'react';

interface SorterTileProps {
  data: JSON; // Callback prop type
  loading: (data: any) => void; // Callback prop type
}

const SorterTile: React.FC<SorterTileProps> = ({ data, loading }) => {
    const [routes, setRoutes] = useState(null);


    const onFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {        
        const file = e.target.files?.[0];
        if (!file) return;

        const formData = new FormData();
        formData.append('file', file);

        const response = await fetch('http://localhost:3000/upload', {
        method: 'POST',
        body: formData,
        });

        const json = await response.json();
        setRoutes(json);
    };
    return (    
      <div className="form-tile sorters">     
        <h2>Sorters</h2>             
        <form>
          <div className="form-group">  
            <div className="cardContainer">
              <div className="pair"><input type="radio" id="CityDeparture" name="sorter"/><label htmlFor="CityDeparture">Departure City</label></div>
              <div className="pair"><input type="radio" id="CityArrival"   name="sorter"/><label htmlFor="CityArrival">Arrival City</label></div>
              <div className="pair"><input type="radio" id="TimeDeparture" name="sorter"/><label htmlFor="TimeDeparture">Departure Time</label></div>
              <div className="pair"><input type="radio" id="TimeArrival"   name="sorter"/><label htmlFor="TimeArrival">Arrival Time</label></div>
              <div className="pair"><input type="radio" id="TrainType"     name="sorter"/><label htmlFor="TrainType">Train Type</label></div>
              <div className="pair"><input type="radio" id="OperationDays" name="sorter"/><label htmlFor="OperationDays">Days of Operation</label></div>
              <div className="pair"><input type="radio" id="Rates1st"      name="sorter"/><label htmlFor="Rates1st">Ticket Rates (₤): 1st Class</label></div>
              <div className="pair"><input type="radio" id="Rates2nd"      name="sorter"/><label htmlFor="Rates2nd">Ticket Rates (₤): 2nd Class</label></div>
            </div>
          </div>
        </form>
      </div> 
    )
}
export default SorterTile;