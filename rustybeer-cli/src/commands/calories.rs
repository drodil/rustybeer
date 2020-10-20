use clap::{value_t, App, Arg, ArgMatches};
pub use rustybeer::calculators::calorie_counter::{
    calculate_alcohol_calories, calculate_carbs_calories, calculate_total_calories,
};
pub use rustybeer_util::abv_calories::{ABVCalories, Criteria, ABV_CALORIES};
pub use rustybeer_util::conversions::{MassBuilder, VolumeBuilder};

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
                 .help("volume as string ('e.g 10m, 4gal')")
                 .required(false)
                 .default_value("100ml")
                 .takes_value(true))
}

fn matches_to_criteria<'a>(matches: &ArgMatches<'a>) -> Criteria {
    Criteria {
        abv: matches.value_of("abv").map(|value| value.parse().unwrap()),
    }
}

pub fn do_matches<'a>(matches: &ArgMatches<'a>) {
    if let Some(matches) = matches.subcommand_matches("calories") {
        if matches.is_present("fg") && matches.is_present("og") {
            let fg = value_t!(matches, "fg", f32).unwrap_or_else(|e| e.exit());
            let og = value_t!(matches, "og", f32).unwrap_or_else(|e| e.exit());
            let cv = value_t!(matches, "volume", String).unwrap_or_else(|e| e.exit());
            let conversion = MassBuilder::from_str("12oz").unwrap().as_grams() as f32;
            let vol = VolumeBuilder::from_str(&cv).unwrap().as_milliliters() as f32;
            let ac = calculate_alcohol_calories(og, fg) / conversion * vol;
            let cc = calculate_carbs_calories(og, fg) / conversion * vol;
            let tc = calculate_total_calories(og, fg) / conversion * vol;
            println!("{:<8} {:>8} {:>8}", "", "kcal", "kJ");
            println!("{:<8} {:>8.0} {:>8.0}", "Alcohol:", ac, ac * 4.184);
            println!("{:<8} {:>8.0} {:>8.0}", "Carbs:", cc, cc * 4.184);
            println!("{:<8} {:>8.0} {:>8.0}", "Total:", tc, tc * 4.184);
            println!("Per: {:.0} ml", vol);
        } else if matches.is_present("abv") {
            let cv = value_t!(matches, "volume", String).unwrap_or_else(|e| e.exit());
            let criteria = matches_to_criteria(matches);

            for abv in ABV_CALORIES.iter() {
                if criteria.matches(abv) {
                    let conversion = MassBuilder::from_str("12oz").unwrap().as_grams() as f32;
                    let vol = VolumeBuilder::from_str(&cv).unwrap().as_milliliters() as f32;
                    let lc = abv.calories_low / conversion * vol;
                    let hc = abv.calories_high / conversion * vol;
                    println!("Calories: {:.0} to {:.0} kcal", lc, hc);
                    println!("          {:.0} to {:.0} kJ", lc * 4.184, hc * 4.184);
                    println!("Per: {:.0} ml", vol);
                    return;
                }
            }

            println!("Could not find any ABV to calories matching criteria");
            return;
        } else {
            println!("Either specify original gravity and final gravity or just abv!");
            println!("{}", matches.usage());
        }
    }
}
