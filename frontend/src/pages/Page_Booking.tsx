import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';

import Spinner from '../components/Spinner';
import BookingTile from '../components/Tile_Booking';


interface props {
  onNotif: any;
}
const Page_Booking: React.FC<props> = ({onNotif}) => {
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
      <BookingTile route={currRoute} onNotif={onNotif}/>
    </div>
  );
};

export default Page_Booking;
