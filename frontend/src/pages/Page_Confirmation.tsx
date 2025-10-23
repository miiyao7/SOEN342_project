
import { useState } from 'react';
import Ticket from '../components/Ticket';

type TravelerInfo = {
  name: string;
  age: number | string;
};
type TravelerMap = {
  id: string;
  first_name: string;
  last_name: string;
  age: number | string;
};

type BookingInfo = {
    id: string | number,
    name: string,
    date: Date,
    travelers: TravelerMap[],
    route: any
}
type Booking = {
    travelers: TravelerMap[], trip_date: Date, route_id: number
}

interface props {
  notif: any;
}
const API = "http://127.0.0.1:3001";
const Page_Confirm: React.FC<props> = ({notif}) => {   

    const [travelerData] =  useState(() => {
        const stored = localStorage.getItem('bookingData');
        return stored ? JSON.parse(stored) : {}; 
    });
    
    const [thisBooking] = useState<BookingInfo>({ id: travelerData.id,  name: travelerData.last_name, date: travelerData.date, travelers: travelerData.travelers, route: travelerData.route});
    const print = () =>{        
        saveBooking();
        /*            After Uploading Booking to Database        */
        discard();
    }
    const discard = () =>{       
        localStorage.removeItem("bookingData");
        localStorage.removeItem("selectedRoute");
        window.open("/", '_blank');
    }

    const saveBooking = () => {
        const ac = new AbortController();
        const isAbort = (e: unknown) => e instanceof DOMException && e.name === "AbortError";

        const newBooking: Booking = {
            travelers: thisBooking.travelers, 
            trip_date: thisBooking.date, 
            route_id: thisBooking.route.idx
        }
    
        const sendJSON = async (url: string) =>  {
          const resp = await fetch(url, {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json"
            },
            credentials: "omit",
            signal: ac.signal,
            body: JSON.stringify(newBooking)
          });
          if (!resp.ok) {
            throw new Error(`${url} failed: ${resp.status} ${resp.statusText}`);
          }
          return resp.json();
        };
        sendJSON(`${API}/handler/bookTrip`);
        return () => ac.abort();
    }

    return ( 
        <form>
            <div className='ticket-container'>
            {Object.entries(thisBooking.travelers).map(([key, value]) => {
               return( 
                <div><Ticket id={key} value={value} route={thisBooking.route} date={thisBooking.date}/></div>
               );
            })}
            </div>
          <button type="button" className="filter-submit" onClick={print}>PRINT</button>
          <button type="button" className="filter-submit" onClick={discard}>DISCARD</button>
        </form>
    )
}
export default Page_Confirm;