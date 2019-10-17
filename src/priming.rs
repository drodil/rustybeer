use clap::{ArgMatches};

fn calculate_co2(temp: i32) -> f32 {
    (3.0378 - (temp as f32 * 0.050062) + (temp.pow(2) as f32 * 0.00026555)
}

pub fn do_matches<'a>(matches: ArgMatches<'a>){
    match matches.value_of("temp").parse::<i32>().unwrap() {
        Ok(m) => { calculate_co2(m)}
        Err(f) => { panic!(f.to_string()) }
    };
    
}