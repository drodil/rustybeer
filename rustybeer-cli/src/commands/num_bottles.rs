use clap::{value_t, App, Arg, ArgMatches};
use rustybeer::calculators::num_bottles::calculate_num_bottles;
use rustybeer_util::conversions::VolumeBuilder; // Converts string input to unit measurements

pub fn add_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new(
            "num_bottles")
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
        )
}

pub fn do_matches<'a>(matches: &ArgMatches<'a>) {
    if let Some(ref matches) = matches.subcommand_matches("num_bottles") {
        let vol = value_t!(matches, "volume", String).unwrap_or_else(|e| e.exit());
        let volume = VolumeBuilder::from_str(&vol).unwrap().as_milliliters();
        println!("Volume to contain: {}", vol);
        println!("=======================================================");
        let bottles = calculate_num_bottles(volume);
        for bottle in bottles {
            let output = format!(
                "Type: {0: <20} | Quantity required: {1: <5} |",
                bottle.0, bottle.1
            );
            println!("{}", output);
        }
    }
}
