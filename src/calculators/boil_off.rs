pub struct BoilOff;

impl BoilOff {
    pub fn calculate_boileoff_new_volume(&self, wv: f32, cg: f32, dg: f32) -> f32 {
        ((cg - 1.0) / (dg - 1.0)) * wv
    }

    pub fn calculate_boileoff_new_gravity(&self, wv: f32, cg: f32, tv: f32) -> f32 {
        (cg - 1.0) * (wv / tv) + 1.0
    }
}
