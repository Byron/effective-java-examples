extern crate item33;

use item33::{Herb, Kind};
use item33::Kind::*;

#[test]
fn herb() {
    use std::collections::{HashMap, HashSet};
    use std::collections::hash_map::Entry::*;

    let garden = [
        Herb::new("Basil", Annual),
        Herb::new("Carroway", Biennial),
        Herb::new("Dill", Annual),
        Herb::new("Lavendar", Perennial),
        Herb::new("Parsley", Biennial), 
        Herb::new("Rosemary", Perennial),
    ];

    let herbs_by_type = {
        let mut h = HashMap::<Kind, HashSet<_>>::new();
        for herb in &garden {
            match h.entry(herb.kind.clone()) {
                Occupied(mut e) => { e.get_mut().insert(herb); },
                Vacant(e) => { e.insert(HashSet::new()).insert(herb); },
            }
        }
        h
    };

    println!("{:#?}", herbs_by_type);

    assert_eq!(*herbs_by_type.get(&Annual).unwrap(), {
            let mut h = HashSet::new();
            h.insert(&garden[0]);
            h.insert(&garden[2]);
            h });
    assert_eq!(*herbs_by_type.get(&Biennial).unwrap(), {
            let mut h = HashSet::new();
            h.insert(&garden[1]);
            h.insert(&garden[4]);
            h });
    assert_eq!(*herbs_by_type.get(&Perennial).unwrap(), {
            let mut h = HashSet::new();
            h.insert(&garden[3]);
            h.insert(&garden[5]);
            h });
}
