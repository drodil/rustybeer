use rustybeer::calculators::diluting::calculate_new_gravity;
use rustybeer::conversions::{RelativeDensity, RelativeDensityParser, ToMap, VolumeParser};
use rustybeer::measurements::Volume;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "diluting",
    author = "Joseph Russell (josephrussell123@gmail.com)"
)]
/// Calculates the SG after dilution
pub struct DilutingOptions {
    #[structopt(short, long, parse(try_from_str = RelativeDensityParser::parse))]
    /// Current specific gravity
    sg: RelativeDensity,

    #[structopt(short, long, parse(try_from_str = VolumeParser::parse))]
    /// Current Volume
    cv: Volume,

    #[structopt(short, long, parse(try_from_str = VolumeParser::parse))]
    /// Target Volume
    tv: Volume,
}

pub fn calculate_and_print(diluting_options: DilutingOptions) {
    println!(
        "New SG: {:#?}",
        calculate_new_gravity(
            &diluting_options.sg,
            &diluting_options.cv,
            &diluting_options.tv
        )
        .to_map()
    );
}
