use futures::{future, stream, Stream};

pub struct Priming;

impl Priming {
    pub fn celsius_to_fahrenheit(&self, celsius: f32) -> f32 {
        (9.0 / 5.0) * celsius + 32.0
    }

    pub fn calculate_co2(&self, fahrenheit: f32) -> f32 {
        3.0378 - 0.050062 * fahrenheit + 0.00026555 * fahrenheit.powf(2.0)
    }

    pub fn calculate_sugars(
        &self,
        fahrenheit: f32,
        amount: f32,
        co2_volumes: f32,
    ) -> impl Stream<Item = Sugar> {
        let sugars = vec![
            Sugar::new(String::from("Table Sugar (sucrose)"), 1.0),
            Sugar::new(String::from("Corn Sugar (dextrose)"), 1.0 / 0.91),
            Sugar::new(String::from("DME - All Varieties"), 1.0 / 0.68),
            Sugar::new(String::from("DME - Laaglander"), 1.0 / 0.5),
            Sugar::new(String::from("Turbinado"), 1.0),
            Sugar::new(String::from("Demarara"), 1.0),
            Sugar::new(String::from("Corn Syrup"), 1.0 / 0.69),
            Sugar::new(String::from("Brown Sugar"), 1.0 / 0.89),
            Sugar::new(String::from("Molasses"), 1.0 / 0.71),
            Sugar::new(String::from("Maple Syrup"), 1.0 / 0.77),
            Sugar::new(String::from("Sorghum Syrup"), 1.0 / 0.69),
            Sugar::new(String::from("Honey"), 1.0 / 0.74),
            Sugar::new(String::from("Belgian Candy Syrup"), 1.0 / 0.63),
            Sugar::new(String::from("Belgian Candy Sugar"), 1.0 / 0.75),
            Sugar::new(String::from("Invert Sugar Syrup"), 1.0 / 0.91),
            Sugar::new(String::from("Black Treacle"), 1.0 / 0.87),
            Sugar::new(String::from("Rice Solids"), 1.0 / 0.79),
        ].into_iter();

        let beer_co2 = self.calculate_co2(fahrenheit);
        let sucrose = ((co2_volumes * 2.0) - (beer_co2 * 2.0)) * 2.0 * amount;

        // println!("{:?}", sugars);

        stream::unfold(sugars, move |mut iter| {
            println!("nth sugar: {:?}", iter);
            match iter.next() {
                Some(sugar_base) => {
                    println!("Adding sugar {}", sugar_base.name);
                    future::ready(Some((
                        Sugar::new(sugar_base.name, sugar_base.ratio * sucrose),
                        iter,
                    )))
                }
                None => {
                    println!("None");
                    future::ready(None)
                }
            }
        })
    }
}
#[derive(Debug)]
pub struct Sugar {
    pub name: String,
    pub ratio: f32,
}

impl Sugar {
    fn new(name: String, ratio: f32) -> Self {
        Self { name, ratio }
    }
}
