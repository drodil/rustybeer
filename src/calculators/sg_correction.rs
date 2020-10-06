pub struct SgCorrection;

impl SgCorrection {
    pub fn correct_sg(&self, sg: f64, ctf: f64, mtf: f64) -> f64 {
        return sg * (  (1.00130346 - 0.000134722124 * mtf + 0.00000204052596 * (mtf * mtf) - 0.00000000232820948 * (mtf * mtf * mtf))
                     / (1.00130346 - 0.000134722124 * ctf + 0.00000204052596 * (ctf * ctf) - 0.00000000232820948 * (ctf * ctf * ctf)));
    }
}

pub enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

pub struct TempParseError;

impl std::convert::TryFrom<char> for Temperature {
    type Error = TempParseError;

    fn try_from(mut value: char) -> Result<Self, Self::Error> {
        value.make_ascii_lowercase();
        match value {
            'c' => Ok(Self::Celsius),
            'f' => Ok(Self::Fahrenheit),
            'k' => Ok(Self::Kelvin),
            _ => Err(TempParseError),
        }
    }
}

impl Temperature {
    // The dead code exists incase someone wants to do other conversions
    #[allow(dead_code)]
    pub fn to_celsius(&self, temp: f32) -> f32 {
        match self {
            Temperature::Celsius => temp,
            Temperature::Fahrenheit => (temp - 32.) / 1.8,
            Temperature::Kelvin => temp - 273.15,
        }
    }
    pub fn to_fahrenheit(&self, temp: f32) -> f32 {
        match self {
            Temperature::Celsius => temp * 1.8 + 32.,
            Temperature::Fahrenheit => temp,
            Temperature::Kelvin => 9. / 5. * (temp - 273.15) + 32.,
        }
    }
    #[allow(dead_code)]
    pub fn to_kelvin(&self, temp: f32) -> f32 {
        match self {
            Temperature::Celsius => temp + 273.15,
            Temperature::Fahrenheit => 5. / 9. * (temp - 32.) + 273.15,
            Temperature::Kelvin => temp,
        }
    }
}