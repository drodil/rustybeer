pub struct SgCorrection;

impl SgCorrection {
    pub fn correct_sg(&self, sg: f64, ctf: f64, mtf: f64) -> f64 {
        sg * ((1.00130346 - 0.000134722124 * mtf + 0.00000204052596 * (mtf * mtf)
            - 0.00000000232820948 * (mtf * mtf * mtf))
            / (1.00130346 - 0.000134722124 * ctf + 0.00000204052596 * (ctf * ctf)
                - 0.00000000232820948 * (ctf * ctf * ctf)))
    }
}
