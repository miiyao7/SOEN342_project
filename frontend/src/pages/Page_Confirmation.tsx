
import { useState } from 'react';
import Ticket from '../components/Ticket';

const Page_Confirm: React.FC = () => {   

    const [travelerData] =  useState(() => {
        const stored = localStorage.getItem('bookingData');
        return stored ? JSON.parse(stored) : {}; 
    });
    const travelers = travelerData.travelers;
    alert(JSON.stringify(travelerData));
    const route = travelerData.route;
    const date = travelerData.date;
    const print = () =>{        
        localStorage.removeItem("selectedRoute");
        localStorage.removeItem("bookingData");
        window.open("/", '_blank');
    }

    return ( 
        <form>
            <div className='ticket-container'>
            {Object.entries(travelers).map(([key, value]) => {
               return( 
                <div><Ticket id={key} value={value} route={route} date={date}/></div>
               );
            })}
            </div>
          <button type="button" className="filter-submit" onClick={print}>PRINT</button>
        </form>
    )
}
export default Page_Confirm;