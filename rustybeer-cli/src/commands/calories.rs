use clap::{value_t, App, Arg, ArgMatches};
use rustybeer::calculators::calorie_counter::{
    calculate_alcohol_calories, calculate_carbs_calories, calculate_total_calories,
};
use rustybeer::calculators::num_bottles::bottles;
use rustybeer_util::abv_calories::{Criteria, ABV_CALORIES};
use rustybeer_util::conversions::{MassBuilder, VolumeBuilder};

pub fn add_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("calories")
            .version("0.1")
            .author("Roger Yu (roger.yu27 [at] gmail.com)")
            .about("Calculates calories by volume from original and final gravity or from alcohol by volume")
            .arg(Arg::with_name("og")
                 .short("o")
                 .long("og")
                 .value_name("OG")
                 .help("Original gravity")
                 .required(false)
                 .takes_value(true))
            .arg(Arg::with_name("fg")
                 .short("f")
                 .long("fg")
                 .value_name("FG")
                 .help("Final gravity")
                 .required(false)
                 .takes_value(true))
            .arg(Arg::with_name("abv")
                 .short("a")
                 .long("abv")
                 .value_name("ABV")
                 .help("Alcohol by volume")
                 .required_unless("fg")
                 .takes_value(true))
            .arg(Arg::with_name("volume")
                 .short("v")
                 .long("volume")
                 .value_name("vol")
                 .help("Volume as a string ('e.g 10mL, 4gal')")
                 .required(false)
                 .takes_value(true))
}

fn matches_to_criteria<'a>(matches: &ArgMatches<'a>) -> Criteria {
    Criteria {
        abv: matches.value_of("abv").map(|value| value.parse().unwrap()),
    }
}

pub fn do_matches<'a>(matches: &ArgMatches<'a>) {
    if let Some(matches) = matches.subcommand_matches("calories") {
        let conversion = MassBuilder::new("12oz").unwrap().as_grams() as f32;
        if matches.is_present("fg") && matches.is_present("og") && !matches.is_present("abv") {
            let fg = value_t!(matches, "fg", f32).unwrap_or_else(|e| e.exit());
            let og = value_t!(matches, "og", f32).unwrap_or_else(|e| e.exit());
            if matches.is_present("volume") {
                let vol = value_t!(matches, "volume", String).unwrap_or_else(|e| e.exit());
                let volume = VolumeBuilder::new(&vol).unwrap().as_milliliters();
                let ac = calculate_alcohol_calories(og, fg) / conversion * volume as f32;
                let cc = calculate_carbs_calories(og, fg) / conversion * volume as f32;
                let tc = calculate_total_calories(og, fg) / conversion * volume as f32;
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
                let bottles = calculate_calories_per_bottle(conversion, og, fg);
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
        } else if matches.is_present("abv")
            && !matches.is_present("fg")
            && !matches.is_present("og")
        {
            if matches.is_present("volume") {
                let vol = value_t!(matches, "volume", String).unwrap_or_else(|e| e.exit());
                let volume = VolumeBuilder::new(&vol).unwrap().as_milliliters() as f32;
                let criteria = matches_to_criteria(matches);
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
                let criteria = matches_to_criteria(matches);
                for abv in ABV_CALORIES.iter() {
                    if criteria.matches(abv) {
                        println!("Total estimated calories range for:");
                        println!("============================================================================");
                        let bottles = get_list_of_volumes_from_bottles();
                        for bottle in bottles {
                            let lc = abv.calories_low / conversion * bottle.1;
                            let hc = abv.calories_high / conversion * bottle.1;
                            let output = format!(
                                "| Type: {: <20} | kcal: {:>5.0} to {:<5.0} | kJ: {:>6.0} to {:<6.0} |",
                                bottle.0, lc, hc, lc * 4.184, hc * 4.184
                            );
                            println!("{}", output);
                        }
                        println!("============================================================================");
                        return;
                    }
                }
            };
            println!("Could not find any ABV to calories matching criteria (range: 0 to 22)");
        } else {
            println!("Either specify original gravity and final gravity or just abv!");
            println!("{}", matches.usage());
        }
    }
}

pub fn calculate_calories_per_bottle(conversion: f32, og: f32, fg: f32) -> Vec<(String, f32)> {
    let bottle_types = bottles();
    let mut bottle_counter: Vec<(String, f32)> = Vec::with_capacity(bottle_types.len());

    for bottle in bottle_types {
        let total_calories: f32 =
            (calculate_total_calories(og, fg) / conversion * bottle.1 as f32).ceil();
        bottle_counter.push((bottle.0, total_calories));
    }
    bottle_counter
}

pub fn get_list_of_volumes_from_bottles() -> Vec<(String, f32)> {
    let bottle_types = bottles();
    let mut bottle_counter: Vec<(String, f32)> = Vec::with_capacity(bottle_types.len());

    for bottle in bottle_types {
        bottle_counter.push((bottle.0, bottle.1 as f32));
    }
    bottle_counter
}
