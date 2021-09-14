//! Is used to correct the specific gravity
//! reading according to the difference
//! between the measurement temperature
//! and the calibration temperature

/// Corrects a specific gravity value with respect to
/// a given calibration temperature and
/// measurement temperature
/// ```
/// use rustybeer::calculators::sg_correction::correct_sg;
/// use rustybeer::conversions::RelativeDensity;
/// use rustybeer::measurements::Temperature;
///
/// assert_eq!(
///     RelativeDensity::from_specific_gravity(5.0002323479056585),
///     correct_sg(
///         &RelativeDensity::from_specific_gravity(5.),
///         &Temperature::from_fahrenheit(23.),
///         &Temperature::from_fahrenheit(22.)
///     )
/// );
/// ```
use crate::conversions::RelativeDensity;
use measurements::Temperature;

pub fn correct_sg(srd: &RelativeDensity, ct: &Temperature, mt: &Temperature) -> RelativeDensity {
    let sg = srd.as_specific_gravity();
    let ctf = ct.as_fahrenheit();
    let mtf = mt.as_fahrenheit();
    let corrected = sg
        * ((1.00130346 - 0.000134722124 * mtf + 0.00000204052596 * (mtf * mtf)
            - 0.00000000232820948 * (mtf * mtf * mtf))
            / (1.00130346 - 0.000134722124 * ctf + 0.00000204052596 * (ctf * ctf)
                - 0.00000000232820948 * (ctf * ctf * ctf)));
    RelativeDensity::from_specific_gravity(corrected)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::assert_approx;
    use crate::conversions::RelativeDensity;
    use measurements::Temperature;

    #[test]
    fn sg_correction() {
        assert_approx!(
            5.001,
            correct_sg(
                &RelativeDensity::from_specific_gravity(5.0),
                &Temperature::from_fahrenheit(2.9),
                &Temperature::from_fahrenheit(1.37)
            )
            .as_specific_gravity()
        );
        assert_approx!(
            7.3023,
            correct_sg(
                &RelativeDensity::from_specific_gravity(7.3),
                &Temperature::from_fahrenheit(8.1),
                &Temperature::from_fahrenheit(5.12)
            )
            .as_specific_gravity()
        );
        assert_approx!(
            7.403060483452605,
            correct_sg(
                &RelativeDensity::from_specific_gravity(7.413),
                &Temperature::from_fahrenheit(2.1),
                &Temperature::from_fahrenheit(55.1212)
            )
            .as_specific_gravity()
        );
    }
}
