use rustybeer::calculators::num_bottles::calculate_num_bottles;
use rustybeer::{conversions::VolumeParser, measurements::Volume};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "num_bottles",
    author = "Ilakkiyan Jeyakumar (ilakkiyan.jeyakumar@gmail.com)"
)]
/// Calculates the number of different standard-size bottles needed to contain a given volume
pub struct NumBottlesOptions {
    #[structopt(short, long, parse(try_from_str = VolumeParser::parse))]
    /// Volume as a string ('e.g 10mL, 4gal')
    volume: Volume,
}

pub fn calculate_and_print(num_bottles_options: NumBottlesOptions) {
    let volume = num_bottles_options.volume.as_milliliters();
    println!("Volume to contain: {} ml", volume);
    println!("=========================================================");
    let bottles = calculate_num_bottles(volume);
    for bottle in bottles {
        let output = format!(
            "| Type: {0: <20} | Quantity required: {1: <5} |",
            bottle.0, bottle.1
        );
        println!("{}", output);
    }
    println!("=========================================================");
}
