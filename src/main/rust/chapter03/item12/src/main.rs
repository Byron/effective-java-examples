extern crate item12;

use std::collections::BTreeSet;

use item12::PhoneNumber;

fn main() {
    fn new_random(s: u16) -> PhoneNumber {
        PhoneNumber::new_checked(s + 0, s + 1, s + 2).unwrap()
    }

    let mut m = BTreeSet::new();
    for i in (0..10).rev() {
        m.insert(new_random(i));
    }

    println!("{:#?}", m);
}