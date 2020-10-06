use crate::utils::conversions::VolumeBuilder; // Converts string input to unit measurements
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
    /// Prints out the quantity of bottles needed to store a given volume
    ///
    /// # Arguments
    /// * 'volume' - A string of the volume to hold, with units appended e.g "10mL", "5.4gal", "3.99L" etc.
    ///
    pub fn calculate_num_bottles(&self, volume: String) -> Vec<(String, i32)> {
        let bottle_types = self.bottles();
        let mut bottle_counter: Vec<(String, i32)> = Vec::with_capacity(bottle_types.len());
        let volume = VolumeBuilder::from_str(volume).unwrap().as_milliliters() as f32;

        for bottle in bottle_types {
            let num_bottles: i32 = ((volume) / bottle.1).ceil() as i32;
            bottle_counter.push((bottle.0, num_bottles));
        }
        bottle_counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_milliliters() {
        let expected = vec![
            ("330ml bottle".to_string(), 1),
            ("Twelve ounce bottle".to_string(), 1),
            ("Groish bottle".to_string(), 1),
            ("Half liter bottle".to_string(), 1),
            ("Pint".to_string(), 1),
            ("Graft bottle".to_string(), 1),
            ("Wine bottle".to_string(), 1),
            ("Grenade jug".to_string(), 1),
            ("Growler jug".to_string(), 1),
            ("Gallon jug".to_string(), 1),
            ("5 liter mini keg".to_string(), 1),
        ];
        assert_eq!(
            expected,
            NumBottles.calculate_num_bottles("330:ml".to_string())
        );
    }

    #[test]
    fn test_liters() {
        let expected = vec![
            ("330ml bottle".to_string(), 1000),
            ("Twelve ounce bottle".to_string(), 930),
            ("Groish bottle".to_string(), 734),
            ("Half liter bottle".to_string(), 660),
            ("Pint".to_string(), 581),
            ("Graft bottle".to_string(), 508),
            ("Wine bottle".to_string(), 440),
            ("Grenade jug".to_string(), 349),
            ("Growler jug".to_string(), 175),
            ("Gallon jug".to_string(), 88),
            ("5 liter mini keg".to_string(), 66),
        ];
        assert_eq!(
            expected,
            NumBottles.calculate_num_bottles("330:l".to_string())
        );
    }
}
