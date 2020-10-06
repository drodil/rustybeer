extern crate clap;
pub use crate::calculators::num_bottles::NumBottles;
use crate::AppSubCommand;
use clap::{value_t, App, Arg, ArgMatches, SubCommand};

impl AppSubCommand for NumBottles {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b> {
        let ret = app.subcommand(
            SubCommand::with_name("num_bottles")
                .version("0.1")
                .author("Ilakkiyan Jeyakumar (ilakkiyan.jeyakumar@gmail.com)")
                .about("Calculates the number of different standard-size bottles needed to contain a given volume")
                .arg(
                    Arg::with_name("volume")
                        .short("v")
                        .long("volume")
                        .value_name("vol")
                        .help("Volume as a string ('e.g 10mL, 4gal')")
                        .required(true)
                        .takes_value(true),
                ),
        );

        ret
    }

    fn do_matches<'a>(&self, matches: &ArgMatches<'a>) {
        if let Some(ref matches) = matches.subcommand_matches("num_bottles") {
            let vol = value_t!(matches, "volume", String).unwrap_or_else(|e| e.exit());
            println!("Volume to contain: {}", vol);
            println!("=======================================================");
            let bottles = self.calculate_num_bottles(vol);
            for bottle in bottles {
                let output = format!(
                    "Type: {0: <20} | Quantity required: {1: <5} |",
                    bottle.0, bottle.1
                );
                println!("{}", output);
            }
        }
    }
}
