
import React, { useEffect, useState, useRef } from 'react';
type TravelerProps = {
  tempKey?: string;
  travelerId?: string;
  travelerInfo?: TravelerInfo;
  confirmed: boolean;
  onConfirm: (tempKey: string, id: string, info: TravelerInfo) => void;
  onRemove: () => void;
};
type TravelerInfo = {
  name: string;
  age: number | string;
};
type Traveler = {
  [id: string]: TravelerInfo;
};
const Traveler: React.FC<TravelerProps> = ({ tempKey="", travelerId = "", travelerInfo = {name: "", age: 18}, confirmed, onConfirm, onRemove }) => {
    const [currentId, setCurrentId] = useState(travelerId);
    const [currentName, setCurrentName] = useState(travelerInfo.name);
    const [currentAge, setCurrentAge] = useState(travelerInfo.age);
    useEffect(() => {
        setCurrentId(travelerId);
        setCurrentName(travelerInfo?.name||"");
        setCurrentAge(travelerInfo?.age||18);
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
                <label htmlFor="Name" >Name</label>
                <input type="text" name="Name" className={confirmed ? "addedTrav" : ""} value={currentName} onChange={(e) => setCurrentName(e.target.value)} ></input>
            </div>
            <div className="card">
                <label htmlFor="Age" >Age</label>
                <input type="number" name="Age" className={confirmed ? "addedTrav" : ""} value={currentAge} onChange={(e) => setCurrentAge(e.target.value)} ></input>
            </div>
            <div className="card">                
                <button type="button" className="filter-submit" onClick={handleDone}>CONFIRM</button>
                <button type="button" className="filter-submit" onClick={onRemove}>REMOVE</button>
            </div>
        </>
    )
}
export default Traveler;