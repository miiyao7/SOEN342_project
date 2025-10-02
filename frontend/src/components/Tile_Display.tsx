import { match } from 'assert';
import React, { useState } from 'react';
interface DisplayTileProps {
  data: JSON; // Callback prop type
  loading: (data: any) => void;
}

const DisplayTile: React.FC<DisplayTileProps> = ({ data, loading }) => {
  const matches = [];  
  const matchesByTime = [];
  const matchesByPrice = [];
  const matchesByType = [];
  let found = true;
  console.log(data);
  return (
    <div className="form-component">        
      <div className="form-tile">
          <h2>Matching Routes...</h2>
          <div>          </div>
      </div>
    </div>
  );
};

export default DisplayTile;
