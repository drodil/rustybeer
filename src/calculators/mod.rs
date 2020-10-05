pub mod abv;
pub mod diluting;
pub mod ibu;
pub mod num_bottles;
pub mod priming;
pub mod sg_correction;

/*pub use abv::{calculate_abv, calculate_fg};
pub use diluting::{calculate_new_gravity, calculate_new_volume};
pub use num_bottles::calculate_num_bottles;
pub use priming::{calculate_co2, calculate_sugars};
pub use sg_correction::correct_sg;*/

#[cfg(test)]
mod utilization_test_vector;
