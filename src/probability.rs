#[derive(Debug, Copy, Clone)]
pub struct Probability(f64);

impl Probability {
    pub fn build(value: f64) -> Option<Self> {
        if !(0.0..=1.0).contains(&value) {
            return None;
        }

        Some(Probability(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
