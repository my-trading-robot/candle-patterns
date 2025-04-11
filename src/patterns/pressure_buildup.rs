use std::collections::BTreeMap;
use crate::candle::Candle;
use crate::get_bounds;

const TOLERANCE_PERCENT: f64 = 10.0;
const PERIOD: usize = 10;

#[derive(Debug, Clone)]
pub struct PressureBuildupPattern {
    pub tolerance_percent: f64,
    pub period: usize,
}

impl PressureBuildupPattern {
    pub fn matches(&self, candles: &BTreeMap<u64, impl Candle>, level: f64) -> bool {
        let (lower_bound, upper_bound) = get_bounds(level, self.tolerance_percent);
        let last_candles: Vec<_> = candles.values().rev().take(PERIOD).collect();

        if last_candles.len() < PERIOD {
            return false; // Not enough candles
        }

        for candle in &last_candles {
            if candle.get_high() > upper_bound || candle.get_low() < lower_bound {
                return false; // Candle outside tolerance
            }
        }

        true
    }
}

impl Default for PressureBuildupPattern {
    fn default() -> Self {
        Self {
            tolerance_percent: TOLERANCE_PERCENT,
            period: PERIOD,
        }
    }
}
