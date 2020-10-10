use clap::{value_t, App, Arg, ArgMatches};
pub use rustybeer::calculators::abv::{calculate_abv, calculate_fg};

pub fn add_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("abv")
            .version("0.1")
            .author("Heikki Hellgren (heiccih@gmail.com)")
            .about("Calculates Alcohol By Volue (ABV) from original and final gravity or final gravity from original gravity and ABV")
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
                 .required_unless("abv")
                 .takes_value(true))
            .arg(Arg::with_name("abv")
                 .short("a")
                 .long("abv")
                 .value_name("ABV")
                 .help("Alcohol by volume")
                 .required_unless("fg")
                 .takes_value(true))
}

pub fn do_matches<'a>(matches: &ArgMatches<'a>) {
    if let Some(matches) = matches.subcommand_matches("abv") {
        if matches.is_present("fg") {
            let fg = value_t!(matches, "fg", f32).unwrap_or_else(|e| e.exit());
            let og = value_t!(matches, "og", f32).unwrap_or_else(|e| e.exit());
            println!("ABV: {:.3}%", calculate_abv(og, fg));
        } else if matches.is_present("abv") {
            let og = value_t!(matches, "og", f32).unwrap_or_else(|e| e.exit());
            let abv = value_t!(matches, "abv", f32).unwrap_or_else(|e| e.exit());
            println!("FG: {:.3}", calculate_fg(og, abv));
        } else {
            println!("Either specify final gravity or abv!");
            println!("{}", matches.usage());
        }
    }
}
