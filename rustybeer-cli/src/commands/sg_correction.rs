use rustybeer::calculators::sg_correction::correct_sg;
use rustybeer::{
    conversions::{RelativeDensity, RelativeDensityParser, TemperatureParser, ToMap},
    measurements::Temperature,
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "sg_correction",
    author("Joseph Russell (josephrussell123@gmail.com)")
)]
/// Corrects SG reading according to the difference between the measurement temperature and the calibration temperature
pub struct SgCorrectionOptions {
    #[structopt(short, long, parse(try_from_str = RelativeDensityParser::parse))]
    /// Specific gravity reading
    sg: RelativeDensity,

    #[structopt(short, long, parse(try_from_str = TemperatureParser::parse))]
    /// Calibration temperature with unit (C, F, K, etc.). Defaults to Celsius.
    ct: Temperature,

    #[structopt(short, long, parse(try_from_str = TemperatureParser::parse))]
    /// Measurement temperature with unit (C, F, K, etc.). Defaults to Celsius.
    mt: Temperature,
}

pub fn calculate_and_print(sg_correction_options: SgCorrectionOptions) {
    println!("Measured gravity: {:#?}", sg_correction_options.sg.to_map());
    println!(
        "Calibration temperature: {:#?}",
        sg_correction_options.ct.to_map()
    );
    println!(
        "Measurement temperature: {:#?}",
        sg_correction_options.mt.to_map()
    );
    println!(
        "Corrected gravity: {:#?}",
        correct_sg(
            &sg_correction_options.sg,
            &sg_correction_options.ct,
            &sg_correction_options.mt
        )
        .to_map()
    );
}
