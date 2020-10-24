use rustybeer::calculators::sg_correction::correct_sg;
use rustybeer_util::{conversions::TemperatureParser, measurements::Temperature};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "sg_correction",
    author("Joseph Russell (josephrussell123@gmail.com)")
)]
/// Corrects SG reading according to the difference between the measurement temperature and the calibration temperature
pub struct SgCorrectionOptions {
    #[structopt(short, long)]
    /// Specific gravity reading
    sg: f64,

    #[structopt(short, long, parse(try_from_str = TemperatureParser::parse))]
    /// Calibration temperature with unit (C, F, K, etc.). Defaults to Celsius.
    ct: Temperature,

    #[structopt(short, long, parse(try_from_str = TemperatureParser::parse))]
    /// Measurement temperature with unit (C, F, K, etc.). Defaults to Celsius.
    mt: Temperature,
}

pub fn calculate_and_print(sg_correction_options: SgCorrectionOptions) {
    let ct = sg_correction_options.ct.as_fahrenheit();
    let mt = sg_correction_options.mt.as_fahrenheit();
    println!("Measured gravity: {}", sg_correction_options.sg);
    println!("Calibration temperature: {} F", ct);
    println!("Measurement temperature: {} F", mt);
    println!(
        "Corrected gravity: {:.3}",
        correct_sg(sg_correction_options.sg, ct, mt)
    );
}
