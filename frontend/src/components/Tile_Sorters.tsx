import React, { useState } from 'react';

interface Sorter {
  label: string;
  value: string;
}

interface SorterTileProps {
  onSorted: (data: String) => void; // Callback prop type
  handler: (data: any) => void; // Callback prop type
}

const SorterTile: React.FC<SorterTileProps> = ({ onSorted, handler }) => {
  const [sorter, setSorter] = useState<String>("");

  const handleSorter = () => {
    onSorted(sorter);
  }

const handleSorterChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    let sorterID = event.target.id;    
    setSorter(sorterID);
    onSorted(sorter);
    //console.log("Sorter: " + sorterID);
  };
    return (    
      <div className="form-tile sorters">     
        <h2>Sorters</h2>             
        <form>
          <div className="form-group">  
            <div className="cardContainer">
              <div className="pair"><input type="radio" id="CityDeparture"    name="sorter" onChange={handleSorterChange}/><label htmlFor="CityDeparture">Departure City</label></div>
              <div className="pair"><input type="radio" id="CityArrival"      name="sorter" onChange={handleSorterChange}/><label htmlFor="CityArrival">Arrival City</label></div>
              <div className="pair"><input type="radio" id="TimeAscendant"    name="sorter" onChange={handleSorterChange}/><label htmlFor="TimeAscendant">⇗ Departure Time</label></div>
              <div className="pair"><input type="radio" id="TrainType"        name="sorter" onChange={handleSorterChange}/><label htmlFor="TrainType">Train Type</label></div>
              <div className="pair"><input type="radio" id="Day"              name="sorter" onChange={handleSorterChange}/><label htmlFor="Day">Week Day</label></div>
              <div className="pair"><input type="radio" id="TicketClass"     name="sorter" onChange={handleSorterChange}/><label htmlFor="TicketClass">Ticket Class</label></div>
              <div className="pair"><input type="radio" id="PriceDescendant" name="sorter" onChange={handleSorterChange}/><label htmlFor="PriceDescendant">Highest Price</label></div>
              <div className="pair"><input type="radio" id="PriceAscendant"  name="sorter" onChange={handleSorterChange}/><label htmlFor="PriceAscendant">Lowest Price</label></div>
            </div>
          </div>
        </form>
      </div> 
    )
}
export default SorterTile;