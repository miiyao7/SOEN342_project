import { match } from 'assert';
import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';

import Spinner from './Spinner';
interface DisplayTileProps {
  filterList: any; // Callback prop type
  loading: boolean;
}

const API = "http://127.0.0.1:3001";
const DisplayTile: React.FC<DisplayTileProps> = ({ filterList, loading }) => {
  
  const [sort, setSort] = useState<string>("");
  const [sortChange, setSortChange] = useState<boolean>(false);
  const [data, setData] = useState<any>(null);
  const [loader, setLoader] = useState<boolean>(loading);
  const [hasNoMatch, setHasNoMatch] = useState<boolean>(false);
  const Sorter = { sort_by: sort }
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
  
  useEffect(() => {
    
    const ac = new AbortController();
    const isAbort = (e: unknown) => e instanceof DOMException && e.name === "AbortError";
    console.log("DEBUG {DISPLAY} filters", Filters);
    const fetchJSON = async (url: string) =>  {
      const resp = await fetch(url, {
        method: "POST",
        headers: {
          "Accept": "application/json",
          "Content-Type": "application/json"
        },
        credentials: "omit",
        signal: ac.signal,
        body: JSON.stringify({filters: Filters, sorter: Sorter})
      });
      if (!resp.ok) {
        throw new Error(`${url} failed: ${resp.status} ${resp.statusText}`);
      }
      return resp.json();
    };
    const fetchData = async () => {
      setLoader(true);
      try {
        const filtered = await Promise.all([fetchJSON(`${API}/handler/search`)]);
          if(sortChange) {
            setLoader(false);   
          } else {showSpinner(3000);}
          if (ac.signal.aborted) return;  
          setData(filtered[0]);
          console.log("DEBUG {DISPLAY} filtered", filtered[0]);
        } catch (e) {
          if (!isAbort(e)) console.error(e);    // ignore AbortError
        } finally {
          if (!ac.signal.aborted) setLoader(false);
        }
    };    

    fetchData();
    return () => ac.abort();
  }, [filterList, sort]);
      
  const handleBooking = (route: any) => {  
    localStorage.setItem("selectedRoute", JSON.stringify(route));
    window.open("/booking-page", '_blank');
  }

  const showSpinner = (time: number) => {
      setLoader(true);     
      setTimeout(() => {
        setLoader(false);   
      }, time);
    };  
  const handleSorterChange = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    setSortChange(true);  

    const button: HTMLButtonElement = event.currentTarget;
    let sorterID = button.name;    
    setSort(sorterID);
    console.log("Sorter: " + sorterID);
  };


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
    if(parseInt(t) == 0) return "None";
      const arrowT = "—";
      const arrowH = "—▷";

      const depStart  = <span style={{backgroundColor: '#4eaf76a4', borderRadius: '8px', padding: '0 8px'}}>{routes[0].departure_city}</span>;
      const arrCity1  = routes[0].arrival_city;
      const endTime   = routes[0].arrival_time;
      const startTime = routes[0].departure_time;
      const t1        = <span style={{color: '#181d1aa4', padding: '0 8px'}}>{arrowT+ getDifferenceTime(startTime, endTime) + arrowH}</span>;

      let result = [depStart, t1, arrCity1];
    if(parseInt(t) >= 1){
      const depCity2    = routes[1].departure_city;
      let arrCity2      = <span style={{backgroundColor: '#dfaf56be', borderRadius: '8px', padding: '0 8px'}}>{routes[1].arrival_city}</span>;
      const restartTime = routes[1].departure_time;
      const finalTime   = routes[1].arrival_time;
      const t2          = <span style={{color: '#181d1aa4', padding: '0 8px'}}>{arrowT+ getDifferenceTime(restartTime, finalTime) + arrowH}</span>;
      const transfer    = getDifferenceTime(endTime, restartTime);
      if(parseInt(t) == 2){ arrCity2  = routes[1].arrival_city;}
      result.push(<br/>,"Wait (" + transfer + ")",<br/>, depCity2, t2, arrCity2);
    }
    if(parseInt(t) == 2){      
      const depCity3     = routes[2].departure_city;
      const arrCity3     = <span style={{backgroundColor: '#dfaf56be', borderRadius: '8px', padding: '0 8px'}}>{routes[2].arrival_city}</span>;
      const depCity2Time = routes[1].departure_time;
      const arrCity2Time = routes[1].arrival_time;
      const depCity3Time = routes[2].departure_time;
      const t3           = <span style={{color: '#181d1aa4', padding: '0 8px'}}>{arrowT+ getDifferenceTime(depCity2Time, arrCity2Time) + arrowH}</span>;
      const transfer     = getDifferenceTime(arrCity2Time, depCity3Time);
      result.push(<br/>,"Wait (" + transfer + ")", <br/>, depCity3 , t3, arrCity3);
    }
    return <div style={{whiteSpace: "pre-line"}}>{result}</div>;
  };
  const renderTable = () => {
    if(data == null || data[0] == null){
      setHasNoMatch(true);
      return (<tr></tr>);
    } 
    /*let outerheaders = Object.keys(data);
    let innerheaders = Object.keys(data[0].routes[0]);
    let headers = outerheaders.concat(innerheaders);
    console.log("DEBUG PATH", data[0].routes);
    console.log(innerheaders);
    console.log("Keys ", outerheaders);*/
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
                
                <td onClick={() => handleBooking(route)}>{route.departure_city}</td>
                <td onClick={() => handleBooking(route)}>{route.arrival_city}</td>
                <td onClick={() => handleBooking(route)}>{route.departure_time}</td>
                <td onClick={() => handleBooking(route)}>{route.arrival_time}</td>
                <td onClick={() => handleBooking(route)}>{route.train_type}</td>
                <td onClick={() => handleBooking(route)}>{getFormattedWeek(route.days_of_operation)}</td>
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
                <button name="TimeAscendant"    className="sorter" onClick={handleSorterChange}>▼</button></span></th>
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
