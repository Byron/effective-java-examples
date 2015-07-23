extern crate chrono;

use std::string::ToString;

use chrono::*;

struct Period {
    start: Date<UTC>,
    end: Date<UTC>,
}

impl Period {
    pub fn new<D: Into<Date<UTC>>>(start: D, end: D) -> Period {
        let start = start.into();
        let end = end.into();

        assert!(start < end, "`start` date must be earlier than `end` date");
        Period {
            start: start,
            end: end,
        }
    }

    pub fn start(&self) -> &Date<UTC> {
        &self.start
    }

    pub fn end(&self) -> &Date<UTC> {
        &self.end
    }
}

impl ToString for Period {
    fn to_string(&self) -> String {
        format!("{} - {}", self.start(), self.end())
    }
}

#[test]
fn period() {
    Period::new(UTC.ymd(2014, 7, 8), UTC.ymd(2014, 7, 9));
}

#[test]
#[should_panic]
fn period_invariant_violation() {
    Period::new(UTC.ymd(2014, 7, 8), UTC.ymd(2013, 7, 9));   
}
