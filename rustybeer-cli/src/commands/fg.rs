use rustybeer::calculators::fg::calculate_fg;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "fg")]
/// Calculates final gravity (FG) from original gravity (OG) and yeast attenuation
pub struct FgOptions {
    #[structopt(short, long)]
    /// Original gravity
    og: f32,

    #[structopt(short, long)]
    /// Yeast attenuation
    att: u8,
}

pub fn calculate_and_print(fg_options: FgOptions) {
    println!("FG: {:.3}", calculate_fg(fg_options.og, fg_options.att));
}
