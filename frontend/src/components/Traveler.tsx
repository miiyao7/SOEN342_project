
import React, { useEffect, useState, useRef } from 'react';
type TravelerProps = {
  tempKey?: string;
  travelerId?: string;
  travelerName?: string;
  travelerAge?: string | number;
  confirmed: boolean;
  onConfirm: (tempKey: string, id: string, info: TravelerInfo) => void;
  onRemove: () => void;
};
type TravelerInfo = {
  name: string;
  age: number | string;
};
const Traveler: React.FC<TravelerProps> = ({ tempKey="", travelerId = "", travelerName= "", travelerAge= 18, confirmed, onConfirm, onRemove }) => {
    const [currentId, setCurrentId] = useState(travelerId);
    const [currentName, setCurrentName] = useState(travelerName);
    const [currentAge, setCurrentAge] = useState(travelerAge);
    useEffect(() => {
        setCurrentId(travelerId);
        setCurrentName(travelerName||"");
        setCurrentAge(travelerAge||18);
    }, []);

    const handleDone = () => {  
        if (!currentId.trim()) {
        alert("ID is required");
        return;
        }
        if (!currentName.trim()) {
        alert("Name is required");
        return;
        }
        onConfirm(tempKey, currentId.trim(), { name: currentName.trim(), age: currentAge });
    };

    return (
        <>            
            <div className="card">
                <label htmlFor="ID">ID Number</label>
                <input type="text" name="ID" className={confirmed ? "addedTrav" : ""} value={currentId} onChange={(e) => setCurrentId(e.target.value)}></input>
            </div>
            <div className="card">
                <label htmlFor="Name" >Full Name</label>
                <input type="text" name="Name" className={confirmed ? "addedTrav" : ""} value={currentName} onChange={(e) => setCurrentName(e.target.value)} ></input>
            </div>
            <div className="card">
                <label htmlFor="Age" >Age</label>
                <input type="number" name="Age" className={confirmed ? "addedTrav" : ""} value={currentAge} onChange={(e) => setCurrentAge(e.target.value)} ></input>
            </div>
            <div className="card">    
                <label id="dud"></label>            
                <button type="button" className="btn-confirm" onClick={handleDone}>CONFIRM</button>
                <button type="button" className="btn-remove" onClick={onRemove}>REMOVE</button>
            </div>
        </>
    )
}
export default Traveler;