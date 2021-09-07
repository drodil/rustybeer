pub use rustybeer::beer_styles::{BeerStyle, Criteria, BEER_STYLES};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "beer_style", author = "Heikki Hellgren (heiccih@gmail.com)")]
/// Finds matches of beer style based on parameters
pub struct BeerStyleOptions {
    #[structopt(short, long)]
    /// Beer style name
    name: Option<String>,

    #[structopt(short, long)]
    /// Original gravity
    og: Option<f32>,

    #[structopt(short, long)]
    /// Final gravity
    fg: Option<f32>,

    #[structopt(short, long)]
    /// Alcohol by volume
    abv: Option<f32>,

    #[structopt(short, long)]
    /// International Bittering Units
    ibu: Option<u8>,

    #[structopt(short, long)]
    /// Standard Reference Model Color
    color: Option<f32>,
}

pub fn calculate_and_print(beer_style_options: BeerStyleOptions) {
    let criteria = Criteria {
        name: beer_style_options.name,
        og: beer_style_options.og,
        fg: beer_style_options.fg,
        abv: beer_style_options.abv,
        ibu: beer_style_options.ibu,
        srm: beer_style_options.color,
    };

    let resp: Vec<&BeerStyle> = BEER_STYLES
        .iter()
        .filter(|style| criteria.matches(style))
        .collect();

    if resp.is_empty() {
        println!("Could not find any beer styles matching criteria");
        return;
    }

    println!("Found the following beer styles with criteria:");
    for x in &resp {
        println!("---------------------");
        println!("{}\n", x.name);
        println!("{}\n", x.description);
        println!("OG: {}-{}", x.original_gravity_min, x.original_gravity_max);
        println!("FG: {}-{}", x.final_gravity_min, x.final_gravity_max);
        println!("ABV: {}%-{}%", x.abv_min, x.abv_max);
        println!("IBU: {}-{}", x.ibu_min, x.ibu_max);
        println!("SRM: {}-{}", x.color_srm_min, x.color_srm_max);
    }
    println!("---------------------");
}
