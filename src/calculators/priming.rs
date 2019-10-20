extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches};
use crate::AppSubCommand;

use std::error::Error;
use futures::{future, Future, stream, Stream};

pub struct Priming;

impl AppSubCommand for Priming {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>{
        app.subcommand(SubCommand::with_name("priming")
                .about("Beer Priming Calculator")   // The message displayed in "-h"
                .arg(Arg::with_name("temp")         // Priming own arguments
                        .long("temp")
                        .short("t")
                        .help("Temperature of Beer (in Celsius, only integers)")
                        .required(true)
                        .takes_value(true))
                .arg(Arg::with_name("amount")
                        .long("amount")
                        .short("a")
                        .help("Amount Being Packaged (in Liters)")
                        .default_value("1")
                        .takes_value(true))
                .arg(Arg::with_name("co2_volumes")
                        .long("co2_volumes")
                        .short("c")
                        .help("Volumes of CO2, depends on beer style (e.g. British Style Ales 1.5 to 2.0)")
                        .default_value("2.0")
                        .takes_value(true))
            )
    }

    fn do_matches<'c>(&self, matches: &ArgMatches<'c>){
        if let Some(ref sub_matches) = matches.subcommand_matches("priming") {
            let temprature = value_t!(sub_matches, "temp", i32).unwrap_or_else(|e| e.exit());
            let amount = value_t!(sub_matches, "amount", f32).unwrap_or_else(|e| e.exit());
            let co2_volumes = value_t!(sub_matches, "co2_volumes", f32).unwrap_or_else(|e| e.exit());

            let fahrenheit = self.celsius_to_fahrenheit(temprature as f32);
            let co2_beer = self.calculate_co2(fahrenheit);
            let sugars = self.calculate_sugars(fahrenheit, amount, co2_volumes);

            println!("Amount (Liters): {}", amount);
            println!("Volumes of CO2: {}", co2_volumes);
            println!("Temprature (C): {}", temprature);
            println!("CO2 in Beer: {} volumes", co2_beer);
            println!("Priming Sugar Options:");
            sugars.for_each(|sugar| {
                println!("{:>23}: {:.2} g", sugar.name, sugar.ratio);
                Ok(())
            }).wait();
        }
    }
}

impl Priming {
    fn celsius_to_fahrenheit(&self, celsius: f32) -> f32 {
        (9.0/5.0) * celsius + 32.0
    }

    fn calculate_co2(&self, fahrenheit: f32) -> f32 {
        (3.0378 - 0.050062 * fahrenheit + 0.00026555 * fahrenheit.powf(2.0))
    }

    fn calculate_sugars(&self, fahrenheit: f32, amount: f32, co2_volumes: f32) -> Box<dyn Stream<Item=Sugar, Error=Box<dyn Error>>>
    {
        let sugars = vec![
            Sugar{name: String::from("Table Sugar (sucrose)"), ratio: 1.0},
            Sugar{name: String::from("Corn Sugar (dextrose)"), ratio: 1.0/0.91},
            Sugar{name: String::from("DME - All Varieties"), ratio: 1.0/0.68},
            Sugar{name: String::from("DME - Laaglander"), ratio: 1.0/0.5},
            Sugar{name: String::from("Turbinado"), ratio: 1.0},
            Sugar{name: String::from("Demarara"), ratio: 1.0},
            Sugar{name: String::from("Corn Syrup"), ratio: 1.0/0.69},
            Sugar{name: String::from("Brown Sugar"), ratio: 1.0/0.89},
            Sugar{name: String::from("Molasses"), ratio: 1.0/0.71},
            Sugar{name: String::from("Maple Syrup"), ratio: 1.0/0.77},
            Sugar{name: String::from("Sorghum Syrup"), ratio: 1.0/0.69},
            Sugar{name: String::from("Honey"), ratio: 1.0/0.74},
            Sugar{name: String::from("Belgian Candy Syrup"), ratio: 1.0/0.63},
            Sugar{name: String::from("Belgian Candy Sugar"), ratio: 1.0/0.75},
            Sugar{name: String::from("Invert Sugar Syrup"), ratio: 1.0/0.91},
            Sugar{name: String::from("Black Treacle"), ratio: 1.0/0.87},
            Sugar{name: String::from("Rice Solids"), ratio: 1.0/0.79},
        ];

        let beer_co2 = self.calculate_co2(fahrenheit);
        let sucrose = ((co2_volumes*2.0)-(beer_co2*2.0))*2.0*amount;
        Box::new( stream::unfold(sugars.into_iter(), move |mut iter| {
            match iter.next() {
                Some(sugar_base) => Some(future::ok((Sugar{name: sugar_base.name, ratio: sugar_base.ratio * sucrose}, iter))),
                None => None
            }
        }))
    }

}

struct Sugar {
    name: String,
    ratio: f32
}