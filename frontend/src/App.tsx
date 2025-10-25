import { Routes, Route } from 'react-router-dom';
import { useEffect, useState } from 'react';
import ParsingPage from './pages/Page_Parser';
import BookingPage from './pages/Page_Booking';
import TicketPage from './pages/Page_Ticket_Parser';
import BookingConfirmPage from './pages/Page_Confirmation';
import './css/App.css';

type Notif = {
  text: string,
  type: string,
}
const App: React.FC = () => {
  const [notif, setNotif] = useState<Notif>({text: "", type: ""});
  const [isUpdated, setIsUpdated] = useState(false);

  const handleNotif = (n: any) => {
    setNotif(n);
    toggler(8000, setIsUpdated);
  };
  const toggler = (time: number, fn: (value: boolean) => void) => {
    fn(true);
    setTimeout(() => fn(false), time);
  };  
  return (
    <div className="app">
      <header className="app-header">
        <h1>Rail Network Bookings</h1>
      </header>
      
      <main>
        <Routes>
          <Route path="/" element={<ParsingPage notif={handleNotif} />} />
          <Route path="/booking-page" element={<BookingPage onNotif={handleNotif} />} />
          <Route path="/bookings-parser-page" element={<TicketPage notif={handleNotif} />} />
          <Route path="/booking-confirm-page" element={<BookingConfirmPage notif={handleNotif} />} />
        </Routes>
      </main>
      <div className={isUpdated ? `notificationBox type-${notif.type}` : 'notificationBox hidden'}>
        <div className='badge'><p>{notif.type}</p></div>
        <div className='content'>{notif.text}</div>
      </div>
    </div>
  );
}

export default App;
