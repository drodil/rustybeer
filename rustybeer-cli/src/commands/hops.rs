pub use rustybeer_util::hops::{Criteria, Hop, HOPS};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "hops", author = "Heikki Hellgren (heiccih@gmail.com)")]
/// Finds matches of hops based on parameters
pub struct HopOptions {
    #[structopt(short, long)]
    /// Alpha acid
    alpha_acid: Option<f64>,

    #[structopt(short, long)]
    /// Beta acid
    beta_acid: Option<f64>,

    #[structopt(short, long)]
    /// Hop purpose
    purpose: Option<String>,

    #[structopt(short, long)]
    /// Substitution of the hop
    substituted: Option<String>,
}

pub fn calculate_and_print(hop_options: HopOptions) {
    let criteria = Criteria {
        alpha_acid: hop_options.alpha_acid,
        beta_acid: hop_options.beta_acid,
        purpose: hop_options.purpose,
        substituted: hop_options.substituted,
    };

    let resp: Vec<&Hop> = HOPS.iter().filter(|hop| criteria.matches(hop)).collect();

    if resp.is_empty() {
        println!("Could not find any hops matching criteria");
        return;
    }

    println!("Found the following hops with criteria:");
    for x in &resp {
        println!("---------------------");
        println!("{}\n", x.name);
        println!("{}", x.description);
        println!("Country: {}", x.country);
        println!("Purpose: {}\n", x.purpose.join(", "));
        println!("Alpha acids: {}-{}", x.alpha_acid_min, x.alpha_acid_max);
        println!("Beta acids: {}-{}", x.beta_acid_min, x.beta_acid_max);
        println!("Substitutions: {}", x.substitutions.join(", "));
    }
    println!("---------------------");
}
