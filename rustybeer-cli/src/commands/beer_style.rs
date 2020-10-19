use clap::{App, Arg, ArgMatches};
pub use rustybeer_util::beer_styles::{BeerStyle, Criteria, BEER_STYLES};

pub fn add_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("beer_style")
        .version("0.1")
        .author("Heikki Hellgren (heiccih@gmail.com)")
        .about("Finds matches of beer style based on parameters")
        .arg(
            Arg::with_name("og")
                .short("o")
                .long("og")
                .value_name("OG")
                .help("Original gravity")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fg")
                .short("f")
                .long("fg")
                .value_name("FG")
                .help("Final gravity")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("abv")
                .short("a")
                .long("abv")
                .value_name("ABV")
                .help("Alcohol By Volume")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ibu")
                .short("i")
                .long("ibu")
                .value_name("IBU")
                .help("International Bittering Units")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .value_name("COLOR")
                .help("Standard Rererence Model Color")
                .takes_value(true),
        )
}

fn matches_to_criteria<'a>(matches: &ArgMatches<'a>) -> Criteria {
    Criteria {
        og: matches.value_of("og").map(|value| value.parse().unwrap()),
        fg: matches.value_of("fg").map(|value| value.parse().unwrap()),
        abv: matches.value_of("abv").map(|value| value.parse().unwrap()),
        ibu: matches.value_of("ibu").map(|value| value.parse().unwrap()),
        srm: matches.value_of("srm").map(|value| value.parse().unwrap()),
    }
}

pub fn do_matches<'a>(matches: &ArgMatches<'a>) {
    if let Some(matches) = matches.subcommand_matches("beer_style") {
        let criteria = matches_to_criteria(matches);
        let mut resp: Vec<&BeerStyle> = Vec::new();

        for style in BEER_STYLES.iter() {
            if criteria.matches(style) {
                resp.push(style)
            }
        }

        if resp.is_empty() {
            println!("Could not find any beer styles matching criteria");
            return;
        }

        println!("Found the following beer styles with criteria:");
        for x in &resp {
            println!("---------------------");
            println!("{}", x.name);
            println!("");
            println!("{}", x.description);
            println!("");
            println!("OG: {}-{}", x.original_gravity_min, x.original_gravity_max);
            println!("FG: {}-{}", x.final_gravity_min, x.final_gravity_max);
            println!("ABV: {}%-{}%", x.abv_min, x.abv_max);
            println!("IBU: {}-{}", x.ibu_min, x.ibu_max);
            println!("SRM: {}-{}", x.color_srm_min, x.color_srm_max);
        }
        println!("---------------------");
    }
}
