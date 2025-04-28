use super::Pattern;
use crate::analyzer::{PatternResult, PatternType, SignalDirection};
use crate::candle::Candle;
use std::collections::BTreeMap;

pub struct SmallBarApproach {
    pub period: usize,
    pub tuning_factor: f64,
    pub direction: Option<SignalDirection>, // None = auto
}

impl<TCandle: Candle> Pattern<TCandle> for SmallBarApproach {
    fn matches(&self, candles: &BTreeMap<u64, TCandle>, level: f64) -> Option<PatternResult> {
        if candles.len() < self.period {
            return None;
        }

        let candles: Vec<&TCandle> = candles.values().collect();
        let window = &candles[candles.len() - self.period..];

        let avg_body_ratio = compute_avg_body_ratio(window);
        let threshold = (avg_body_ratio * self.tuning_factor).max(0.25);
        // println!("AVG: {:.4}, THRESHOLD: {:.4}", avg_body_ratio, threshold);

        if !window.iter().all(|c| is_small_bar(*c, threshold)) {
            return None;
        }

        let direction = match &self.direction {
            Some(d) => d.clone(),
            None => auto_detect_direction(window)?,
        };

        let last = *window.last()?;

        let near = match direction {
            SignalDirection::Bullish => is_near_bullish_level(last, level),
            SignalDirection::Bearish => is_near_bearish_level(last, level),
            SignalDirection::Neutral => false,
        };

        if near {
            return Some(PatternResult {
                name: "Small Bar Approach".to_string(),
                direction: direction.clone(),
                description: format!(
                    "Small bar approach toward level {:.2} ({:?})",
                    level, direction
                ),
                //TODO: Calc automatically
                confidence: Some(0.8),
                pattern_type: PatternType::SmallBarApproach,
            });
        }

        None
    }
}

fn is_small_bar(c: &impl Candle, threshold: f64) -> bool {
    let body = (c.get_open() - c.get_close()).abs();
    let range = c.get_high() - c.get_low();
    range > 0.0 && (body / range) <= threshold + f64::EPSILON
}

fn compute_avg_body_ratio(candles: &[&impl Candle]) -> f64 {
    candles
        .iter()
        .map(|c| {
            let body = (c.get_open() - c.get_close()).abs();
            let range = c.get_high() - c.get_low();
            if range == 0.0 { 1.0 } else { body / range }
        })
        .sum::<f64>()
        / candles.len() as f64
}

fn auto_detect_direction(candles: &[&impl Candle]) -> Option<SignalDirection> {
    let first = candles.first()?.get_close();
    let last = candles.last()?.get_close();
    Some(if last > first {
        SignalDirection::Bullish
    } else if last < first {
        SignalDirection::Bearish
    } else {
        SignalDirection::Neutral
    })
}

fn is_near_bullish_level(c: &impl Candle, level: f64) -> bool {
    c.get_close() < level && c.get_high() >= level
}

fn is_near_bearish_level(c: &impl Candle, level: f64) -> bool {
    c.get_close() > level && c.get_low() <= level
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::SignalDirection;
    use crate::candle::CandleInstance;

    #[test]
    fn test_small_bar_approach_up_to_level_bullish() {
        let candles = vec![
            CandleInstance {
                time_key: 1,
                open: 98.0,
                high: 99.2,
                low: 97.6,
                close: 98.3, // body = 0.3, range = 1.6 → 0.1875
                volume: 1.0,
            },
            CandleInstance {
                time_key: 2,
                open: 98.4,
                high: 99.3,
                low: 98.1,
                close: 98.7, // body = 0.3, range = 1.2 → 0.25
                volume: 1.0,
            },
            CandleInstance {
                time_key: 3,
                open: 99.0,
                high: 100.2,
                low: 98.8,
                close: 99.3, // body = 0.3, range = 1.4 → 0.2142
                volume: 1.0,
            },
        ];

        let pattern = SmallBarApproach {
            period: 3,
            tuning_factor: 1.0,
            direction: None,
        };
        let candles: BTreeMap<u64, CandleInstance> =
            candles.into_iter().map(|c| (c.time_key, c)).collect();

        let result = pattern.matches(&candles, 100.0);
        assert!(
            result.is_some(),
            "Expected bullish approach to level from below"
        );
        assert_eq!(result.unwrap().direction, SignalDirection::Bullish);
    }

    #[test]
    fn test_small_bar_approach_down_to_level_bearish() {
        let candles = vec![
            CandleInstance {
                time_key: 1,
                open: 102.0,
                high: 103.0,
                low: 101.5,
                close: 102.3,
                volume: 1.0,
            },
            CandleInstance {
                time_key: 2,
                open: 102.4,
                high: 102.8,
                low: 101.8,
                close: 102.2,
                volume: 1.0,
            },
            CandleInstance {
                time_key: 3,
                open: 102.0,
                high: 102.2,
                low: 100.5,
                close: 101.7,
                volume: 1.0,
            },
        ];

        let pattern = SmallBarApproach {
            period: 3,
            tuning_factor: 1.0,
            direction: None,
        };
        let candles: BTreeMap<u64, CandleInstance> =
            candles.into_iter().map(|c| (c.time_key, c)).collect();

        let result = pattern.matches(&candles, 101.0);
        assert!(
            result.is_some(),
            "Expected bearish approach to level from above"
        );
        assert_eq!(result.unwrap().direction, SignalDirection::Bearish);
    }
}
