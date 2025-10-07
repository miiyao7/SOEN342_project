import { match } from 'assert';
import React, { useState, useEffect } from 'react';
import Spinner from './Spinner';
interface DisplayTileProps {
  filterList: any; // Callback prop type
  loading: boolean;
}

const DisplayTile: React.FC<DisplayTileProps> = ({ filterList, loading }) => {
  const [sort, setSort] = useState<string>("");
  const [data, setData] = useState<any>(null);
  const [loader, setLoader] = useState<boolean>(loading);
  const [hasNoMatch, setHasNoMatch] = useState<boolean>(false);
  const Sorter = { sort_by: sort }
  const showSpinner = (time: number) => {
      setLoader(true);     
      setTimeout(() => {
        setLoader(false);   
      }, time);
    };  
  const Filters = {
      departure_city: filterList.CityDeparture || null,
      arrival_city: filterList.CityArrival || null,
      earliest_departure: filterList.DepartureTime || null, 
      train_type: filterList.TrainType || null,         
      day_of_week: filterList.SelectedDay || null,        
      price_range: filterList.TicketClass || null,        
      max_price: filterList.Price || null,
      allowed_transfers: filterList.Transferring || null,
      min_transfer_minutes: filterList.Minutes || null,
  }
  
  const handleSorterChange = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();

    const button: HTMLButtonElement = event.currentTarget;
    let sorterID = button.name;    
    setSort(sorterID);
    console.log("Sorter: " + sorterID);
  };

  useEffect(() => {
    const fetchData = async () => {
      setLoader(true);
      try {
          const response = await fetch("http://localhost:3001/handler/search", {
              method: "POST",
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({filters: Filters, sorter: Sorter})
          });

          if (!response.ok) {
              throw new Error(`Get failed: ${response.statusText}`);
          }

          showSpinner(5000);
          const filtered = await response.json();
          setData(filtered);
          //console.log("filtered: ", filtered);
        } catch (err) {
            console.error(err);
        } 
    };
    fetchData();
  }, [filterList, sort]);
      
  const renderTable = () => {
    if(data[0] == null){
      setHasNoMatch(true);
      return (<tbody></tbody>);
    } 
    let headers = Object.keys(data[0]);
    return (
        <tbody>
          {data.map((item: any, index: number) => (
            <tr key={index}>
              {headers.map((key) => (
                <td key={key}>
                  {typeof item[key] === "object" && item[key] !== null
                    ? JSON.stringify(item[key])
                    : String(item[key])}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
    )
  }
  const tableElement = React.useMemo(() => {
    if (!data) return null;
    return renderTable();
  }, [data]);
  return (
      <div className="form-tile display">
          <table className="result-table">
          <thead>
            <tr>
              <th>Connections</th>
              <th>Total Duration<span>
                <button name="Duration" className="sorter" onClick={handleSorterChange}>▲</button></span></th>
              <th>First Price<span>
                <button name="PriceAscendant1" className="sorter" onClick={handleSorterChange}>▲</button>
                <button name="PriceDescendant1" className="sorter" onClick={handleSorterChange}>▼</button></span></th>
              <th>Second Price<span>
                <button name="PriceAscendant2" className="sorter" onClick={handleSorterChange}>▲</button>
                <button name="PriceDescendant2" className="sorter" onClick={handleSorterChange}>▼</button></span></th>
              <th>Transfer Duration<span>
                <button name="TimeAscendant" className="sorter" onClick={handleSorterChange}>▲</button></span></th>
            </tr>
          </thead>            
            {tableElement}
          </table>
            {loader && <Spinner/>}
            {(hasNoMatch && !loader) && <div>No matches found</div>}
      </div>
  );
};

export default DisplayTile;
