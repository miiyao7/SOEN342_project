import React, { useEffect, useState } from 'react';
import UserForm from './components/UserForm';
import './css/App.css';

function App() {
  
  return (
    <div className="app">
      <header className="app-header">
        <h1>Rail Network Parser</h1>
      </header>
      
      <main>
        <UserForm />
      </main>
    </div>
  );
}

export default App;
