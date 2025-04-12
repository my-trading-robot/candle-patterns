use crate::candle::Candle;
use crate::get_bounds;
use std::collections::BTreeMap;

const TOLERANCE_PERCENT: f64 = 2.0;
const PERIOD: usize = 6;

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
            let is_from_bottom = candle.get_high() <= level;
            let price = if is_from_bottom {
                candle.get_high()
            } else {
                candle.get_low()
            };

            if price > upper_bound || price < lower_bound {
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
