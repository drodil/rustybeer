extern crate clap;
pub use crate::calculators::abv::Abv;
pub use crate::utils::beer_styles::{BeerStyle, BEER_STYLES};
use crate::AppSubCommand;
use clap::{App, Arg, ArgMatches, SubCommand};

pub struct BeerStyleFinder;

impl AppSubCommand for BeerStyleFinder {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b> {
        let ret = app.subcommand(
            SubCommand::with_name("beer_style")
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
                ),
        );
        ret
    }

    fn do_matches<'a>(&self, matches: &ArgMatches<'a>) {
        if let Some(ref matches) = matches.subcommand_matches("beer_style") {
            let mut resp = BEER_STYLES.to_vec();

            if let Some(og) = matches.value_of("og") {
                let og_value = og.parse::<f32>().unwrap();
                resp.retain(|&style| {
                    og_value > style.original_gravity_min && og_value < style.original_gravity_max
                });
            }

            if let Some(fg) = matches.value_of("fg") {
                let fg_value = fg.parse::<f32>().unwrap();
                resp.retain(|&style| {
                    fg_value > style.final_gravity_min && fg_value < style.final_gravity_max
                });
            }

            if let Some(abv) = matches.value_of("abv") {
                let abv_value = abv.parse::<f32>().unwrap();
                resp.retain(|&style| abv_value > style.abv_min && abv_value < style.abv_max);
            }

            if let Some(ibu) = matches.value_of("ibu") {
                let ibu_value = ibu.parse::<u8>().unwrap();
                resp.retain(|&style| ibu_value > style.ibu_min && ibu_value < style.ibu_max);
            }

            if let Some(srm) = matches.value_of("srm") {
                let srm_value = srm.parse::<f32>().unwrap();
                resp.retain(|&style| {
                    srm_value > style.color_srm_min && srm_value < style.color_srm_max
                });
            }

            if resp.is_empty() {
                println!("Could not find any beer styles matching criteria");
                return;
            }

            println!("Found the following beer styles with criteria:");
            for x in &resp {
                println!("* {}", x.name);
                println!(
                    "    OG: {}-{}, FG: {}-{}, ABV: {}%-{}%, IBU: {}-{}, SRM: {}-{}",
                    x.original_gravity_min,
                    x.original_gravity_max,
                    x.final_gravity_min,
                    x.final_gravity_max,
                    x.abv_min,
                    x.abv_max,
                    x.ibu_min,
                    x.ibu_max,
                    x.color_srm_min,
                    x.color_srm_max
                );
            }
        }
    }
}
