use super::TechStopLoss;

pub struct Luft(f64);

impl Luft {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn to_value(&self) -> f64 {
        self.0
    }

    pub fn as_ref(&self) -> f64 {
        self.0
    }
}

impl From<TechStopLoss> for Luft {
    fn from(value: TechStopLoss) -> Self {
        Self::new(value.get_value() * 0.2)
    }
}
