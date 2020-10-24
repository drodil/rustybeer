use rustybeer::calculators::abv::{calculate_abv, calculate_fg};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "abv", author = "Heikki Hellgren (heiccih@gmail.com)")]
/// Calculates Alcohol By Volume (ABV) from original and final gravity or final gravity from original gravity and ABV
pub struct AbvOptions {
    #[structopt(short, long)]
    /// Original gravity
    og: f32,

    #[structopt(short, long, required_unless("abv"))]
    /// Final gravity
    fg: Option<f32>,

    #[structopt(short, long, required_unless("fg"))]
    /// Alcohol by volume
    abv: Option<f32>,
}

pub fn calculate_and_print(abv_options: AbvOptions) {
    if let Some(fg) = abv_options.fg {
        println!("ABV: {:.3}%", calculate_abv(abv_options.og, fg));
    }

    if let Some(abv) = abv_options.abv {
        println!("ABV: {:.3}%", calculate_fg(abv_options.og, abv));
    }
}
