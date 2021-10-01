use rustybeer::conversions::TemperatureParser;
use rustybeer::measurements::Temperature;
use rustybeer::yeasts::{get_yeasts, Criteria, Yeast};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "yeast")]
/// Search yeast information
pub struct YeastOptions {
    #[structopt(short, long)]
    /// Search by yeast name
    name: Option<String>,
    #[structopt(short, long)]
    /// Search by yeast producer
    company: Option<String>,
    #[structopt(short, long)]
    /// Search by yeast attenuation
    attenuation: Option<u8>,
    #[structopt(short, long, parse(try_from_str = TemperatureParser::parse))]
    /// Search by yeast optimal temperature
    temperature: Option<Temperature>,
}

pub fn search_and_print(opt: YeastOptions) {
    let criteria = Criteria {
        name: opt.name,
        company: opt.company,
        attenuation: opt.attenuation,
        temperature: opt.temperature,
    };

    let resp: Vec<&Yeast> = get_yeasts(Some(criteria));

    if resp.is_empty() {
        println!("Could not find any yeasts matching criteria");
        return;
    }

    println!("Found the following yeasts with criteria:");
    for x in &resp {
        println!("---------------------");
        println!("{}", x.name);
        println!("{}\n", x.company);
        println!(
            "Attenuation: {}-{}",
            x.min_attenuation.unwrap_or(0),
            x.max_attenuation.unwrap_or(0)
        );
        println!(
            "Temperature: {}-{}",
            x.min_temp
                .unwrap_or(Temperature::from_celsius(0.0))
                .as_celsius(),
            x.max_temp
                .unwrap_or(Temperature::from_celsius(0.0))
                .as_celsius()
        );
        println!("Alcohol tolerance: {}", x.alc_tolerance.unwrap_or(0));
    }
    println!("---------------------");
}
