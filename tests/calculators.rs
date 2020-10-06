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
fn sg_correction() {
    use rustybeer::calculators::sg_correction::SgCorrection;

    let sg_correction = SgCorrection;

    assert_eq!(5.00096332765874, sg_correction.correct_sg(5.0, 2.9, 1.37));

    assert_eq!(7.3023498553759225, sg_correction.correct_sg(7.3, 8.1, 5.12));

    assert_eq!(
        7.417526019059315,
        sg_correction.correct_sg(7.413, 28.1, 55.1212)
    );
}
