use std::fmt;

const MINUTES_IN_ONE_HOUR: i32 = 60;
const HOURS_IN_ONE_DAY: i32 = 24;

#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {

    fn handle_minutes(minutes:i32) -> Self{

        let mut total_minutes:i32 = minutes;
        let mut total_hours:i32;
        
        if total_minutes < 0 && total_minutes % MINUTES_IN_ONE_HOUR != 0 {
            total_hours = (total_minutes / MINUTES_IN_ONE_HOUR) - 1; //round hours down if negative and is not on the hour
        } else {
            total_hours = total_minutes / MINUTES_IN_ONE_HOUR;
        }

        while total_hours < 0 {
            total_hours += HOURS_IN_ONE_DAY;
        } 

        while total_minutes < 0 {
            total_minutes += MINUTES_IN_ONE_HOUR;
        }

        Self { hours: (total_hours % HOURS_IN_ONE_DAY), minutes: (total_minutes % MINUTES_IN_ONE_HOUR) }

    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        
        let total_minutes:i32 = hours * MINUTES_IN_ONE_HOUR + minutes;

        Self::handle_minutes(total_minutes)

    }

    pub fn add_minutes(&self, minutes: i32) -> Self {

        let total_minutes:i32 = &self.hours * MINUTES_IN_ONE_HOUR + &self.minutes + minutes;

        Self::handle_minutes(total_minutes)
        
    }

    
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let hours_padded: String = format!("{0:0>2}", self.hours);
        let minutes_padded: String = format!("{0:0>2}", self.minutes);

        write!(f, "{}:{}", hours_padded, minutes_padded)
    }
}