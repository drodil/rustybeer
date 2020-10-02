pub struct Conversions{ // TODO: replace with a global conversions, defined by a configuration initialised from main
    temperature_units: char,
}

impl Conversions {
    fn temperature_to_fahrenheit(&self, temp: f32) -> f32 {
        if self.temperature_units == 'F' {
            return temp;
        } else if self.temperature_units == 'K' {
            return (9.0/5.0) * temp - 459.67;
        } else {
            return (9.0/5.0) * temp + 32.0;
        }
    }
}

pub struct SgCorrection;

impl SgCorrection {
    pub fn correct_sg(&self, sg: f32, calibration_temperature: f32, measured_temperature: f32) -> f32 {
        let conversions = Conversions{temperature_units: 'C'}; // TODO: replace with global conversions
        let ctf = conversions.temperature_to_fahrenheit(calibration_temperature);
        let mtf = conversions.temperature_to_fahrenheit(measured_temperature);

        return sg * (  (1.00130346 - 0.000134722124 * mtf + 0.00000204052596 * (mtf * mtf) - 0.00000000232820948 * (mtf * mtf * mtf))
                     / (1.00130346 - 0.000134722124 * ctf + 0.00000204052596 * (ctf * ctf) - 0.00000000232820948 * (ctf * ctf * ctf)));
    }
}
