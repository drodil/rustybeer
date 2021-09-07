pub use rustybeer::calculators::alcohol_volume_weight::{
    calculate_abv_abw, calculate_abv_abw_density, calculate_abw_abv, calculate_abw_abv_density,
    calculate_alc_vol, calculate_alc_weight,
};
use rustybeer::{conversions::VolumeParser, measurements::Volume};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "abv_abw",
    author = "Joseph Russell (josephrussell123@gmail.com)"
)]
/// Calculates Alcohol by Weight (ABW) from Alcohol By Volue (ABV) and vice versa
pub struct AbvAbwOptions {
    #[structopt(short, long)]
    /// 'From' alcohol percentage
    percent: f64,

    #[structopt(short, long, parse(try_from_str = VolumeParser::parse))]
    /// Total beer volume
    volume: Option<Volume>,

    #[structopt(short, long, required_unless("fg"))]
    /// Total density of beer in g/cm³
    density: Option<f64>,

    #[structopt(short, long)]
    /// Calculates ABW to ABV
    reverse: Option<bool>,
}

pub fn calculate_and_print(abv_abw: AbvAbwOptions) {
    let end_percentage: f64;
    // main ABV <-> ABW conversion
    if Some(true) == abv_abw.reverse {
        end_percentage = match abv_abw.density {
            Some(density) => calculate_abw_abv_density(abv_abw.percent, density),
            None => calculate_abw_abv(abv_abw.percent),
        };

        println!("ABV: {:.3}%", end_percentage);
    } else {
        end_percentage = match abv_abw.density {
            Some(density) => calculate_abv_abw_density(abv_abw.percent, density),
            None => calculate_abv_abw(abv_abw.percent),
        };
        println!("ABW: {:.3}%", end_percentage);
    }

    // Quantity of alcohol
    if let Some(volume) = abv_abw.volume {
        if Some(true) == abv_abw.reverse {
            println!(
                "Alcohol: {:.3} ml",
                calculate_alc_vol(volume.as_millilitres(), end_percentage)
            );
        } else {
            println!(
                "Alcohol: {:.3} g",
                calculate_alc_weight(volume.as_millilitres(), abv_abw.percent)
            );
        }
    }
}
