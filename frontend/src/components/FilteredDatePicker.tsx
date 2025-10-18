import React, { useState } from 'react';
import DatePicker from 'react-datepicker';
import "react-datepicker/dist/react-datepicker.css";

const Week = {
    Sunday: 0,
    Monday: 1,
    Tuesday: 2,
    Wednesday: 3,
    Thursday: 4,
    Friday: 5,
    Saturday: 6,
} as const;
type WeekDay = typeof Week[keyof typeof Week];

const isSelectableDay = (date: Date, arrdate: (keyof typeof Week)[]): boolean => {
    const day = date.getDay() as WeekDay;
    const allowedDayNumbers = arrdate.map(d => Week[d]);
    return allowedDayNumbers.includes(day);
};

interface DayProps {
  validDays: any; 
  onSelectedDate: (data: any) => void;
}

const FilteredDatePicker: React.FC<DayProps> = ({ validDays, onSelectedDate }) => {
  const [selectedDate, setSelectedDate] = useState<Date | null>(null);

  const setDate = (date: (Date | null)) => {
    setSelectedDate(date);
    onSelectedDate(date);
  }

  return (
    <DatePicker
      selected={selectedDate}
      onChange={date => setDate(date)}
      filterDate={date => isSelectableDay(date, validDays)}
      dateFormat="yyyy-MM-dd"
      required
    />
  );
};

export default FilteredDatePicker;

