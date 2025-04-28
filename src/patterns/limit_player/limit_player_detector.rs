use std::collections::BTreeMap;

use crate::candle::Candle;

const LPD_TOLERANCE_PERCENT: f64 = 0.2;
const LPD_MIN_DEPTH: usize = 3;

#[derive(Debug, Clone)]
pub struct LimitPlayerDetectorPattern {
    pub tolerance_percent: f64,
    pub min_depth: usize,
}
pub struct LimitPlayerSignal {
    pub level: f64,               
    pub side: LimitPlayerSide, 
}

#[derive(Debug, PartialEq)]
pub enum LimitPlayerSide {
    Buyer,  // Limit player on lows
    Seller, // Limit player on highs
}

impl Default for LimitPlayerDetectorPattern {
    fn default() -> Self {
        Self {
            tolerance_percent: LPD_TOLERANCE_PERCENT,
            min_depth: LPD_MIN_DEPTH,
        }
    }
}

impl LimitPlayerDetectorPattern {
    pub fn detect<T: Candle>(&self, candles: &BTreeMap<u64, T>) -> Option<LimitPlayerSignal> {
        let candle_vec: Vec<&T> = candles.values().rev().collect();

        if candle_vec.len() < self.min_depth {
            return None;
        }

        for window in candle_vec.windows(self.min_depth) {
            // average high and low
            let avg_high = window.iter().map(|c| c.get_high()).sum::<f64>() / window.len() as f64;
            let avg_low = window.iter().map(|c| c.get_low()).sum::<f64>() / window.len() as f64;

            // check highs near avg_high
            let highs_near = window.iter().all(|c| {
                let distance = (c.get_high() - avg_high).abs() / avg_high;
                distance <= self.tolerance_percent / 100.0
            });

            // check lows near avg_low
            let lows_near = window.iter().all(|c| {
                let distance = (c.get_low() - avg_low).abs() / avg_low;
                distance <= self.tolerance_percent / 100.0
            });

            // check mixed candle directions
            let up_count = window.iter().filter(|c| c.get_close() > c.get_open()).count();
            let down_count = window.iter().filter(|c| c.get_close() < c.get_open()).count();

            if highs_near && up_count > 0 && down_count > 0 {
                return Some(LimitPlayerSignal {
                    level: avg_high,
                    side: LimitPlayerSide::Seller,
                });
            } else if lows_near && up_count > 0 && down_count > 0 {
                return Some(LimitPlayerSignal {
                    level: avg_low,
                    side: LimitPlayerSide::Buyer,
                });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::candle::CandleInstance;

    use super::*;


    #[test]
    fn detects_limit_seller() {
        let candles = vec![
            CandleInstance { time_key: 1, open: 100.0, close: 99.0, high: 105.0, low: 98.0, volume: 1.0, },
            CandleInstance { time_key: 2, open: 99.0, close: 100.5, high: 104.8, low: 97.5, volume: 1.0, },
            CandleInstance { time_key: 3, open: 101.0, close: 100.0, high: 105.1, low: 99.0, volume: 1.0, },
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitPlayerDetectorPattern::default();
        let result = detector.detect(&map);

        assert!(result.is_some());
        let signal = result.unwrap();
        assert_eq!(signal.side, LimitPlayerSide::Seller);
        println!("Detected Seller at {:.2}", signal.level);
    }

    #[test]
    fn detects_limit_buyer() {
        let candles = vec![
            CandleInstance { time_key: 1, open: 95.0, close: 96.5, high: 96.5, low: 90.0, volume: 1.0, },
            CandleInstance { time_key: 2, open: 96.0, close: 95.0, high: 95.0, low: 90.2, volume: 1.0, },
            CandleInstance { time_key: 3, open: 94.5, close: 96.0, high: 96.8, low: 90.1, volume: 1.0, },
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitPlayerDetectorPattern::default();
        let result = detector.detect(&map);

        assert!(result.is_some());
        let signal = result.unwrap();
        assert_eq!(signal.side, LimitPlayerSide::Buyer);
        println!("Detected Buyer at {:.2}", signal.level);
    }

    #[test]
    fn no_detection_when_not_enough_clustering() {
        let candles = vec![
            CandleInstance { time_key: 1, open: 100.0, close: 99.0, high: 105.0, low: 98.0, volume: 1.0,  },
            CandleInstance { time_key: 2, open: 99.0, close: 100.5, high: 106.8, low: 97.5, volume: 1.0,  }, // too far
            CandleInstance { time_key: 3, open: 101.0, close: 100.0, high: 108.1, low: 99.0, volume: 1.0,  }, // too far
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitPlayerDetectorPattern::default();
        let result = detector.detect(&map);

        assert!(result.is_none());
    }
}