extern crate item34;

use item34::{Operation, ExtendedOperation, Applicator};

#[test]
fn dynamic_dispatch() {
    fn dynamic(op: &Applicator, x: f64, y: f64) -> f64 {
        op.apply(x, y)
    }

    assert_eq!(dynamic(&"^".parse::<ExtendedOperation>().unwrap(), 2.0, 2.0), 4.0);
    assert_eq!(dynamic(&"-".parse::<ExtendedOperation>().unwrap(), 2.0, 2.0), 0.0);
    assert_eq!(dynamic(&"-".parse::<Operation>().unwrap(), 2.0, 2.0), 0.0);
    assert_eq!(dynamic(&Operation::Plus, 2.0, 2.0), 4.0);
    assert_eq!(dynamic(&ExtendedOperation::Basic(Operation::Plus), 2.0, 2.0), 4.0);
}