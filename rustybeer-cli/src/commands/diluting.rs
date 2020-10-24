use rustybeer::calculators::diluting::calculate_new_gravity;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "diluting",
    author = "Joseph Russell (josephrussell123@gmail.com)"
)]
/// Calculates the SG after dilution
pub struct DilutingOptions {
    #[structopt(short, long)]
    /// Current specific gravity
    sg: f32,

    #[structopt(short, long)]
    /// Current Volume
    cv: f32,

    #[structopt(short, long)]
    /// Target Volume
    tv: f32,
}

pub fn calculate_and_print(diluting_options: DilutingOptions) {
    println!(
        "New SG: {:.3}",
        calculate_new_gravity(
            diluting_options.sg,
            diluting_options.cv,
            diluting_options.tv
        )
    );
}
