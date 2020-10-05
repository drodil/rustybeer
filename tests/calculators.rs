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
    // use futures::stream::Stream;
    use futures::StreamExt;

    use rustybeer::calculators::priming::Priming;

    // use rustybeer::calculators::priming::Sugar;

    let priming = Priming;

    assert_eq!(59., priming.celsius_to_fahrenheit(15.));

    assert_eq!(53.877197, priming.celsius_to_fahrenheit(12.154));

    assert_eq!(2.34661875, priming.calculate_co2(15.));

    assert_eq!(2.455689014, priming.calculate_co2(12.45));

    println!("Assigning stream");

    let stream = priming.calculate_sugars(77., 5., 2.);

    println!("Stream assigned");

    stream.for_each(move |sugar| {
        println!("{:>23}: {:.2} g", sugar.name, sugar.ratio);
        futures::future::ready(())
    });
}
