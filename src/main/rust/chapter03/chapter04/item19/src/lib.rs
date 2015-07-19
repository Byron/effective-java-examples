//! Traits cannot have data fields, whereas in java you can enforce static 
//! as well as instance fields.

mod constants {
    /// Avogadro's number (1/mol)
    pub const AVOGADROS_NUMBER: f64 = 6.02214199e23;

    /// Boltzmann constant (J/K)
    pub const BOLTZMANN_CONSTANT: f64 = 1.3806503e-23;

    /// Mass of the electron (kg)
    pub const  ELECTRON_MASS: f64 = 9.10938188e-31;
}

enum Constants {
    Boltzman,
    ElectronMass,   
}

#[cfg(test)]
mod tests {
    use super::constants::*;
    use super::Constants::*;

    #[test]
    fn const_use() {
        let _ = AVOGADROS_NUMBER * BOLTZMANN_CONSTANT * ELECTRON_MASS;
        let _ = Boltzman;
    }
}

