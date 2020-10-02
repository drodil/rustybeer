extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::AppSubCommand;
pub use crate::calculators::abv::Abv;

impl AppSubCommand for Abv {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>{
        let ret = app.subcommand(SubCommand::with_name("abv")
            .version("0.1")
            .author("Heikki Hellgren (heiccih@gmail.com)")
            .about("Calculates ABV from original and final gravity")
            .arg(Arg::with_name("og")
                 .short("o")
                 .long("og")
                 .value_name("OG")
                 .help("Original gravity")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("fg")
                 .short("f")
                 .long("fg")
                 .value_name("FG")
                 .help("Final gravity")
                 .required(true)
                 .takes_value(true)));

        ret
    }

    fn do_matches<'a>(&self, matches: &ArgMatches<'a>) {
        if let Some(ref matches) = matches.subcommand_matches("abv") {
            let fg = value_t!(matches, "fg", f32).unwrap_or_else(|e| e.exit());
            let og = value_t!(matches, "og", f32).unwrap_or_else(|e| e.exit());
            println!("ABV: {:.3}%", self.calculate_abv(og, fg));
        }
    }
}

