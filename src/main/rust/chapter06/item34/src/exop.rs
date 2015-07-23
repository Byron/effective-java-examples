use std::str::FromStr;

use super::Applicator;
use op::Operation;

use self::ExtendedOperation::*;



pub enum ExtendedOperation {
    Basic(Operation),
    Exp, 
    Remainder,
}

impl Applicator for ExtendedOperation {
    fn apply(&self, x: f64, y: f64) -> f64 {
        match *self {
            Basic(ref o) => o.apply(x, y),
            Exp => x.powf(y),
            Remainder => x % y,
        }
    }
}

impl AsRef<str> for ExtendedOperation {
    fn as_ref(&self) -> &str {
        match *self {
            Basic(ref o) => o.as_ref(),
            Exp => "^",
            Remainder => "%",
        }
    }
}

impl FromStr for ExtendedOperation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "^" => Exp,
            "%" => Remainder,
            _   => Basic(try!(Operation::from_str(s)))
        })
    }
}