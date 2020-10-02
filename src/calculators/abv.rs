pub struct Abv;

impl Abv {
    pub fn calculate_abv(&self, og: f32, fg: f32) -> f32 {
        (og - fg) * 131.25
    }
}
