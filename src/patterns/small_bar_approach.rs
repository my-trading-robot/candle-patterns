use super::Pattern;
use crate::analyzer::{PatternResult, SignalDirection};
use crate::candle::Candle;

pub struct SmallBarApproach {
    pub levels: Vec<f64>,
    pub period: usize,
    pub tuning_factor: f64,
    pub direction: Option<SignalDirection>, // None = auto
}

impl Pattern for SmallBarApproach {
    fn name(&self) -> &str {
        "Small Bar Approach"
    }

    fn matches(&self, candles: &[Candle]) -> Option<PatternResult> {
        if candles.len() < self.period {
            return None;
        }
    
        let window = &candles[candles.len() - self.period..];
    
        let avg_body_ratio = compute_avg_body_ratio(window);
        let threshold = (avg_body_ratio * self.tuning_factor).max(0.25);
        // println!("AVG: {:.4}, THRESHOLD: {:.4}", avg_body_ratio, threshold);
    
        if !window.iter().all(|c| is_small_bar(c, threshold)) {
            return None;
        }
    
        let direction = match &self.direction {
            Some(d) => d.clone(),
            None => auto_detect_direction(window)?,
        };

        let last = window.last()?;
    
        for &level in &self.levels {
            let near = match direction {
                SignalDirection::Bullish => is_near_bullish_level(last, level),
                SignalDirection::Bearish => is_near_bearish_level(last, level),
                SignalDirection::Neutral => false,
            };
                
            if near {
                return Some(PatternResult {
                    name: self.name().to_string(),
                    direction: direction.clone(),
                    description: format!(
                        "Small bar approach toward level {:.2} ({:?})",
                        level, direction
                    ),
                    //TODO: Calc automatically
                    confidence: Some(0.8),
                });
            }
        }
    
        None
    }
}

fn is_small_bar(c: &Candle, threshold: f64) -> bool {
    let body = (c.open - c.close).abs();
    let range = c.high - c.low;
    range > 0.0 && (body / range) <= threshold + f64::EPSILON
}

fn compute_avg_body_ratio(candles: &[Candle]) -> f64 {
    candles
        .iter()
        .map(|c| {
            let body = (c.open - c.close).abs();
            let range = c.high - c.low;
            if range == 0.0 { 1.0 } else { body / range }
        })
        .sum::<f64>()
        / candles.len() as f64
}

fn auto_detect_direction(candles: &[Candle]) -> Option<SignalDirection> {
    let first = candles.first()?.close;
    let last = candles.last()?.close;
    Some(if last > first {
        SignalDirection::Bullish
    } else if last < first {
        SignalDirection::Bearish
    } else {
        SignalDirection::Neutral
    })
}

fn is_near_bullish_level(c: &Candle, level: f64) -> bool {
    c.close < level && c.high >= level
}

fn is_near_bearish_level(c: &Candle, level: f64) -> bool {
    c.close > level && c.low <= level
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::SignalDirection;
    use crate::candle::Candle;

    #[test]
    fn test_small_bar_approach_up_to_level_bullish() {
        let candles = vec![
            Candle {
                timestamp: 1,
                open: 98.0,
                high: 99.2,
                low: 97.6,
                close: 98.3, // body = 0.3, range = 1.6 → 0.1875
            },
            Candle {
                timestamp: 2,
                open: 98.4,
                high: 99.3,
                low: 98.1,
                close: 98.7, // body = 0.3, range = 1.2 → 0.25
            },
            Candle {
                timestamp: 3,
                open: 99.0,
                high: 100.2,
                low: 98.8,
                close: 99.3, // body = 0.3, range = 1.4 → 0.2142
            },
        ];

        let pattern = SmallBarApproach {
            levels: vec![100.0],
            period: 3,
            tuning_factor: 1.0,
            direction: None,
        };

        let result = pattern.matches(&candles);
        assert!(
            result.is_some(),
            "Expected bullish approach to level from below"
        );
        assert_eq!(result.unwrap().direction, SignalDirection::Bullish);
    }

    #[test]
    fn test_small_bar_approach_down_to_level_bearish() {
        let candles = vec![
            Candle {
                timestamp: 1,
                open: 102.0,
                high: 103.0,
                low: 101.5,
                close: 102.3,
            },
            Candle {
                timestamp: 2,
                open: 102.4,
                high: 102.8,
                low: 101.8,
                close: 102.2,
            },
            Candle {
                timestamp: 3,
                open: 102.0,
                high: 102.2,
                low: 100.5,
                close: 101.7,
            },
        ];

        let pattern = SmallBarApproach {
            levels: vec![101.0],
            period: 3,
            tuning_factor: 1.0,
            direction: None,
        };

        let result = pattern.matches(&candles);
        assert!(
            result.is_some(),
            "Expected bearish approach to level from above"
        );
        assert_eq!(result.unwrap().direction, SignalDirection::Bearish);
    }
}
