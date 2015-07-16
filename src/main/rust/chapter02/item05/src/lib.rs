extern crate chrono;

use chrono::*;

pub fn is_baby_boomer(date: &Date<UTC>) -> bool {
    BabyBoomChecker::new().is_baby_boomer(date)
}

pub struct BabyBoomChecker {
    start: Date<UTC>,
    end: Date<UTC>
}

impl BabyBoomChecker {
    pub fn new() -> BabyBoomChecker {
        BabyBoomChecker {
            start: UTC.ymd(1946, 1, 1),
            end: UTC.ymd(1965, 1, 1)
        }
    }

    pub fn is_baby_boomer(&self, date: &Date<UTC>) -> bool {
        *date >= self.start && *date <= self.end
    }
}
