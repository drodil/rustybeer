use anyhow::{Context, Result};
use structopt::StructOpt;
mod commands;

#[derive(Debug, StructOpt)]
#[structopt(name = "RustyBeer", version = "0.1")]
/// RustyBeer Calculators CLI
pub enum RustyBeer {
    Abv(commands::abv::AbvOptions),
    AbvAbw(commands::alcohol_volume_weight::AbvAbwOptions),
    BeerStyle(commands::beer_style::BeerStyleOptions),
    BoilOff(commands::boil_off::BoilOffOptions),
    Calories(commands::calories::CaloriesOptions),
    Diluting(commands::diluting::DilutingOptions),
    Fg(commands::fg::FgOptions),
    NumBottles(commands::num_bottles::NumBottlesOptions),
    Priming(commands::priming::PrimingOptions),
    SgCorrection(commands::sg_correction::SgCorrectionOptions),
    YeastViability(commands::yeast_viability::YeastViabilityOptions),
}

fn main() -> Result<()> {
    let opt = RustyBeer::from_args_safe().with_context(|| "wrong arguments")?;
    match opt {
        RustyBeer::Abv(opts) => commands::abv::calculate_and_print(opts),
        RustyBeer::AbvAbw(opts) => commands::alcohol_volume_weight::calculate_and_print(opts),
        RustyBeer::BeerStyle(opts) => commands::beer_style::calculate_and_print(opts),
        RustyBeer::BoilOff(opts) => commands::boil_off::calculate_and_print(opts),
        RustyBeer::Calories(opts) => commands::calories::calculate_and_print(opts),
        RustyBeer::Diluting(opts) => commands::diluting::calculate_and_print(opts),
        RustyBeer::Fg(opts) => commands::fg::calculate_and_print(opts),
        RustyBeer::NumBottles(opts) => commands::num_bottles::calculate_and_print(opts),
        RustyBeer::Priming(opts) => commands::priming::calculate_and_print(opts),
        RustyBeer::SgCorrection(opts) => commands::sg_correction::calculate_and_print(opts),
        RustyBeer::YeastViability(opts) => commands::yeast_viability::calculate_and_print(opts),
    }

    Ok(())
}
