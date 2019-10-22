extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::AppSubCommand;

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

impl AppSubCommand for SgCorrection {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>{
        let ret = app.subcommand(SubCommand::with_name("sg_correction")
            .version("0.1")
            .author("Joseph Russell (josephrussell123@gmail.com)")
            .about("Corrects SG reading according to the difference between the measurement temperature and the calibration temperature")
            .arg(Arg::with_name("sg")
                 .short("s")
                 .long("sg")
                 .value_name("SG")
                 .help("Specific gravity reading")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("ct")
                 .short("c")
                 .long("ct")
                 .value_name("CT")
                 .help("Calibration temperature")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("mt")
                 .short("m")
                 .long("mt")
                 .value_name("MT")
                 .help("Measurement temperature")
                 .required(true)
                 .takes_value(true)));

        ret
    }

    fn do_matches<'a>(&self, matches: &ArgMatches<'a>) {
        if let Some(ref matches) = matches.subcommand_matches("sg_correction") {
            let sg = value_t!(matches, "sg", f32).unwrap_or_else(|e| e.exit());
            let ct = value_t!(matches, "ct", f32).unwrap_or_else(|e| e.exit());
            let mt = value_t!(matches, "mt", f32).unwrap_or_else(|e| e.exit());
            println!("Corrected SG: {:.3}", self.correct_sg(sg, ct, mt));
        }
    }
}

impl SgCorrection {
    fn correct_sg(&self, sg: f32, calibration_temperature: f32, measured_temperature: f32) -> f32 {
        let conversions = Conversions{temperature_units: 'C'}; // TODO: replace with global conversions
        let ctf = conversions.temperature_to_fahrenheit(calibration_temperature);
        let mtf = conversions.temperature_to_fahrenheit(measured_temperature);

        return sg * (  (1.00130346 - 0.000134722124 * mtf + 0.00000204052596 * (mtf * mtf) - 0.00000000232820948 * (mtf * mtf * mtf))
                     / (1.00130346 - 0.000134722124 * ctf + 0.00000204052596 * (ctf * ctf) - 0.00000000232820948 * (ctf * ctf * ctf)));
    }
}
