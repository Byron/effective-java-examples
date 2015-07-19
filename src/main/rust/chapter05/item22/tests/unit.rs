extern crate item22;

use item22::*;

#[test]
fn iter() {
    let f = Foo {
        a: 4,
        b: 2,
    };

    assert_eq!(f.iter().collect::<Vec<u16>>(), vec![4u16, 2]);
}