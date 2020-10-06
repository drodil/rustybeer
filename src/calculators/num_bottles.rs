/// A struct used to print how many bottles are needed to contain a given volume (in mL or L)
pub struct NumBottles;

impl NumBottles {
    // A function to store the different bottle types.
    // Can be extended as needed
    fn bottles(&self) -> Vec<(String, f32)> {
        vec![
            ("330ml bottle".to_string(), 330.0),
            ("Twelve ounce bottle".to_string(), 355.0),
            ("Groish bottle".to_string(), 450.0),
            ("Half liter bottle".to_string(), 500.0),
            ("Pint".to_string(), 568.0),
            ("Graft bottle".to_string(), 650.0),
            ("Wine bottle".to_string(), 750.0),
            ("Grenade jug".to_string(), 946.0),
            ("Growler jug".to_string(), 1893.0),
            ("Gallon jug".to_string(), 3785.0),
            ("5 liter mini keg".to_string(), 5000.0),
        ]
    }
    /// Prints out the quantity of bottles needed to contain a volume given in mL
    ///
    /// # Arguments
    /// * 'volume' - A f32 in milliliters of the volume to hold
    ///
    pub fn bottles_milliliters(&self, volume: f32) {
        println!("VOLUME GIVEN: {} mL\n=============", volume);
        for bottle in self.bottles() {
            let num_bottles: f32 = volume / bottle.1;
            let output = format!(
                "Type: {0: <20} | Number: {1: <5} |",
                bottle.0,
                num_bottles.ceil()
            );
            println!("{}", output);
        }
    }
    /// Prints out the quantity of bottles needed to contain a volume given in L
    ///
    /// # Arguments
    /// * 'volume' - A f32 in liters of the volume to hold
    ///
    pub fn bottles_liters(&self, volume: f32) {
        println!("\nVOLUME GIVEN: {} L\n=============", volume);
        for bottle in self.bottles() {
            let num_bottles: f32 = (volume * 1000.0) / bottle.1;
            let output = format!(
                "Type: {0: <20} | Number: {1: <5} |",
                bottle.0,
                num_bottles.ceil()
            );
            println!("{}", output);
        }
    }
}
