pub struct Diluting;

impl Diluting {
    pub fn calculate_dilution(
        &self,
        current_gravity: f32,
        current_volume: f32,
        target_volume: f32,
    ) -> f32 {
        ((current_gravity - 1.0) * (current_volume / target_volume)) + 1.0
    }
}
