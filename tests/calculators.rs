#[test]
fn abv() {
    use rustybeer::calculators::abv::Abv;

    let abv = Abv;

    assert_eq!(1050., abv.calculate_abv(10., 2.));

    assert_eq!(39.5548125, abv.calculate_abv(0.3026, 0.00123))
}

#[test]
fn boil_off() {
    use rustybeer::calculators::boil_off::BoilOff;

    let boil_off = BoilOff;

    assert_eq!(2., boil_off.calculate_boileoff_new_volume(2., 2., 2.));

    assert_eq!(14., boil_off.calculate_boileoff_new_volume(7., 5., 3.));

    assert_eq!(
        10.333333,
        boil_off.calculate_boileoff_new_gravity(7., 5., 3.)
    );

    assert_eq!(
        63.85714285714286,
        boil_off.calculate_boileoff_new_gravity(4., 3.2, 0.14)
    );
}

#[test]
fn diluting() {
    use rustybeer::calculators::diluting::Diluting;

    let diluting = Diluting;

    assert_eq!(14.162499, diluting.calculate_dilution(9.1, 5.2, 3.2));

    assert_eq!(4.5304832, diluting.calculate_dilution(9.1, 3.16, 7.25));
}

#[test]
fn priming() {
    use rustybeer::calculators::priming::{Priming, Sugar};

    let priming = Priming;

    assert_eq!(59., priming.celsius_to_fahrenheit(15.));

    assert_eq!(53.877197, priming.celsius_to_fahrenheit(12.154));

    assert_eq!(2.34661875, priming.calculate_co2(15.));

    assert_eq!(2.455689014, priming.calculate_co2(12.45));

    let stream = priming.calculate_sugars(77., 5., 2.);

    let expected = vec![
        Sugar::new(String::from("Table Sugar (sucrose)"), 24.850557),
        Sugar::new(String::from("Corn Sugar (dextrose)"), 27.308304),
        Sugar::new(String::from("DME - All Varieties"), 36.544937),
        Sugar::new(String::from("DME - Laaglander"), 49.701115),
        Sugar::new(String::from("Turbinado"), 24.850557),
        Sugar::new(String::from("Demarara"), 24.850557),
        Sugar::new(String::from("Corn Syrup"), 36.0153),
        Sugar::new(String::from("Brown Sugar"), 27.921974),
        Sugar::new(String::from("Molasses"), 35.000786),
        Sugar::new(String::from("Maple Syrup"), 32.27345),
        Sugar::new(String::from("Sorghum Syrup"), 36.0153),
        Sugar::new(String::from("Honey"), 33.581837),
        Sugar::new(String::from("Belgian Candy Syrup"), 39.44533),
        Sugar::new(String::from("Belgian Candy Sugar"), 33.13408),
        Sugar::new(String::from("Invert Sugar Syrup"), 27.308304),
        Sugar::new(String::from("Black Treacle"), 28.563858),
        Sugar::new(String::from("Rice Solids"), 31.456402),
    ];

    assert_eq!(expected, stream);
}

#[test]
fn sg_correction() {}

#[test]
fn temperature() {
    use rustybeer::calculators::sg_correction::Temperature;

    let temp_type = Temperature::Celsius;
    let temp_in_celsius = 25.;

    assert_eq!(25., temp_type.to_celsius(temp_in_celsius));
    assert_eq!(77., temp_type.to_fahrenheit(temp_in_celsius));
    assert_eq!(298.15, temp_type.to_kelvin(temp_in_celsius));

    let temp_type = Temperature::Fahrenheit;
    let temp_in_farenheight = 60.;

    assert_eq!(15.555556, temp_type.to_celsius(temp_in_farenheight));
    assert_eq!(60., temp_type.to_fahrenheit(temp_in_farenheight));
    assert_eq!(288.70554, temp_type.to_kelvin(temp_in_farenheight));

    let temp_type = Temperature::Kelvin;
    let temp_in_kelvin = 400.;

    assert_eq!(126.850006, temp_type.to_celsius(temp_in_kelvin));
    assert_eq!(260.33002, temp_type.to_fahrenheit(temp_in_kelvin));
    assert_eq!(400., temp_type.to_kelvin(temp_in_kelvin));
}
