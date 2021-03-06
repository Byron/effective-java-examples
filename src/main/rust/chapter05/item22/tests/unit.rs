extern crate item22;

use item22::*;

#[test]
fn iter() {
    let f = Foo::new(4, 2);
    assert_eq!(f.iter().collect::<Vec<u16>>(), vec![4u16, 2]);
}

#[test]
fn into_iter() {
    let f = Foo::new(4, 2);
    assert_eq!(f.into_iter().collect::<Vec<u16>>(), vec![4u16, 2]);
}

#[test]
fn builder() {
    let f = Builder::new().a(4).b(4).build();
    assert_eq!(f.iter().collect::<Vec<u16>>(), vec![4u16, 4]);
}