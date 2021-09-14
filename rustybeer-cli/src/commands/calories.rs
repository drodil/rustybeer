use rustybeer::calculators::calorie_counter::{
    calculate_alcohol_calories, calculate_carbs_calories, calculate_total_calories,
};
use rustybeer::calculators::num_bottles::bottles;
use rustybeer::{
    abv_calories::{Criteria, ABV_CALORIES},
    conversions::{MassParser, RelativeDensity, RelativeDensityParser, VolumeParser},
    measurements::Volume,
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "calories", author = "Roger Yu (roger.yu27 [at] gmail.com)")]
/// Calculates calories by volume from original and final gravity or from alcohol by volume
pub struct CaloriesOptions {
    #[structopt(short, long, parse(try_from_str = RelativeDensityParser::parse))]
    /// Original gravity
    og: Option<RelativeDensity>,

    #[structopt(short, long, requires("og"), required_unless("abv"), parse(try_from_str = RelativeDensityParser::parse))]
    /// Final gravity
    fg: Option<RelativeDensity>,

    #[structopt(short, long, required_unless("fg"))]
    /// Alcohol by volume
    abv: Option<f64>,

    #[structopt(short, long, parse(try_from_str = VolumeParser::parse))]
    /// Volume
    volume: Option<Volume>,
}

pub fn calculate_and_print(calories: CaloriesOptions) {
    let conversion = MassParser::parse("12oz").unwrap().as_grams();
    if calories.og.is_some() && calories.fg.is_some() {
        let fg = calories.fg.unwrap();
        let og = calories.og.unwrap();
        if let Some(volume) = calories.volume {
            let volume = volume.as_milliliters();
            let ac = calculate_alcohol_calories(&og, &fg) / conversion * volume;
            let cc = calculate_carbs_calories(&og, &fg) / conversion * volume;
            let tc = calculate_total_calories(&og, &fg) / conversion * volume;
            println!("Estimated calories for: {} ml", volume);
            println!("==============================");
            println!("| {:<8} | {:<6} | {:<6} |", "", "kcal:", "kJ:");
            println!("| {:<8} | {:>6.0} | {:>6.0} |", "Alcohol:", ac, ac * 4.184);
            println!("| {:<8} | {:>6.0} | {:>6.0} |", "Carbs:", cc, cc * 4.184);
            println!("| {:<8} | {:>6.0} | {:>6.0} |", "Total:", tc, tc * 4.184);
            println!("==============================");
        } else {
            println!("Total estimated calories for:");
            println!("==========================================================");
            let bottles = calculate_calories_per_bottle(conversion, &og, &fg);
            for bottle in bottles {
                let output = format!(
                    "| Type: {: <20} | kcal: {: >6} | kJ: {: >6.0} |",
                    bottle.0,
                    bottle.1,
                    bottle.1 * 4.184
                );
                println!("{}", output);
            }
            println!("==========================================================");
        }
    } else if calories.abv.is_some() {
        if let Some(volume) = calories.volume {
            let volume = volume.as_milliliters();
            let criteria = Criteria { abv: calories.abv };
            for abv in ABV_CALORIES.iter() {
                if criteria.matches(abv) {
                    let lc = abv.calories_low / conversion * volume;
                    let hc = abv.calories_high / conversion * volume;
                    println!("Total estimated calories range for: {} ml", volume);
                    println!("=========================");
                    println!("| {:>6.0} to {:<6.0} kcal |", lc, hc);
                    println!("| {:>6.0} to {:<6.0} kJ   |", lc * 4.184, hc * 4.184);
                    println!("=========================");
                    return;
                }
            }
        } else {
            let criteria = Criteria { abv: calories.abv };
            for abv in ABV_CALORIES.iter() {
                if criteria.matches(abv) {
                    println!("Total estimated calories range for:");
                    println!("============================================================================");
                    let bottles = get_list_of_volumes_from_bottles();
                    for bottle in bottles {
                        let lc = abv.calories_low / conversion * bottle.1;
                        let hc = abv.calories_high / conversion * bottle.1;
                        let output =
                            format!(
                            "| Type: {: <20} | kcal: {:>5.0} to {:<5.0} | kJ: {:>6.0} to {:<6.0} |",
                            bottle.0, lc, hc, lc * 4.184, hc * 4.184
                        );
                        println!("{}", output);
                    }
                    println!("============================================================================");
                    return;
                }
            }
        }
        println!("Could not find any ABV to calories matching criteria (range: 0 to 22)");
    }
}

pub fn calculate_calories_per_bottle(
    conversion: f64,
    og: &RelativeDensity,
    fg: &RelativeDensity,
) -> Vec<(String, f64)> {
    let bottle_types = bottles();
    let mut bottle_counter: Vec<(String, f64)> = Vec::with_capacity(bottle_types.len());

    for bottle in bottle_types {
        let total_calories: f64 = (calculate_total_calories(og, fg) / conversion * bottle.1).ceil();
        bottle_counter.push((bottle.0, total_calories));
    }
    bottle_counter
}

pub fn get_list_of_volumes_from_bottles() -> Vec<(String, f64)> {
    let bottle_types = bottles();
    let mut bottle_counter: Vec<(String, f64)> = Vec::with_capacity(bottle_types.len());

    for bottle in bottle_types {
        bottle_counter.push((bottle.0, bottle.1));
    }
    bottle_counter
}
