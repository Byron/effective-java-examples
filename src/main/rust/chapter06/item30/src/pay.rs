use self::PayrollDay::*;

const HOURS_PER_SHIFT: f64 = 8.0;

pub enum PayrollDay {
    Monday,
    Friday,
    Sunday,
}

impl PayrollDay {
    
    fn overtime_pay(&self, hours: f64, pay_rate: f64) -> f64 {
        match *self {
            Monday
            |Friday => {
                if hours <= HOURS_PER_SHIFT {
                    0.0
                } else {
                    (hours - HOURS_PER_SHIFT) * pay_rate / 2.0
                }        
            },
            Sunday => hours * (pay_rate / 2.0)
        }
    }

    pub fn pay(&self, hours_worked: f64, pay_rate: f64) -> f64 {
        let base_pay = hours_worked * pay_rate;
        base_pay + self.overtime_pay(hours_worked, pay_rate)
    }
}