use std::error::Error;
use futures::{future, Future, stream, Stream};

pub struct Priming;

impl Priming {
    pub fn calculate_co2(&self, fahrenheit: f64) -> f64 {
        (3.0378 - 0.050062 * fahrenheit + 0.00026555 * fahrenheit.powf(2.0))
    }

    pub fn calculate_sugars(&self, fahrenheit: f64, amount: f64, co2_volumes: f64) -> Box<dyn Stream<Item=Sugar, Error=Box<dyn Error>>>
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

pub struct Sugar {
    pub name: String,
    pub ratio: f64
}
