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
      




  const getDifferenceTime = (endTime: any, restartTime: any) => {
    // Parse "HH:MM:SS" into seconds
    const toSeconds = (t: string) => {
      const [h, m, s] = t.split(":").map(Number);
      return h * 3600 + m * 60;
    };

    const seconds1 = toSeconds(endTime);
    const seconds2 = toSeconds(restartTime);
    let diff = seconds2 - seconds1;
    if(diff < 0) diff += 24 * 3600;  // handle overnight difference

    // Convert diff back to HH:MM:SS
    const h = Math.floor(diff / 3600);
    const m = Math.floor((diff % 3600) / 60);
    const s = diff % 60;
    
    return `${String(h).padStart(2, "0")}h${String(m).padStart(2, "0")}m`;

  };


  const getFormattedText = (text: string) => {
    const formattedText = text.split("\n").map((line, index) => (
      <React.Fragment key={index}>
        {line} <br />
      </React.Fragment>
    ));
    return formattedText;
  };


  const getFormattedWeek = (arr: any) => {
    let trimmedArr = arr.map((str: any) => str.slice(0, 3));
    if(arr.length == 7) return "All Week";
    return trimmedArr.join(", ");
  };
  
  const getFormattedTransfers = (t: string, routes: any) => {
      let depStart = routes[0].departure_city;
      let arrCity1 = routes[0].arrival_city;
      let endTime = routes[0].arrival_time;
      let startTime = routes[0].departure_time;
      let t1 = getDifferenceTime(startTime, endTime);
      let result = depStart + "-("+ t1 +")->" + arrCity1;
    if(parseInt(t) >= 1){
      let depCity2    = routes[1].departure_city;
      let arrCity2    = routes[1].arrival_city;
      let restartTime = routes[1].departure_time;
      let finalTime   = routes[1].arrival_time;
      let t2          = getDifferenceTime(restartTime, finalTime);
      let transfer    = getDifferenceTime(endTime, restartTime);
      result += "\n Wait (" + transfer + ")\n" + depCity2 + "-("+ t2+")->" + arrCity2;
    }
    if(parseInt(t) == 2){      
      let arrCity2     = routes[1].arrival_city;
      let depCity3     = routes[2].departure_city;
      let arrCity3     = routes[2].arrival_city;
      let depCity2Time = routes[1].departure_time;
      let arrCity2Time = routes[1].arrival_time;
      let depCity3Time = routes[2].departure_time;
      let t3           = getDifferenceTime(depCity2Time, arrCity2Time);
      let transfer     = getDifferenceTime(arrCity2Time, depCity3Time);
      result += "\n Wait (" + transfer + ")\n" + depCity3 + "-("+ t3+")->" + arrCity3;
    }
    if(parseInt(t) == 0) return "None";
    return <div>{getFormattedText(result)}</div>;
  };
  const renderTable = () => {
    if(data == null || data[0] == null){
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
                    <td rowSpan={item.routes.length}>{getFormattedTransfers(item.total_transfers, item.routes)}</td>
                  </>
                ) : null}
                
                <td>{route.departure_city}</td>
                <td>{route.arrival_city}</td>
                <td>{route.departure_time}</td>
                <td>{route.arrival_time}</td>
                <td>{route.train_type}</td>
                <td>{getFormattedWeek(route.days_of_operation)}</td>
              </tr>
            ))
          ) : (
            // if no routes, show total info with empty route columns
            <tr key={idx}>
              <td>{item.total_duration}</td>      
              <td>{item.total_price_first}</td> 
              <td>{item.total_price_second}</td>
              <td>None</td>
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
              <th>1st-Class Price(€)<span>
                <button name="PriceAscendant1"  className="sorter" onClick={handleSorterChange}>▲</button>
                <button name="PriceDescendant1" className="sorter" onClick={handleSorterChange}>▼</button></span></th>
              <th>2nd-Class Price(€)<span>
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
