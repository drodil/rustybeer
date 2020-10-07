#[test]
fn abv() {
    use rustybeer::calculators::abv::Abv;

    let abv = Abv;

    assert!(1050. - abv.calculate_abv(10., 2.) < f32::EPSILON);

    assert!(39.554_813 - abv.calculate_abv(0.3026, 0.00123) < f32::EPSILON)
}

#[test]
fn boil_off() {
    use rustybeer::calculators::boil_off::BoilOff;

    let boil_off = BoilOff;

    assert!(2. - boil_off.calculate_boileoff_new_volume(2., 2., 2.) < f32::EPSILON);

    assert!(14. - boil_off.calculate_boileoff_new_volume(7., 5., 3.) < f32::EPSILON);

    assert!(10.333333 - boil_off.calculate_boileoff_new_gravity(7., 5., 3.) < f32::EPSILON);

    assert!(63.857_143 - boil_off.calculate_boileoff_new_gravity(4., 3.2, 0.14) < f32::EPSILON);
}

#[test]
fn diluting() {
    use rustybeer::calculators::diluting::Diluting;

    let diluting = Diluting;

    assert!(14.162499 - diluting.calculate_new_gravity(9.1, 5.2, 3.2) < f32::EPSILON);

    assert!(4.5304832 - diluting.calculate_new_gravity(9.1, 3.16, 7.25) < f32::EPSILON);

    assert!(2. - diluting.calculate_new_volume(2., 2., 2.) < f32::EPSILON);

    assert!(14. - diluting.calculate_new_volume(7., 5., 3.) < f32::EPSILON);
}

#[test]
fn priming() {
    use rustybeer::calculators::priming::{Priming, Sugar};

    let priming = Priming;

    assert!(2.3466187499999998 - priming.calculate_co2(15.) < f64::EPSILON);

    assert!(2.4556890138750003 - priming.calculate_co2(12.45) < f64::EPSILON);

    let stream = priming.calculate_sugars(77., 5., 2.);

    let expected = vec![
        Sugar::new(String::from("Table Sugar (sucrose)"), 24.850561000000013),
        Sugar::new(String::from("Corn Sugar (dextrose)"), 27.308308791208802),
        Sugar::new(String::from("DME - All Varieties"), 36.54494264705884),
        Sugar::new(String::from("DME - Laaglander"), 49.701122000000026),
        Sugar::new(String::from("Turbinado"), 24.850561000000013),
        Sugar::new(String::from("Demarara"), 24.850561000000013),
        Sugar::new(String::from("Corn Syrup"), 36.015305797101476),
        Sugar::new(String::from("Brown Sugar"), 27.92197865168541),
        Sugar::new(String::from("Molasses"), 35.00079014084509),
        Sugar::new(String::from("Maple Syrup"), 32.27345584415586),
        Sugar::new(String::from("Sorghum Syrup"), 36.015305797101476),
        Sugar::new(String::from("Honey"), 33.5818391891892),
        Sugar::new(String::from("Belgian Candy Syrup"), 39.44533492063494),
        Sugar::new(String::from("Belgian Candy Sugar"), 33.13408133333335),
        Sugar::new(String::from("Invert Sugar Syrup"), 27.308308791208802),
        Sugar::new(String::from("Black Treacle"), 28.563863218390818),
        Sugar::new(String::from("Rice Solids"), 31.45640632911394),
    ];

    assert_eq!(expected, stream);
}

#[test]
fn sg_correction() {
    use rustybeer::calculators::sg_correction::SgCorrection;

    let sg_correction = SgCorrection;

    assert!(5.00096332765874 - sg_correction.correct_sg(5.0, 2.9, 1.37) < f64::EPSILON);

    assert!(7.3023498553759225 - sg_correction.correct_sg(7.3, 8.1, 5.12) < f64::EPSILON);

    assert!(7.417526019059315 - sg_correction.correct_sg(7.413, 28.1, 55.1212) < f64::EPSILON);
}
