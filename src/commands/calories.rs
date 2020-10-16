pub use crate::calculators::calorie_counter::{
    calculate_alcohol_calories, calculate_carbs_calories, calculate_total_calories,
};
use clap::{value_t, App, Arg, ArgMatches};

pub fn add_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("calories")
            .version("0.1")
            .about("Calculates Alcohol By Volue (ABV) from original and final gravity or final gravity from original gravity and ABV")
            .arg(Arg::with_name("og")
                 .short("o")
                 .long("og")
                 .value_name("OG")
                 .help("Original gravity")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("fg")
                 .short("f")
                 .long("fg")
                 .value_name("FG")
                 .help("Final gravity")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("abv")
                 .short("a")
                 .long("abv")
                 .value_name("ABV")
                 .help("Alcohol by volume")
                 .required_unless("fg")
                 .takes_value(true))
}

pub fn do_matches<'a>(matches: &ArgMatches<'a>) {
    if let Some(matches) = matches.subcommand_matches("calories") {
        if matches.is_present("fg") {
            let fg = value_t!(matches, "fg", f32).unwrap_or_else(|e| e.exit());
            let og = value_t!(matches, "og", f32).unwrap_or_else(|e| e.exit());
            println!(
                "Amount of alcohol calories: {:.2}",
                calculate_alcohol_calories(og, fg)
            );
            println!(
                "Amount of carbohydrates calories: {:.2}",
                calculate_carbs_calories(og, fg)
            );
            println!(
                "Total calories: {:.2} kcal per 12 ounces",
                calculate_total_calories(og, fg)
            );
        } else if matches.is_present("abv") {
            let og = value_t!(matches, "og", f32).unwrap_or_else(|e| e.exit());
            let abv = value_t!(matches, "abv", f32).unwrap_or_else(|e| e.exit());
            println!("kcal: {:.3}", calculate_carbs_calories(og, abv));
        } else {
            println!("Either specify original gravity and final gravity or just abv!");
            println!("{}", matches.usage());
        }
    }
}
