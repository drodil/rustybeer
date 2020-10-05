use clap::{App, Arg, ArgMatches, SubCommand, value_t};
use crate::AppSubCommand;
pub use crate::calculators::sg_correction::SgCorrection;

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

