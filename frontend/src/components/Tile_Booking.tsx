
import { useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';

import FilteredDatePicker from './FilteredDatePicker';
import TravelerData from './Traveler';

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

type Booking = {
    id: string | number,
    name: string,
    date: any,
    travelers: TravelerMap[],
    route: any
}

interface BookingProp{
    route: any;
    onNotif: any;
}

const BookingTile: React.FC<BookingProp> = ( { route, onNotif } ) => {
    const [thisRoute, setThisRoute] = useState(route);
    const [thisDate, setDate] = useState<any>();
    const [thisBooking, setThisBooking] = useState<Booking>({ id: "",  name: "", date: Date.now(), travelers: [], route: {}});
    //alert("Booking " + route);
        
    // Array of temporary IDs for forms not confirmed yet
    const [tempFormKeys, setTempFormKeys] = useState<string[]>([]);
    const [ travelers, setTravelers ] = useState<TravelerMap[]>([]);
    const addNewTraveler = () => {
        const tempKey = Date.now().toString();
        setTempFormKeys((prev) => [...prev, tempKey]);
    };
    const setBooking = () => {
        //let booking = { travelers: travelers, route: thisRoute, date: thisDate.toISOString().split('T')[0]}; 
        if(thisDate === null || !(thisDate instanceof Date)) { onNotif({type: "x",text:"No date set."}); return; }
        if(travelers.length == 0 || travelers === null) { onNotif({type: "x", text:"No travelers confirmed."}); return; }
        thisBooking.id = travelers[0].id;
        thisBooking.name = travelers[0].last_name;
        thisBooking.date = thisDate.toISOString().split('T')[0];
        thisBooking.travelers = travelers;
        thisBooking.route = thisRoute;
        //alert("Booking: " + JSON.stringify(thisBooking));
        localStorage.setItem("bookingData", JSON.stringify(thisBooking));
        onNotif({type: "i", text: "Generating tickets..."});
        window.open("/booking-confirm-page", '_blank');

    }
    const handleTravelerConfirm = (tempKey: string, realId: string, info: TravelerInfo  ) => {
        if (!realId.trim()) {
            onNotif({text: "Passenger ID is required."});
            return;
        }
        if (travelers.some(({id}) => (id === realId))){
            var text = "Already confirmed." + " All Travelers: " + JSON.stringify(travelers);
            onNotif({text: text});
            return;
        }

        const newTrav: TravelerMap = {
            id: realId,
            first_name: info.name.split(' ').slice(0, -1).join(' '),
            last_name: info.name.split(' ').slice(-1).join(' '),
            age: info.age,
        };

        // Remove the temp form key
        setTempFormKeys((prev) => prev.filter((key) => key !== tempKey));

        // Add or update the traveler array
        setTravelers((prev) => [...prev, newTrav]);
        //alert("New Traveler: " + JSON.stringify(newTrav));
    };
    const handleRemoveTraveler = (idToRemove: string) => {
       setTravelers((prev) => prev.filter(t => t.id !== idToRemove));
    };
    // Remove a temporary form by its tempKey - user cancelled input
    const handleRemoveTempForm = (tempKey: string) => {
        setTempFormKeys((prev) => prev.filter((key) => key !== tempKey));
    };

    // Remove a temporary form by its tempKey - user cancelled input
    const handleDate = (date: any) => {
        setDate(new Date(date));
    };
    return ( 
        <form>
          <div className="form-group">
              <div className="dateCard">
                <label htmlFor="desiredDate">Date</label>
                <FilteredDatePicker validDays={thisRoute.days_of_operation} onSelectedDate={handleDate}/>                
              </div>
            <div className="cardContainer">
                {travelers.map(({id, first_name, last_name, age}) => (
                    <TravelerData key={id} travelerId={id} travelerName={first_name + " " +last_name} travelerAge={age} onConfirm={handleTravelerConfirm} onRemove={() => handleRemoveTraveler(id)} confirmed={true}/>
                ))}
                {tempFormKeys.map((key) => (
                    <TravelerData
                    key={key}
                    tempKey={key}
                    onConfirm={handleTravelerConfirm}
                    onRemove={() => handleRemoveTempForm(key)}
                    confirmed={false}
                    />
                ))}
            </div>
          </div>
            <button type="button" className="btn-add" onClick={addNewTraveler}>ADD TRAVELER</button>
            <button type="button" className="btn-submit" onClick={setBooking}>BOOK</button>
        </form>
    )
}
export default BookingTile;