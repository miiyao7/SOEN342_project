import { match } from 'assert';
import React, { useState, useEffect } from 'react';
import Spinner from './Spinner';
interface DisplayTileProps {
  filterList: any; // Callback prop type
  loading: boolean;
}

const DisplayTile: React.FC<DisplayTileProps> = ({ filterList, loading }) => {
  const [sort, setSort] = useState<string>("");
  const [sortChange, setSortChange] = useState<boolean>(false);
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
      arrival_time_from: filterList.ArrivalTimeFrom || null, 
      arrival_time_to: filterList.ArrivalTime || null, 
      train_type: filterList.TrainType || null,         
      day_of_week: filterList.SelectedDay || null,        
      price_range: filterList.TicketClass || null,        
      max_price: filterList.Price || null,
      allowed_transfers: filterList.Transferring || null,
      min_transfer_minutes: filterList.Minutes || null,
  }
  
  const handleSorterChange = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    setSortChange(true);  

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

          if(sortChange) {
            setLoader(false);   
          } else {showSpinner(3000);}
          const filtered = await response.json();
          setData(filtered);
          // console.log("filtered: ", filtered);
        } catch (err) {
            console.error(err);
        } 
    };
    fetchData();
  }, [filterList, sort]);
      
  const getWeekValue = (arr: any) => {
    let trimmedArr = arr.map((str: any) => str.slice(0, 3));
    if(arr.length == 7) return "All Week";
    return trimmedArr.join(", ");
  };
  const renderTable = () => {
    if(data == null){
      setHasNoMatch(true);
      return (<tbody></tbody>);
    } 
    let outerheaders = Object.keys(data[0]);
    let innerheaders = Object.keys(data[0].routes[0]);
    let headers = outerheaders.concat(innerheaders);
    console.log("DEBUG PATH", data[0].routes);
    console.log(innerheaders);
    console.log(outerheaders);
    return (
        <tbody>
          {data.map((item: any, idx: number) => (
          // For each top-level item in data...
          item.routes.length > 0 ? (
            item.routes.map((route: any, rIdx: any) => (
              <tr key={`${idx}-${rIdx}`}>
                {rIdx === 0 ? (
                  <>            
                    <td rowSpan={item.routes.length}>{item.total_duration}</td>      
                    <td rowSpan={item.routes.length}>{item.total_price_first}</td> 
                    <td rowSpan={item.routes.length}>{item.total_price_second}</td>
                    <td rowSpan={item.routes.length}>{item.total_transfers}</td>
                  </>
                ) : null}
                
                <td>{route.departure_city}</td>
                <td>{route.arrival_city}</td>
                <td>{route.departure_time}</td>
                <td>{route.arrival_time}</td>
                <td>{route.train_type}</td>
                <td>{getWeekValue(route.days_of_operation)}</td>
              </tr>
            ))
          ) : (
            // if no routes, show total info with empty route columns
            <tr key={idx}>
              <td>{item.total_duration}</td>      
              <td>{item.total_price_first}</td> 
              <td>{item.total_price_second}</td>
              <td>{item.total_transfers}</td>
              <td>-</td>
              <td>-</td>
              <td>-</td>
              <td>-</td>
              <td>-</td>
              <td>-</td>
              <td colSpan={3} style={{ textAlign: "center", fontStyle: "italic" }}>
                No routes
              </td>
            </tr>
          )
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
              <th>Total Duration<span>
                <button name="Duration" className="sorter" onClick={handleSorterChange}>▲</button></span></th>
              <th>First-Class Price<span>
                <button name="PriceAscendant1"  className="sorter" onClick={handleSorterChange}>▲</button>
                <button name="PriceDescendant1" className="sorter" onClick={handleSorterChange}>▼</button></span></th>
              <th>Second-Class Price<span>
                <button name="PriceAscendant2"  className="sorter" onClick={handleSorterChange}>▲</button>
                <button name="PriceDescendant2" className="sorter" onClick={handleSorterChange}>▼</button></span></th>
              <th>Total Transfers<span>
                <button name="TimeAscendant"    className="sorter" onClick={handleSorterChange}>▲</button></span></th>
              <th>Departure City</th>
              <th>Arrival City</th>
              <th>Departure Time</th>
              <th>Arrival Time</th>
              <th>Train Type</th>
              <th>Days of Operation</th>
            </tr>
          </thead>            
            {!loader && tableElement}
          </table>
            {loader && <Spinner/>}
            {(hasNoMatch && !loader) && <div>No matches found</div>}
      </div>
  );
};

export default DisplayTile;
