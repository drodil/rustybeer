use rustybeer::calculators::fg::calculate_fg;
use rustybeer::conversions::{RelativeDensity, RelativeDensityParser, ToMap};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "fg")]
/// Calculates final gravity (FG) from original gravity (OG) and yeast attenuation
pub struct FgOptions {
    #[structopt(short, long, parse(try_from_str = RelativeDensityParser::parse))]
    /// Original gravity
    og: RelativeDensity,

    #[structopt(short, long)]
    /// Yeast attenuation
    att: u8,
}

pub fn calculate_and_print(fg_options: FgOptions) {
    println!(
        "FG: {:#?}",
        calculate_fg(&fg_options.og, fg_options.att).to_map()
    );
}
