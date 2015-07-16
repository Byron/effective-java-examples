extern crate chrono;
extern crate item05;

use item05::*;
use chrono::*;

#[test]
fn test_baby_boomer() {
    assert!(is_baby_boomer(&UTC.ymd(1946, 4, 2)));
    assert!(!is_baby_boomer(&UTC::today()));
}