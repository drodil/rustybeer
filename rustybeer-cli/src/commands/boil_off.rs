use rustybeer::calculators::diluting::{calculate_new_gravity, calculate_new_volume};
use rustybeer::conversions::{RelativeDensity, RelativeDensityParser, ToMap, VolumeParser};
use rustybeer::measurements::Volume;
use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "boil_off", group = ArgGroup::with_name("desired").required(true))]
/// Calculates how much you need to dilute or boil down your wort volume to hit a certain gravity
pub struct BoilOffOptions {
    #[structopt(short, long, parse(try_from_str = VolumeParser::parse))]
    /// Wort Volume
    wort_volume: Volume,

    #[structopt(short, long, parse(try_from_str = RelativeDensityParser::parse))]
    /// Current Gravity
    current_gravity: RelativeDensity,

    #[structopt(short, long, group = "desired", parse(try_from_str = RelativeDensityParser::parse))]
    /// Desired Gravity
    desired_gravity: Option<RelativeDensity>,

    #[structopt(short, long, group = "desired", parse(try_from_str = VolumeParser::parse))]
    /// Target Volume
    target_volume: Option<Volume>,
}

pub fn calculate_and_print(boil_off_options: BoilOffOptions) {
    println!("Wort Volume: {:#?}", boil_off_options.wort_volume.to_map());
    println!(
        "Current Gravity: {:#?}",
        boil_off_options.current_gravity.to_map()
    );

    if let Some(desired_gravity) = boil_off_options.desired_gravity {
        let new_volume = calculate_new_volume(
            &boil_off_options.current_gravity,
            &boil_off_options.wort_volume,
            &desired_gravity,
        );
        println!("New Volume: {:#?}", new_volume.to_map());
        println!(
            "Difference: {:#?}",
            (new_volume - boil_off_options.wort_volume).to_map()
        );
    }

    if let Some(target_volume) = boil_off_options.target_volume {
        let new_gravity = calculate_new_gravity(
            &boil_off_options.current_gravity,
            &boil_off_options.wort_volume,
            &target_volume,
        );
        println!("New Gravity: {:#?}", new_gravity.to_map());
        println!(
            "Difference: {:#?}",
            (new_gravity - boil_off_options.current_gravity).to_map()
        );
    }
}
