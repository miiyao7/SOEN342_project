import React, { useState, useEffect } from 'react';
import UploadTile from './Tile_Uploader';
import FilterTile from './Tile_Filters';
import SorterTile from './Tile_Sorters';
import DisplayTile from './Tile_Display';
import Spinner from './Spinner';

const UserForm: React.FC = () => {
    const [parsedData, setParsedData] = useState<any|null>(null);
    const [filters, setFilters] = useState<any|null>(null);
    const [sorter, setSorter] = useState<any|null>(null);
    const [loading, setLoading] = useState(false);
    // Callback to receive parsing result from UploadTile
    const handleParsedData = (data: any) => {
      setParsedData(data);
      console.log("DEBUG {PARENT} all", data);
    };
    const handleLoading = (data: any) => {
      setLoading(data);
    };
    const handleFilteredData = (data: any) => {
      setFilters(data);
      console.log("DEBUG {PARENT} filters", data);
    };
    const handleSortedData = (data: any) => {
      setSorter(data);
      console.log("DEBUG {PARENT} sorters", data);
    };
  const showFilter = parsedData && !loading;
  const showSortAndDisplay = showFilter && filters;
  return (
    <div className="form-container">
      <UploadTile onParsed={handleParsedData} onLoading={handleLoading} />
      {/* Conditionally show other tiles if parsedData exists */}
      {showFilter && <FilterTile onFiltered={handleFilteredData} handler={handleFilteredData} />}
      {showSortAndDisplay && <SorterTile onSorted={handleSortedData} handler={handleSortedData} />}
      {showSortAndDisplay && <DisplayTile filterList={filters} sortTag={sorter} loading={loading} />}
  

      {loading && <Spinner/>}
    </div>
  );
};

export default UserForm;
