use crate::analyzer::{PatternResult, PatternType, SignalDirection};
use crate::candle::Candle;
use crate::get_bounds;
use crate::patterns::{Pattern};
use std::collections::BTreeMap;

const TOLERANCE_PERCENT: f64 = 2.0;
const PERIOD: usize = 6;

#[derive(Debug, Clone)]
pub struct PressureBuildupPattern {
    pub tolerance_percent: f64,
    pub period: usize,
}

impl<TCandle: Candle> Pattern<TCandle> for PressureBuildupPattern {
    fn matches(&self, candles: &BTreeMap<u64, TCandle>, level: f64) -> Option<PatternResult> {
        let (lower_bound, upper_bound) = get_bounds(level, self.tolerance_percent);
        let last_candles: Vec<_> = candles.values().rev().take(PERIOD).collect();

        if last_candles.len() < PERIOD {
            return None; // Not enough candles
        }

        let mut is_from_bottom = false;

        for (i, candle) in last_candles.iter().enumerate() {
            let prev_is_from_bottom = is_from_bottom;
            is_from_bottom = candle.get_high() <= level;

            if i > 0 && prev_is_from_bottom != is_from_bottom {
                return None;
            }

            let price = if is_from_bottom {
                candle.get_high()
            } else {
                candle.get_low()
            };

            if price > upper_bound || price < lower_bound {
                return None; // Candle outside tolerance
            }
        }

        let pattern_type = PatternType::PressureBuildup;

        Some(PatternResult {
            name: format!("{:?}", pattern_type),
            direction: if is_from_bottom {
                SignalDirection::Bearish
            } else {
                SignalDirection::Bullish
            },
            description: "".to_string(),
            confidence: None,
            pattern_type,
        })
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
