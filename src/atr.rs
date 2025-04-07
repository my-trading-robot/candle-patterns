#[derive(Debug, Clone, Copy)]
pub struct Atr(f64);

impl Atr {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn get_value(&self) -> f64 {
        self.0
    }

    pub fn as_ref(&self) -> f64 {
        self.0
    }
}

impl Into<Atr> for f64 {
    fn into(self) -> Atr {
        Atr(self)
    }
}
