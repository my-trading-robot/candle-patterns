use crate::analyzer::{PatternResult, PatternType, SignalDirection};
use crate::candle::Candle;
use crate::get_bounds;
use crate::patterns::Pattern;
use std::collections::BTreeMap;

const TOLERANCE_PERCENT: f64 = 2.0;
const PERIOD: usize = 4;

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

        let mut under_level = false;

        for (i, candle) in last_candles.iter().enumerate() {
            let prev_under_level = under_level;
            under_level = candle.get_high() <= upper_bound;

            if i > 0 && prev_under_level != under_level {
                return None;
            }

            let price = if under_level {
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
            direction: if under_level {
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

#[cfg(test)]
mod tests {
    use crate::candle::CandleInstance;
    use crate::patterns::{Pattern, PressureBuildupPattern};
    use std::collections::BTreeMap;

    #[test]
    fn test_1() {
        let candles = vec![
            CandleInstance {
                time_key: 0,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
                volume: 1.0,
            },
            CandleInstance {
                time_key: 1,
                high: 7.0,
                open: 4.0,
                close: 5.0,
                low: 3.0,
                volume: 1.0,
            },
            CandleInstance {
                time_key: 2,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
                volume: 1.0,
            },
            CandleInstance {
                time_key: 3,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
                volume: 1.0,
            },
        ];

        let candles: BTreeMap<u64, CandleInstance> =
            candles.into_iter().map(|c| (c.time_key, c)).collect();

        let pattern = PressureBuildupPattern::default();
        let result = pattern.matches(&candles, 7.0);

        assert!(result.is_some());
    }

    #[test]
    fn test_2() {
        let candles = vec![
            CandleInstance {
                time_key: 0,
                high: 6.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
                volume: 1.0,
            },
            CandleInstance {
                time_key: 1,
                high: 7.0,
                open: 4.0,
                close: 5.0,
                low: 3.0,
                volume: 1.0,
            },
            CandleInstance {
                time_key: 2,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
                volume: 1.0,
            },
            CandleInstance {
                time_key: 3,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
                volume: 1.0,
            },
        ];

        let candles: BTreeMap<u64, CandleInstance> =
            candles.into_iter().map(|c| (c.time_key, c)).collect();

        let pattern = PressureBuildupPattern::default();
        let result = pattern.matches(&candles, 7.0);

        assert!(result.is_none());
    }
}
