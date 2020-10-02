extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::AppSubCommand;
pub use crate::calculators::diluting::Diluting;

impl AppSubCommand for Diluting {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>{
        let ret = app.subcommand(SubCommand::with_name("diluting")
            .version("0.1")
            .author("Joseph Russell (josephrussell123@gmail.com)")
            .about("Calculates the SG after dilution")
            .arg(Arg::with_name("sg")
                 .short("g")
                 .long("sg")
                 .value_name("SG")
                 .help("Current specific gravity")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("cv")
                 .short("c")
                 .long("cv")
                 .value_name("CV")
                 .help("Current volume")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("tv")
                 .short("t")
                 .long("tv")
                 .value_name("TV")
                 .help("Target volume")
                 .required(true)
                 .takes_value(true)));

        ret
    }

    fn do_matches<'a>(&self, matches: &ArgMatches<'a>) {
        if let Some(ref matches) = matches.subcommand_matches("diluting") {
            let sg = value_t!(matches, "sg", f32).unwrap_or_else(|e| e.exit());
            let cv = value_t!(matches, "cv", f32).unwrap_or_else(|e| e.exit());
            let tv = value_t!(matches, "tv", f32).unwrap_or_else(|e| e.exit());
            println!("New SG: {:.3}", self.calculate_dilution(sg, cv, tv));
        }
    }
}


