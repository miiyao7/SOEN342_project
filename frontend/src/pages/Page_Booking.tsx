import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';

import Spinner from '../components/Spinner';
import BookingTile from '../components/Tile_BookingForm';


const Page_Booking: React.FC = () => {
    const [currRoute, setCurrRoute] = useState(() => {
      const stored = localStorage.getItem('selectedRoute');
      return stored ? JSON.parse(stored) : {};
    });

    useEffect(() => {
      // Check if state and booking_data exist
      if (localStorage.getItem("selectedRoute")) {
        const booking_data = localStorage.getItem("selectedRoute");
        setCurrRoute(booking_data);
      }
    }, [localStorage.getItem("selectedRoute")]);


    
  return (
    <div className="booking-container">
      <BookingTile route={currRoute}/>
    </div>
  );
};

export default Page_Booking;
