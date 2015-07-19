extern crate item14;

use item14::{Point, Time};

#[test]
fn point_access() {
    let p = Point { x: 1.0, y: 2.0 };
    assert_eq!(p.x, 1.0);
}

#[test]
fn time() {
    assert!(Time::new_checked(25, 20).is_err());
    assert!(Time::new_checked(1, 61).is_err());

    let t = Time::new_checked(4, 20).unwrap();
    assert_eq!(t.hour(), 4);
    assert_eq!(t.minute(), 20);
}