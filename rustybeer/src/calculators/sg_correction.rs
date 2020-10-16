//! Is used to correct the specific gravity
//! reading according to the difference
//! between the measurement temperature
//! and the calibration temperature

/// Corrects a specific gravity value with respect to
/// a given calibration temperature and
/// measurement temperature
/// ```
/// use rustybeer::calculators::sg_correction::correct_sg;
///
/// assert_eq!(5.0002323479056585, correct_sg(5., 23., 22.));
/// ```
pub fn correct_sg(sg: f64, ctf: f64, mtf: f64) -> f64 {
    sg * ((1.00130346 - 0.000134722124 * mtf + 0.00000204052596 * (mtf * mtf)
        - 0.00000000232820948 * (mtf * mtf * mtf))
        / (1.00130346 - 0.000134722124 * ctf + 0.00000204052596 * (ctf * ctf)
            - 0.00000000232820948 * (ctf * ctf * ctf)))
}
