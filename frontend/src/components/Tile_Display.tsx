import { match } from 'assert';
import React, { useState, useEffect } from 'react';
interface DisplayTileProps {
  filterList: any; // Callback prop type
  sortTag: String; // Callback prop type
  loading: boolean;
}

const DisplayTile: React.FC<DisplayTileProps> = ({ filterList, sortTag, loading }) => {
  
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
  const Sorter = { sort_by: sortTag }

  const [data, setData] = useState<any>(null);


  useEffect(() => {
    const fetchData = async () => {
      try {
          const response = await fetch("http://localhost:3001/handler/search", {
              method: "POST",
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({filters: Filters, sorter: Sorter})
          });

          if (!response.ok) {
              throw new Error(`Get failed: ${response.statusText}`);
          }

          const filtered = await response.json();
          setData(filtered)
        } catch (err) {
            console.error(err);
        } 
    };
    fetchData();
  }, [filterList, sortTag]);
      
  const renderTable = () => {
    if(data[0] == null){
      return (<table><thead><tr><th>NO MATCHES FOUND</th></tr></thead></table>);
    } 
    let headers = Object.keys(data[0]);
    return (
    <table className="result-table">
      <thead>
        <tr>
          {headers.map((key) => (
            <th key={key}>{key.replaceAll('_', ' ').toUpperCase()}</th>
          ))}
        </tr>
      </thead>
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
  
      </table>
    )
  }
  const tableElement = React.useMemo(() => {
    if (!data) return null;
    return renderTable();
  }, [data]);
  return (
      <div className="form-tile display">
          {loading && <div className="mini-spinner"></div>}
          {tableElement}
      </div>
  );
};

export default DisplayTile;
