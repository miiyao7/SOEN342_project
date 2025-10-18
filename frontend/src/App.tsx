import { Routes, Route } from 'react-router-dom';
import ParsingPage from './pages/Page_Parser';
import BookingPage from './pages/Page_Booking';
import TicketPage from './pages/Page_Tickets';
import BookingConfirmPage from './pages/Page_Confirmation';
import './css/App.css';

function App() {
  
  return (
    <div className="app">
      <header className="app-header">
        <h1>Rail Network Bookings</h1>
      </header>
      
      <main>
        <Routes>
          <Route path="/" element={<ParsingPage />} />
          <Route path="/booking-page" element={<BookingPage />} />
          <Route path="/bookings-parser-page" element={<TicketPage />} />
          <Route path="/booking-confirm-page" element={<BookingConfirmPage />} />
        </Routes>
      </main>
    </div>
  );
}

export default App;
