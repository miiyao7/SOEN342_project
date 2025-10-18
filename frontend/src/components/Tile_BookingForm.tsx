
import { useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';

import FilteredDatePicker from '../components/FilteredDatePicker';
import TravelerData from '../components/Traveler';

type TravelerInfo = {
  name: string;
  age: number | string;
};
type TravelerMap = {
  [id: string]: TravelerInfo;
};
interface BookingProp{
    route: any;
}

const BookingTile: React.FC<BookingProp> = ( { route } ) => {
    const [thisRoute, setThisRoute] = useState(route);
    const [date, setDate] = useState(route);
    //alert("Booking " + route);
        
    // Array of temporary IDs for forms not confirmed yet
    const [tempFormKeys, setTempFormKeys] = useState<string[]>([]);
    const [ travelers, setTravelers ] = useState<TravelerMap>({});
    const addNewTraveler = () => {
        const tempKey = Date.now().toString();
        setTempFormKeys((prev) => [...prev, tempKey]);
    };
    const setBooking = () => {
        let booking = { travelers: travelers, route: thisRoute, date: date.toISOString().split('T')[0]}; 
        localStorage.setItem("bookingData", JSON.stringify(booking));
        window.open("/booking-confirm-page", '_blank');

    }
    const handleTravelerConfirm = (tempKey: string, realId: string, info: TravelerInfo  ) => {
        if (!realId.trim()) {
            alert("Passenger ID is required.");
            return;
        }
        // Remove the temp form key
        setTempFormKeys((prev) => prev.filter((key) => key !== tempKey));

        // Add or update the confirmed traveler keyed by real ID
        setTravelers((prev) => ({
        ...prev,
        [realId]: info,
        }));
    };
    const handleRemoveTraveler = (idToRemove: string) => {
       setTravelers(prev => {
            const { [idToRemove]: _, ...rest } = prev;
            return rest;
        });
    };
    // Remove a temporary form by its tempKey - user cancelled input
    const handleRemoveTempForm = (tempKey: string) => {
        setTempFormKeys((prev) => prev.filter((key) => key !== tempKey));
    };

    // Remove a temporary form by its tempKey - user cancelled input
    const handleDate = (date: any) => {
        setDate(date);
    };
    return ( 
        <form>
          <div className="form-group">
              <div className="DateCard">
                <label htmlFor="DesiredDate">Date</label>
                <FilteredDatePicker validDays={thisRoute.days_of_operation} onSelectedDate={handleDate}/>                
              </div>
            <div className="cardContainer">
                {Object.entries(travelers).map(([id, info]) => (
                    <TravelerData key={id} travelerId={id} travelerInfo={info} onConfirm={handleTravelerConfirm} onRemove={() => handleRemoveTraveler(id)} confirmed={true}/>
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
            <button type="button" className="filter-submit" onClick={addNewTraveler}>ADD TRAVELER</button>
            <button type="button" className="filter-submit" onClick={setBooking}>BOOK</button>
        </form>
    )
}
export default BookingTile;