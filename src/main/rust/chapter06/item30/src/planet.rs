use self::Planet::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Planet {
    Earth,
    Mars,
    Jupiter,
}

impl Planet {
    pub fn mass(&self) -> f64 {
        match *self {
            Earth => 5.975e+24,
            Mars => 6.419e+23,
            Jupiter => 1.899e+27,
        }
    }

    pub fn radius(&self) -> f64 {
        match *self {
            Earth => 6.378e6,
            Mars => 3.393e6,
            Jupiter => 7.149e7,
        }
    }

    pub fn surface_gravity(&self) -> f64 {
        6.67300E-11 * self.mass() / self.radius().powi(2)
    }

    pub fn surface_weight(&self, mass: f64) -> f64 {
        mass * self.surface_gravity()
    }
}

impl AsRef<str> for Planet {
    fn as_ref(&self) -> &str {
        match *self {
            Earth => "Earth",
            Mars => "Mars",
            Jupiter => "Jupiter"
        }
    }
}