
pub const ZERO: Complex = Complex {re: 0.0, im: 0.0};
pub const ONE: Complex = Complex {re: 1.0, im: 0.0};
pub const I: Complex = Complex {re: 0.0, im: 1.0};

#[derive(PartialEq, Debug)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex {
            re: re, 
            im: im
        }
    }

    pub fn new_polar(r: f64, theta: f64) -> Complex {
        Complex {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    pub fn real(&self) -> f64 {
        self.re
    }

    pub fn imaginary(&self) -> f64 {
        self.im
    }

    pub fn add(&self, c: &Complex) -> Complex {
        Complex {
            re: self.re + c.re,
            im: self.im + c.im
        }
    }
}