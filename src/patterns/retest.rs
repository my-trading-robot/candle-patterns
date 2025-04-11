use crate::candle::Candle;
use std::collections::BTreeMap;

const LEVEL_TOLERANCE_PERCENT: f64 = 2.0;
const NEAR_PERIOD: usize = 10;
const FAR_PERIOD: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub enum RetestPatternType {
    Near,
    Far,
}

#[derive(Debug, Clone)]
pub struct RetestPattern {
    pub level_tolerance_percent: f64,
    pub near_period: usize,
    pub far_period: usize,
}

impl Default for RetestPattern {
    fn default() -> Self {
        Self {
            level_tolerance_percent: LEVEL_TOLERANCE_PERCENT,
            near_period: NEAR_PERIOD,
            far_period: FAR_PERIOD,
        }
    }
}

impl RetestPattern {
    pub fn matches(&self, candles: &BTreeMap<u64, impl Candle>, level: f64) -> Option<RetestPatternType> {
        if candles.len() < 3 {
            return None;
        }

        let mut bumps_count = 0;
        let mut result = None;

        for (index, (_key, candle)) in candles.iter().rev().enumerate() {
            let bump_dir = bumped_into_level(candle, level, LEVEL_TOLERANCE_PERCENT);

            if bump_dir.is_some() {
                bumps_count += 1;
            }

            if index == 0 && bump_dir.is_none() {
                return None;
            }

            if index == candles.len() - 2 && bump_dir.is_some() {
                // candle before last candle also bumped into level so we are near level
                return None;
            }

            if index <= NEAR_PERIOD && bumps_count >= 2 {
                result = Some(RetestPatternType::Near);
                break;
            }

            if index <= FAR_PERIOD && bumps_count >= 2 {
                result = Some(RetestPatternType::Near);
                break;
            }
        }

        result
    }
}

fn bumped_into_level(
    candle: &impl Candle,
    level: f64,
    tolerance_percent: f64,
) -> Option<BumpDirection> {
    let tolerance_percent = tolerance_percent / 100.0;
    let tolerance_lower = level * (1.0 - tolerance_percent);
    let tolerance_upper = level + (level * tolerance_percent);

    if in_range(candle.get_high(), tolerance_lower, tolerance_upper) {
        return Some(BumpDirection::FromBelow);
    }

    if in_range(candle.get_low(), tolerance_lower, tolerance_upper) {
        return Some(BumpDirection::FromAbove);
    }

    None
}

fn in_range(value: f64, lower_bound: f64, upper_bound: f64) -> bool {
    value >= lower_bound && value <= upper_bound
}

pub enum BumpDirection {
    FromBelow,
    FromAbove,
}

#[cfg(test)]
mod tests {
    use crate::candle::CandleInstance;
    use crate::patterns::{RetestPattern, RetestPatternType};
    use std::collections::BTreeMap;

    #[test]
    fn near_retest_1() {
        let candles = vec![
            CandleInstance {
                time_key: 0,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
            CandleInstance {
                time_key: 01,
                high: 6.0,
                open: 4.0,
                close: 5.0,
                low: 3.0,
            },
            CandleInstance {
                time_key: 02,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
            CandleInstance {
                time_key: 03,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
        ];

        let candles: BTreeMap<u64, CandleInstance> =
            candles.into_iter().map(|c| (c.time_key, c)).collect();

        let pattern = RetestPattern::default();
        let result = pattern.matches(&candles, 7.0);

        assert_eq!(result, Some(RetestPatternType::Near));
    }
}
