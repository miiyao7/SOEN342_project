

interface TicketProps{
    id?: any
    route?: any;
    value?: any;
    date?: any
}
const Ticket: React.FC<TicketProps> = ({ id, value, route, date }) => {
    function randomIntBetween(min: number, max: number): number {
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }
    function getRandomLetter(): string {
    const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const randomIndex = Math.floor(Math.random() * alphabet.length);
    return alphabet.charAt(randomIndex);
    }
    //alert(JSON.stringify(value));
    const seatNo = randomIntBetween(1, 1000);
    const carNo = getRandomLetter();
    return(
        <div className="ticket">
            <div className="ticket-header">
                <div className="card"><h3>TRAIN TICKET</h3></div>
                <span className="cardrow">
                    <div className="card"><label>DATE</label><label className="ticketdata ticketDate">{date}</label></div>
                    <div className="card"><label>DEPART</label><label className="ticketdata ticketFromTime">{route.departure_time}</label></div>
                    <div className="card"><label>ARRIVE</label><label className="ticketdata ticketToTime">{route.arrival_time}</label></div>
                </span>
            </div>
            <div className="ticket-body">
                <div className="card"><label>NAME OF PASSENGER</label><label className="ticketdata ticketPassenger">{value.first_name + " " + value.last_name}</label></div>
                <div className="card"><label>PRICE:</label><label className="ticketdata ticketPrice">{route.price} -- €</label></div>
                <div className="card"><label>FROM:</label><label className="ticketdata ticketFrom">{route.departure_city}</label></div> 
                <div className="card"><label>TRAIN:</label><label className="ticketdata ticketTrain">{route.train_type}</label></div>
                <div className="card"><label>TO:</label><label className="ticketdata ticketTo">{route.arrival_city}</label></div>
                <div className="card"><label>Seat.....[{seatNo}]</label><label>Carr.....[{carNo}]</label></div>
            </div>
            <div className="ticket-footer">
                <span className="cardRow"><label className="ticketPassengerID">{route.train_type} {id}-{seatNo}-{carNo}</label>
                <label className="ticketTicketType">{route.price_range}</label></span>
            </div>
        </div>
    );

};

export default Ticket;