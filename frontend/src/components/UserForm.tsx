import React, { useState, useEffect } from 'react';
import FilterTile from './Tile_Filters';
import DisplayTile from './Tile_Display';
import Spinner from './Spinner';

const UserForm: React.FC = () => {
    const [parsedData, setParsedData] = useState<any|null>(null);
    const [validCities, setValidCities] = useState<any|null>(null);
    const [validTrains, setValidTrains] = useState<any|null>(null);
    const [filters, setFilters] = useState<any|null>(null);
    const [sorter, setSorter] = useState<any|null>(null);
    const [loading, setLoading] = useState(false);
    const showSpinner = (time: number) => {
      setLoading(true);     
      setTimeout(() => {
        setLoading(false);   
      }, time);
    };  
    
    useEffect(() => {
      const fetchData = async () => {
          setLoading(true);
          try {
            const response = await fetch("http://localhost:3001/handler/get", {
                method: "GET"
            });
            if (!response.ok) {
                throw new Error(`Get failed: ${response.statusText}`);
            }
            const json = await response.json();
            setParsedData(json);
            showSpinner(3000);
          } catch (err) {
              console.error(err);
          } 
          try {
            const response = await fetch("http://localhost:3001/handler/getTrains", {
                method: "GET"
            });
            if (!response.ok) {
                throw new Error(`Get failed: ${response.statusText}`);
            }
            const vt = await response.json();
            setValidTrains(vt);
            //console.log("DEBUG {PARENT} validTrains", vt);
          } catch (err) {
              console.error(err);
          } 
          try {
            const response = await fetch("http://localhost:3001/handler/getCities", {
                method: "GET"
            });
            if (!response.ok) {
                throw new Error(`Get failed: ${response.statusText}`);
            }
            const vc = await response.json();
            setValidCities(vc);
            //console.log("DEBUG {PARENT} validCities", vc);
          } catch (err) {
              console.error(err);
          } 
      };
      fetchData();
    }, []);
    
    const handleFilteredData = (data: any) => {
      setFilters(data);
      showSpinner(5000);
      console.log("DEBUG {PARENT} filters", data);
    };
    
    const showWithLoader = loading && filters;
    const showFilter = parsedData && !loading;
    const showSortAndDisplay = showFilter && filters;
  //<UploadTile onParsed={handleParsedData} onLoading={handleLoading} />
 //{showSortAndDisplay && <SorterTile onSorted={handleSortedData}/>}
  return (
    <div className="form-container">
      {/* Conditionally show other tiles if parsedData exists */}
      {(showFilter || showWithLoader) && <FilterTile onFiltered={handleFilteredData} validTrains={validTrains} validCities={validCities}/>}
      {showSortAndDisplay && <DisplayTile filterList={filters} loading={loading} />}
  

      {loading && <Spinner/>}
    </div>
  );
};

export default UserForm;
