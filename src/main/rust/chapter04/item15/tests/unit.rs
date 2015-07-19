extern crate item15;

use item15::{Complex, ONE};

#[test]
fn complex() {
    let c = Complex::new(1.0, 0.0);
    assert_eq!(c, ONE);
}