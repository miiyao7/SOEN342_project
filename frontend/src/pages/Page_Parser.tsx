import React, { useState, useEffect } from 'react';
import FilterTile from '../components/Tile_Filters';
import DisplayTile from '../components/Tile_Display';
import Spinner from '../components/Spinner';

const API = "http://127.0.0.1:3001";

const Page_Parser: React.FC = () => {
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
    const ac = new AbortController();
    const isAbort = (e: unknown) => e instanceof DOMException && e.name === "AbortError";

    const fetchJSON = async (url: string) =>  {
      const resp = await fetch(url, {
        method: "POST",
        headers: { Accept: "application/json" },
        credentials: "omit",
        signal: ac.signal,
      });
      if (!resp.ok) {
        throw new Error(`${url} failed: ${resp.status} ${resp.statusText}`);
      }
      return resp.json();
    };

    const fetchData = async () => {
     setLoading(true);
    try {
      const [vt, vc, routes] = await Promise.all([
        fetchJSON(`${API}/handler/getTrains`),
        fetchJSON(`${API}/handler/getCities`),
        fetchJSON(`${API}/handler/get`),
      ]);
      if (ac.signal.aborted) return;        // guard after awaits
      setValidTrains(vt);
      setValidCities(vc);
      setParsedData(routes);
      //console.log("DEBUG {PARENT} rawdata", routes);
    } catch (e) {
      if (!isAbort(e)) console.error(e);    // ignore AbortError
    } finally {
      if (!ac.signal.aborted) setLoading(false);
    }
  };

    fetchData();
    return () => ac.abort();
  }, []);
    
    const handleFilteredData = (data: any) => {
      setFilters(data);
      showSpinner(5000);
      //console.log("DEBUG {PARENT} filters", data);
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

export default Page_Parser;
