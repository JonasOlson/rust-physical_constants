//! Provides access to the [CODATA recommended values for physical constants][codata].
//!
//! [codata]: http://physics.nist.gov/cuu/Constants/

mod table;

/// Takes a name of a physical constant and returns its value.
///
/// Names and units units are found in the [CODATA 2014 table][codata 2014].
///
/// # Examples
///
/// ```
/// use physical_constants::physical_constants;
///
/// let epsilon_0 = physical_constants("electric constant").unwrap();
/// let mu_0 = physical_constants("mag. constant").unwrap();
/// println!("speed of massless particles: {}", 1f64/(epsilon_0*mu_0).sqrt());
/// println!("impedance of free space: {}", (mu_0/epsilon_0).sqrt());
/// ```
///
/// [codata 2014]: http://physics.nist.gov/cuu/Constants/Table/allascii.txt
pub fn physical_constants(name: &str) -> Result<f64, ()> {
    table::table(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing for exact equality of floating-point numbers is actually appropriate for once.

    #[test]
    fn test_exact_quantity() {
        // An exact quantity whose decimal expansion ends with "..." in the table
        assert_eq!(physical_constants("speed of light in vacuum").unwrap(),
                   299792458f64);
    }

    #[test]
    fn test_no_exponent() {
        // A value that has no exponent in the listing
        assert_eq!(physical_constants("atomic mass constant energy equivalent in MeV").unwrap(),
                   931.4940954);
    }

    #[test]
    fn test_positive_exponent() {
        assert_eq!(physical_constants("Boltzmann constant in Hz/K").unwrap(),
                   2.0836612e10);
    }

    #[test]
    fn test_negative_exponent() {
        assert_eq!(physical_constants("classical electron radius").unwrap(),
                   2.8179403227e-15);
    }

    #[test]
    fn test_negative_value() {
        assert_eq!(physical_constants("electron charge to mass quotient").unwrap(),
                   -1.758820024e11);
    }

    #[test]
    fn test_dimensionless_value() {
        assert_eq!(physical_constants("proton-electron mass ratio").unwrap(),
                   1836.15267389);
    }

    #[test]
    fn test_first_quantity() {
        // The first quantity listed in the table
        assert_eq!(physical_constants("{220} lattice spacing of silicon").unwrap(),
                   192.0155714e-12);
    }

    #[test]
    fn test_last_quantity() {
        // The last quantity listed in the table
        assert_eq!(physical_constants("Wien wavelength displacement law constant").unwrap(),
                   2.8977729e-3);
    }

    #[test]
    fn test_nonalphanumeric_name() {
        assert_eq!(physical_constants("{220} lattice spacing of silicon").unwrap(),
                   192.0155714e-12);
    }
}
