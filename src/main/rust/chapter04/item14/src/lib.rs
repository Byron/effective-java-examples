pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Time {
    hour: u8,
    minute: u8,
}

impl Time {
    pub fn new_checked(hour: u8, minute: u8) -> Result<Time, String> {
        if hour >= 24 {
            return Err(format!("{} must nut exceed 24", hour))
        }
        if minute >= 60 {
            return Err(format!("{}  must not exceed 60", minute))
        }

        Ok(Time {
            hour: hour, 
            minute: minute,
        })
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }
}

#[test]
fn access_private_point_fields() {
    let p = Point { x: 1.0, y: 2.0 };
    assert_eq!(p.x, 1.0);
}