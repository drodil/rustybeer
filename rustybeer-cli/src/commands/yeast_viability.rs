use chrono::format::ParseError;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use rustybeer::calculators::yeast_viability::{calculate_cc, calculate_yv};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "abv", author = "Philip Golovin")]
/// Estimates yeast viability based off production date
pub struct YeastViabilityOptions {
    #[structopt(short, long)]
    /// Production date
    pd: String,

    #[structopt(short, long)]
    /// Cell count
    cc: Option<f32>,

    #[structopt(short, long)]
    /// Date format (default: "%d/%m/%Y")
    f: Option<String>,
}

fn parse_date(date: String, format: String) -> Result<NaiveDateTime, ParseError> {
    let t = NaiveTime::from_hms(0, 0, 0);
    let date_only = NaiveDate::parse_from_str(&date, &format)?.and_time(t);
    Ok(date_only)
}

pub fn calculate_and_print(yv_options: YeastViabilityOptions) {
    let mut format = String::from("%d/%m/%Y");
    if let Some(f) = yv_options.f {
        format = f;
    }
    if let Ok(date) = parse_date(yv_options.pd, format) {
        let days = (Local::now().timestamp() - date.timestamp()) / Duration::days(1).num_seconds();
        println!("Yeast viability: {:.3}%", calculate_yv(days as f32));
        if let Some(cc) = yv_options.cc {
            println!("Cell count: {:.3}", calculate_cc(cc, days as f32))
        }
    } else {
        println!("Date is invalid.");
    }
}
