//! Provides access to the [CODATA recommended values for physical constants][codata].
//!
//! The units are the appropriate prefix-less SI units unless otherwise noted in the name of the quantity. A listing of all quantities and their units are found in the [CODATA 2014 table][codata 2014]
//!
//! # Examples
//!
//! ```
//! use physical_constants;
//!
//! let epsilon_0 = physical_constants::ELECTRIC_CONSTANT;
//! let mu_0 = physical_constants::MAG_CONSTANT;
//! println!("speed of massless particles: {} m/s", 1f64/(epsilon_0*mu_0).sqrt());
//! println!("impedance of free space: {} Ω", (mu_0/epsilon_0).sqrt());
//! ```
//! [codata]: http://physics.nist.gov/cuu/Constants/
//! [codata 2014]: http://physics.nist.gov/cuu/Constants/Table/allascii.txt

// Include automatically generated function containing the physical constants.
include!(concat!(env!("OUT_DIR"), "/table.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    // Testing for exact equality of floating-point numbers is actually appropriate for once.

    #[test]
    fn test_exact_quantity() {
        // An exact quantity whose decimal expansion ends with "..." in the table
        assert_eq!(SPEED_OF_LIGHT_IN_VACUUM,
                   299792458f64);
    }

    #[test]
    fn test_no_exponent() {
        // A value that has no exponent in the listing
        assert_eq!(ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT_IN_MEV,
                   931.4940954);
    }

    #[test]
    fn test_positive_exponent() {
        assert_eq!(BOLTZMANN_CONSTANT_IN_HZ_PER_K,
                   2.0836612e10);
    }

    #[test]
    fn test_negative_exponent() {
        assert_eq!(CLASSICAL_ELECTRON_RADIUS,
                   2.8179403227e-15);
    }

    #[test]
    fn test_negative_value() {
        assert_eq!(ELECTRON_CHARGE_TO_MASS_QUOTIENT,
                   -1.758820024e11);
    }

    #[test]
    fn test_dimensionless_value() {
        assert_eq!(PROTON_ELECTRON_MASS_RATIO,
                   1836.15267389);
    }

    #[test]
    fn test_first_quantity() {
        // The first quantity listed in the table
        assert_eq!(LATTICE_SPACING_220_OF_SILICON,
                   192.0155714e-12);
    }

    #[test]
    fn test_last_quantity() {
        // The last quantity listed in the table
        assert_eq!(WIEN_WAVELENGTH_DISPLACEMENT_LAW_CONSTANT,
                   2.8977729e-3);
    }

    #[test]
    fn test_changed_name() {
        // This quantity is called "{220} lattice spacing of silicon" in the original listing. To get a valid identifier, which must not contain curly brackets and must not begin with a digit, its name has to be changed entirely, not just by a simple character replacement. This test checks that the new name came through as it should.
        assert_eq!(LATTICE_SPACING_220_OF_SILICON,
                   192.0155714e-12);
    }
}
