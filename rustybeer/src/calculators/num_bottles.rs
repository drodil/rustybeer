// A function to store the different bottle types.
// Can be extended as needed
fn bottles() -> Vec<(String, f64)> {
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
/// * 'volume' - A volume to bottle in milliliters
///
pub fn calculate_num_bottles(volume: f64) -> Vec<(String, i32)> {
    let bottle_types = bottles();
    let mut bottle_counter: Vec<(String, i32)> = Vec::with_capacity(bottle_types.len());

    for bottle in bottle_types {
        let num_bottles: i32 = ((volume) / bottle.1).ceil() as i32;
        bottle_counter.push((bottle.0, num_bottles));
    }
    bottle_counter
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
        assert_eq!(expected, calculate_num_bottles(330.0));
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
        assert_eq!(expected, calculate_num_bottles(330000.0));
    }
}
