import { match } from 'assert';
import React, { useState, useEffect } from 'react';

import Spinner from './Spinner';
import Ticket from './Ticket';

type TicketInfo = {
    id: string,
    name: string,
    age: number | string,
    date: any,
    route: any
}

type TravelerMap = {
  id: string;
  name: string;
  age: number | string;
};

interface DisplayTileProps {
  ticketsInfo: any;
  loading: boolean;
}

const API = "http://127.0.0.1:3001";
const DisplayBookingTickets: React.FC<DisplayTileProps> = ({ ticketsInfo, loading }) => {
  
  const [loader, setLoader] = useState<boolean>(loading);
  const [hasNoMatch, setHasNoMatch] = useState<boolean>(false);

  useEffect(() => {
  }, []);
      

  const showSpinner = (time: number) => {
      setLoader(true);     
      setTimeout(() => {
        setLoader(false);   
      }, time);
    };  

  const renderTickets = () => {
    if(ticketsInfo == null || ticketsInfo[0] == null){
      setHasNoMatch(true);
      return (<div></div>);
    } 
    //console.log((ticketsInfo));
    return (
        <div className="ticketContainer">
          {ticketsInfo.map((t: any) => 
              <Ticket key={t.date} route={t.route} id={t.id} date={t.date} value={{rate1:t.route.first_class_ticket_rate, rate2: t.route.second_class_ticket_rate, first_name: t.first_name, last_name: t.last_name}}/>
            )
          }
        </div>
    )
  }  
    const ticketListElement = React.useMemo(() => {
      if (!ticketsInfo) return null;
      if (ticketsInfo.length === 0) return null;
      return renderTickets();
    }, [ticketsInfo]);

   const showFilter = !hasNoMatch && !loading;
  return (
      <div className="ticket-display">
        {showFilter && ticketListElement}
      </div>
  );
};

export default DisplayBookingTickets;
