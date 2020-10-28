use rustybeer::calculators::diluting::{calculate_new_gravity, calculate_new_volume};
use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "boil_off", group = ArgGroup::with_name("desired").required(true))]
/// Calculates how much you need to dilute or boil down your wort volume to hit a certain gravity
pub struct BoilOffOptions {
    #[structopt(short, long)]
    /// Wort Volume
    wort_volume: f32,

    #[structopt(short, long)]
    /// Current Gravity
    current_gravity: f32,

    #[structopt(short, long, group = "desired")]
    /// Desired Gravity
    desired_gravity: Option<f32>,

    #[structopt(short, long, group = "desired")]
    /// Target Volume
    target_volume: Option<f32>,
}

pub fn calculate_and_print(boil_off_options: BoilOffOptions) {
    println!("Wort Volume: {} l", boil_off_options.wort_volume);
    println!("Current Gravity: {}", boil_off_options.current_gravity);

    if let Some(desired_gravity) = boil_off_options.desired_gravity {
        let new_volume = calculate_new_volume(
            boil_off_options.wort_volume,
            boil_off_options.current_gravity,
            desired_gravity,
        );
        println!("New Volume: {}", new_volume);
        println!("Difference: {}", new_volume - boil_off_options.wort_volume);
    }

    if let Some(target_volume) = boil_off_options.target_volume {
        let new_gravity = calculate_new_gravity(
            boil_off_options.wort_volume,
            boil_off_options.current_gravity,
            target_volume,
        );
        println!("New Gravity: {}", new_gravity);
        println!(
            "Difference: {}",
            new_gravity - boil_off_options.current_gravity
        );
    }
}
