import React, { useState } from 'react';

const UserForm: React.FC = () => {
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [message, setMessage] = useState('');
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    try {
      //const response = await apiClient.post('/users', { username, email });
      //setMessage(`User created with ID: ${response.data.id}`);
      setUsername('');
      setEmail('');
    } catch (error) {
      setMessage('Error creating user');
      console.error(error);
    }
  };
  
  return (
    <div className="form-component">
      <div className="form-tile sheet">
        <h2>Upload Spread Sheet</h2>
        {message && <div className="alert">{message}</div>}
        
        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <input
              type="file"
              id="sheet"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              required
            />
          </div>
          <input type="submit" value="UPLOAD"/>
        </form>
      </div>
      <div className="form-tile filters">     
        <h2>Filters</h2>             
        <form onSubmit={handleSubmit}>
          <div className="form-group">  
            <div className="cardContainer">
              <div className="card"><label htmlFor="username">Departure City</label>
                <select id="countryDeparture" name="countryDeparture">
                  <option value="au">Australia</option>
                  <option value="ca">Canada</option>
                  <option value="usa">USA</option>
                </select></div>
              <div className="card"><label htmlFor="username">Arrival City</label>
                <select id="countryArrival" name="countryArrival">
                  <option value="au">Australia</option>
                  <option value="ca">Canada</option>
                  <option value="usa">USA</option>
                </select></div>
              <div className="card"><label htmlFor="username">Departure Time</label>
                <select id="DepartureTime" name="DepartureTime">
                  <option value="au">1:00</option>
                  <option value="ca">5:40</option>
                  <option value="usa">8:20</option>
                </select></div>
              <div className="card"><label htmlFor="username">Arrival Time</label>
                <select id="ArrivalTime" name="ArrivalTime">
                  <option value="au">1:00</option>
                  <option value="ca">5:40</option>
                  <option value="usa">8:20</option>
                </select></div>
              <div className="card"><label htmlFor="username">Train Type</label>
                <select id="TrainType" name="TrainType">
                  <option value="au">1:00</option>
                  <option value="ca">5:40</option>
                  <option value="usa">8:20</option>
                </select></div>
              <div className="card"><label htmlFor="username">Days of Operation</label>
                <select id="OperationDays" name="OperationDays">
                  <option value="au">10</option>
                  <option value="ca">540</option>
                  <option value="usa">82</option>
                </select></div>
              <div className="card">
                <label htmlFor="username">Ticket Rates (₤): 1st Class</label>
                <select id="1stRates" name="1stRates">
                  <option value="au">10</option>
                  <option value="ca">540</option>
                  <option value="usa">82</option>
                </select>
              </div>
              <div className="card">
                <label htmlFor="username">Ticket Rates (₤): 2nd Class</label>
                <select id="2ndRates" name="2ndRates">
                  <option value="au">10</option>
                  <option value="ca">540</option>
                  <option value="usa">82</option>
                </select>
              </div>
            </div>
          </div>
        </form>
      </div>
    </div>
  );
};

export default UserForm;
