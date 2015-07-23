mod op;
mod exop;

pub trait Applicator {
    fn apply(&self, x: f64, y: f64) -> f64;
}

pub use op::Operation;
pub use exop::ExtendedOperation;