pub use rustybeer_util::beer_styles::{BeerStyle, Criteria, BEER_STYLES};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "beer_style", author = "Heikki Hellgren (heiccih@gmail.com)")]
/// Finds matches of beer style based on parameters
pub struct BeerStyleOptions {
    #[structopt(short, long)]
    /// Original gravity
    og: f32,

    #[structopt(short, long)]
    /// Final gravity
    fg: f32,

    #[structopt(short, long)]
    /// Alcohol by volume
    abv: f32,

    #[structopt(short, long)]
    /// International Bittering Units
    ibu: u8,

    #[structopt(short, long)]
    /// Standard Reference Model Color
    color: f32,
}

pub fn calculate_and_print(beer_style_options: BeerStyleOptions) {
    let criteria = Criteria {
        og: Some(beer_style_options.og),
        fg: Some(beer_style_options.fg),
        abv: Some(beer_style_options.abv),
        ibu: Some(beer_style_options.ibu),
        srm: Some(beer_style_options.color),
    };
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
