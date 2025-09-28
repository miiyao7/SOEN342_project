import React, { useEffect, useState } from 'react';
import UserForm from './components/UserForm';
import MatchDisplay from './components/MatchDisplay';
import './css/App.css';

function App() {
  const [data, setSheet] = useState([]);

  useEffect(() => {
    const sheet = localStorage.getItem('spreadSheet');
    if (sheet) {
      setSheet(JSON.parse(sheet));
    }
  }, [data]);
  
  return (
    <div className="app">
      <header className="app-header">
        <h1>Spread Sheet Parser</h1>
      </header>
      
      <main>
        <UserForm />
        <MatchDisplay />
      </main>
    </div>
  );
}

export default App;
