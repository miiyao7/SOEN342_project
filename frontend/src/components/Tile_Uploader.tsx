import React, { useState } from 'react';
interface UploadTileProps {
  onParsed: (data: any) => void; // Callback prop type
  onLoading: (data: any) => void; // Callback prop type
}

const UploadTile: React.FC<UploadTileProps> = ({ onParsed, onLoading }) => {
    const [routes, setRoutes] = useState(null);
    const [fileName, setFileName] = useState('Select a File');
    const showSpinner = () => {
      onLoading(true);         // Show the element
      setTimeout(() => {
        onLoading(false);      // Hide the element after 5 seconds
      }, 5000);
    };
    
    
    const onFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {    
        showSpinner();
        const file = e.target.files?.[0];
        if (!file) return;

        const formData = new FormData();
        formData.append('file', file);
        setFileName(file.name);
        try {
          const response = await fetch("http://localhost:3001/upload", {
              method: "POST",
              body: formData,
          });

          if (!response.ok) {
              throw new Error(`Upload failed: ${response.statusText}`);
          }

          const json = await response.json();
          setRoutes(json); // or update state as needed
          onParsed(json);   // inform parent with actual backend data
        } catch (err) {
            console.error(err);
            // You could set error state here to inform user
        } 
    };
    return (    
      <div className="form-tile sheet">
        <h2>Upload Spread Sheet</h2>
          <div className="form-group">
            <input type="file" accept=".csv" id="file-input" onChange={onFileChange}/>
            <label id="file-input-label" htmlFor="file-input">{fileName}</label>
          </div>
      </div>    
    )
}
export default UploadTile;