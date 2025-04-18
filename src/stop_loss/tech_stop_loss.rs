use crate::Atr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TechStopLoss(f64);

impl TechStopLoss {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn from_crypto_and_us_stock_day_atr(day_atr: Atr) -> Self {
        Self(day_atr.get_value() * 0.15)
    }

    pub fn get_value(&self) -> f64 {
        self.0
    }

    pub fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl Into<TechStopLoss> for f64 {
    fn into(self) -> TechStopLoss {
        TechStopLoss(self)
    }
}
