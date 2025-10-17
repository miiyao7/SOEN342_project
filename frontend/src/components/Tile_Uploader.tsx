import React, { useState } from 'react';
interface UploadTileProps {
  onParsed: (data: any) => void; // Callback prop type
  onLoading: (data: any) => void; // Callback prop type
}

const UploadTile: React.FC<UploadTileProps> = ({ onParsed, onLoading }) => {
    const [routes, setRoutes] = useState(null);
    const showSpinner = () => {
      onLoading(true);         // Show the element
      setTimeout(() => {
        onLoading(false);      // Hide the element after 3 seconds
      }, 3000);
    };  
    
    const handleClick = async (e: React.MouseEvent<HTMLButtonElement>) => {    
        showSpinner();
         try {
            const response = await fetch("http://localhost:3001/handler/get", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
          },
            body: JSON.stringify({}), 
          });

          const json = await response.json();
          setRoutes(json);
          onParsed(json);
        } catch (err) {
            console.error(err);
            // You could set error state here to inform user
        } 
    };
    return (    
      <div className="form-tile sheet">
        <h2>Upload Spread Sheet</h2>
          <div className="form-group">
            <button type="button" id="file-input" onClick={handleClick}/>
            <label id="file-input-label" htmlFor="file-input">Load Railway Data</label>
          </div>
      </div>    
    )
}
export default UploadTile;