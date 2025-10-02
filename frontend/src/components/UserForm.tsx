import React, { useState, useEffect } from 'react';
import UploadTile from './Tile_Uploader';
import FilterTile from './Tile_Filters';
import SorterTile from './Tile_Sorters';
import DisplayTile from './Tile_Display';
import Spinner from './Spinner';

const UserForm: React.FC = () => {
  /*const FetchSelect = ({ url, label }: { url: string; label: string }) => {
    const [options, setOptions] = useState<string[]>([]);

    useEffect(() => {
      fetch(url)
        .then((resp) => resp.json())
        .then(setOptions);
    }, [url]);
  }*/




    const [parsedData, setParsedData] = useState<any|null>(null);
    const [loading, setLoading] = useState(false);
    // Callback to receive parsing result from UploadTile
    const handleParsedData = (data: any) => {
      setParsedData(data);
    };
    const handleLoading = (data: any) => {
      setLoading(data);
    };
  
  return (
    <div className="form-container">
      <UploadTile onParsed={handleParsedData} onLoading={handleLoading} />
      {/* Conditionally show other tiles if parsedData exists */}
      {(parsedData && !loading) && <FilterTile data={parsedData} loading={parsedData} />}
      {(parsedData && !loading) && <SorterTile data={parsedData} loading={parsedData} />}
      {(parsedData && !loading) && <DisplayTile data={parsedData} loading={parsedData} />}
      {!parsedData && <p>Please upload and parse a CSV file.</p>}
      {loading && <Spinner/>}
    </div>
  );
};

export default UserForm;
