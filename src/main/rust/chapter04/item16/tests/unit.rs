extern crate item16;

use std::collections::HashSet;
use item16::*;

#[test]
fn forwarding_set() {
    let mut s = InstrumentedSet::new(HashSet::new());
    
    assert_eq!(s.add_count(), 0);
    assert!(s.add("foo") == true);
    assert_eq!(s.add_count(), 1);
    s.add_all(vec!["foo", "bar"]);
    assert_eq!(s.add_count(), 2);
}