use std::str::FromStr;

use self::Operation::*;


pub enum Operation {
    Plus, 
    Minus,
    Times,
    Divide,
}

impl Operation {
    pub fn apply(&self, x: f64, y: f64) -> f64 {
        match *self {
            Plus => x + y,
            Minus => x - y,
            Times => x * y,
            Divide => x / y
        }
    }
}

impl AsRef<str> for Operation {
    fn as_ref(&self) -> &str {
        match *self {
            Plus => "+",
            Minus => "-",
            Times => "*",
            Divide => "/"
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Plus,
            "-" => Minus,
            "*" => Times,
            "/" => Divide,
            _   => return Err(format!("'{}' is not a valid operation", s))
        })
    }
}