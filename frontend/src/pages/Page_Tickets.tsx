
import { useState } from 'react';
import DatePicker from 'react-datepicker';
import "react-datepicker/dist/react-datepicker.css";
const Page_Tickets: React.FC = () => {
   
       const [currentId, setCurrentId] = useState("");
       const [currentName, setCurrentName] = useState("");
       const [filter, setFilter] = useState("");

    const setFilters = ()=>{
        if(currentId){
            setFilter(currentId);
        } else if(currentName) {
            setFilter(currentName)
        }
    }

    const [selectedDate, setSelectedDate] = useState<Date | null>(null);
    return ( 
        <form>
          <div className="form-group">
            <div className="cardContainer bookings">
               
              <div className="card">
                <label htmlFor="DesiredDate" >Date</label>
                <DatePicker
                    selected={selectedDate}
                    onChange={date => setSelectedDate(date)}
                    dateFormat="yyyy-MM-dd"
                    required
                />                
                </div>
                <div className="card">
                <label htmlFor="ID">ID Number</label>
                <input type="text" name="ID" onChange={(e) => setCurrentId(e.target.value)}></input>
                </div>
                <div className="card">
                    <label htmlFor="Name" >Name</label>
                    <input type="text" name="Name" onChange={(e) => setCurrentName(e.target.value)} ></input>
                </div>
            </div>
          </div>
          
          <button type="button" className="filter-submit" onClick={setFilters}>FILTER</button>
        </form>
    )
}
export default Page_Tickets;