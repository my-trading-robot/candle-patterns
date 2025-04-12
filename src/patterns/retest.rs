use crate::analyzer::{PatternResult, PatternType, SignalDirection};
use crate::candle::Candle;
use crate::in_range;
use crate::patterns::{Pattern};
use std::collections::BTreeMap;

const LEVEL_TOLERANCE_PERCENT: f64 = 2.0;
const CLOSE_PERIOD: usize = 10;
const LONG_PERIOD: usize = 30;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub enum RetestPatternType {
    Close,
    Long,
}

#[derive(Debug, Clone)]
pub struct RetestPattern {
    pub tolerance_percent: f64,
    pub close_period: usize,
    pub long_period: usize,
}

impl<TCandle: Candle> Pattern<TCandle> for RetestPattern {
    fn matches(&self, candles: &BTreeMap<u64, TCandle>, level: f64) -> Option<PatternResult> {
        let (direction, pattern_type) = self.get_type(candles, level)?;
        let name = format!("{:?}", pattern_type);

        let direction = match direction {
            BumpDirection::FromBelow => SignalDirection::Bullish,
            BumpDirection::FromAbove => SignalDirection::Bearish,
        };

        let result = match pattern_type {
            RetestPatternType::Close => PatternResult {
                name,
                direction,
                description: "".to_string(),
                confidence: None,
                pattern_type: PatternType::CloseRetest,
            },
            RetestPatternType::Long => PatternResult {
                name,
                direction,
                description: "".to_string(),
                confidence: None,
                pattern_type: PatternType::LongRetest,
            },
        };

        Some(result)
    }
}
impl Default for RetestPattern {
    fn default() -> Self {
        Self {
            tolerance_percent: LEVEL_TOLERANCE_PERCENT,
            close_period: CLOSE_PERIOD,
            long_period: LONG_PERIOD,
        }
    }
}

impl RetestPattern {
    pub fn get_type(
        &self,
        candles: &BTreeMap<u64, impl Candle>,
        level: f64,
    ) -> Option<(BumpDirection, RetestPatternType)> {
        if candles.len() < 3 {
            return None;
        }

        let mut bumps_count = 0;
        let mut result = None;
        let mut bump_dir = None;

        for (index, (_key, candle)) in candles.iter().rev().enumerate() {
            let prev_bump_dir = bump_dir;
            bump_dir = bumped_into_level(candle, level, LEVEL_TOLERANCE_PERCENT);

            if bump_dir.is_some() {
                bumps_count += 1;
            }

            if index == 0 && bump_dir.is_none() {
                return None;
            }

            if prev_bump_dir.is_some() && bump_dir.is_some() && prev_bump_dir != bump_dir {
                return None;
            }

            if index == candles.len() - 2 && bump_dir.is_some() {
                // candle before last candle also bumped into level so we are near level
                return None;
            }

            if index <= CLOSE_PERIOD && bumps_count >= 2 {
                result = Some((bump_dir.unwrap(), RetestPatternType::Close));
                break;
            }

            if index <= LONG_PERIOD && bumps_count >= 2 {
                result = Some((bump_dir.unwrap(), RetestPatternType::Long));
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

#[derive(Debug, Clone, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub enum BumpDirection {
    FromBelow,
    FromAbove,
}

#[cfg(test)]
mod tests {
    use crate::candle::CandleInstance;
    use crate::patterns::{BumpDirection, RetestPattern, RetestPatternType};
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
        let result = pattern.get_type(&candles, 7.0);

        assert_eq!(result, Some((BumpDirection::FromBelow, RetestPatternType::Close)));
    }
}
