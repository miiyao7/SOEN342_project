import React, { useState, useEffect } from 'react';
import DatePicker from 'react-datepicker';
import TicketsDisplay from '../components/Tile_TicketDisplay';
import "react-datepicker/dist/react-datepicker.css";


type Ticket = {
    id: string,
    name: string,
    age: number | string,
    date: any,
    route: any
};
type TravelerMap = {
  id: string;
  name: string;
  age: number | string;
};


type BookingInfo = {
    id: string,
    name: string,
    date: any,
    travelers: TravelerMap[],
    route: any
}

type BookingFilter = {
    id: string,
    last_name: string,
    is_ongoing: boolean,
}


interface props {
  notif: any;
}

const API = "http://127.0.0.1:3001";
const Page_Ticket_Parser: React.FC<props> = ({notif}) => {
   
    const [filters, setFilterList] = useState<BookingFilter>({id: "",  last_name: "",  is_ongoing: true});
    const [loading, setLoading] = useState(true);
    const [tickets, setTickets] = useState<Ticket[]>([]);
    const [ticketsInfo, setTicketsInfo] = useState<BookingInfo[]>([]);
    const [trip, setTripInfo] = useState({});
    const [selectedDate, setSelectedDate] = useState<Date|null>(new Date(Date.now()));
    //const [thisBooking, setThisBooking] = useState<BookingInfo>({ id: "",  name: "", date: Date.now(), travelers: {}, route: {}});


    const showSpinner = (time: number) => {
        setLoading(true);     
        setTimeout(() => {
            setLoading(false);   
        }, time);
    }; 
    const filterBookings = (filter: any) => {       
        
        //console.log("Filters", filters);
        if(filter.id !== "" && filter.last_name !== "") {
            const ac = new AbortController();
            const isAbort = (e: unknown) => e instanceof DOMException && e.name === "AbortError";
            const sendJSON = async (url: string) =>  {
            const resp = await fetch(url, {
                method: "POST",
                headers: {
                    "Accept": "application/json",
                    "Content-Type": "application/json"
                },
                credentials: "omit",
                signal: ac.signal,
                body: JSON.stringify(filter)
            });
            if (!resp.ok) {
                throw new Error(`${url} failed: ${resp.status} ${resp.statusText}`);
            }
            return resp.json();
            };
            const fetchData = async () => {
                setLoading(true);
                const [newTicketsInfo]: BookingInfo[] = [];
                try {
                    let trips = await Promise.all([sendJSON(`${API}/handler/filterBookings`)]);
                    if (ac.signal.aborted) return;  
                    console.log("Response", trips[0]);
                    if (trips[0].length == 0){
                        notif({type: "i", text:"No Bookings Found "}); 
                        setTickets([]);
                        console.log("No Trip");
                        return;
                    }
                    const newTickets = trips[0].flatMap((trip: any) => {
                        return trip.tickets.map((person: any)=> ({
                            id: person.traveler.id,
                            first_name: person.traveler.first_name ,
                            last_name: person.traveler.last_name,
                            age: person.traveler.age,
                            date: trip.date,
                            route: trip.route
                        })
                        );
                    });
                    setTickets(newTickets);
                    console.log("No Trip", newTickets);
                       
                    setLoading(false);
                    //notif("Return " + JSON.stringify(newTickets));
                    //alert(JSON.stringify(trips[0].flatMap((trip: any) => {return trip.tickets})));
                } catch (e) {
                    if (!isAbort(e)) console.error(e); 
                } finally {
                    if (!ac.signal.aborted) setLoading(false);
                }
            }
            fetchData();
            return () => ac.abort();
        } 
        notif({type: "x", text:"All filters are required"});
    };  

    const showFilter = tickets && !loading && filters.id && filters.last_name;
    return ( 
        <div className="form-container">
            <div className="form-tile booking-filters">
                <form>
                <div className="form-group">
                    <div className="cardContainer bookings">
                    
                        <div className="card">
                            <label htmlFor="DesiredDate" >Booking Collections</label>
                            <select 
                            value={filters.is_ongoing ? "true" : "false"}
                            onChange={e => setFilterList(prev => ({...prev, is_ongoing: e.target.value === "true" }))}>
                                <option value="false">Past Trips</option>
                                <option value="true">Ongoing Trips</option>
                            </select>
                            </div>
                        <div className="card">
                        <label htmlFor="ID">ID Number</label>
                        <input type="text" name="ID" onChange={e => setFilterList(prev => ({...prev, id: e.target.value}))}></input>
                        </div>
                        <div className="card">
                            <label htmlFor="Name" >Last Name</label>
                            <input type="text" name="Name" onChange={e => setFilterList(prev => ({...prev, last_name: e.target.value}))} ></input>
                        </div>
                    </div>
                </div>          
                <button type="button" className="filter-submit" onClick={() => filterBookings(filters)}>FILTER</button>
                </form>          
            </div>                      
            <form>{showFilter && <TicketsDisplay ticketsInfo={tickets} loading={loading}/>}</form>      
        </div>
    )
}
export default Page_Ticket_Parser;