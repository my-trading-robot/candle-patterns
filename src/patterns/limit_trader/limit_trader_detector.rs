use std::collections::BTreeMap;

use crate::candle::Candle;

const LTD_TOLERANCE_PERCENT: f64 = 0.2;
const LTD_MIN_DEPTH: usize = 3;

#[derive(Debug, Clone)]
pub struct LimitTraderDetectorPattern {
    pub tolerance_percent: f64,
    pub window_size: usize,
}

#[derive(Debug, Clone)]
pub struct LimitTraderSignal {
    pub level: f64,  
    pub date_time_key: u64,             
    pub side: LimitTraderSide, 
}

#[derive(Debug, PartialEq, Clone)]
pub enum LimitTraderSide {
    Buyer,  // Limit Trader on lows
    Seller, // Limit Trader on highs
}

impl Default for LimitTraderDetectorPattern {
    fn default() -> Self {
        Self {
            tolerance_percent: LTD_TOLERANCE_PERCENT,
            window_size: LTD_MIN_DEPTH,
        }
    }
}

impl LimitTraderDetectorPattern {
    pub fn detect<T: Candle>(&self, candles: &BTreeMap<u64, T>) -> Option<LimitTraderSignal> {
        let candle_vec: Vec<&T> = candles.values().rev().collect();

        if candle_vec.len() < self.window_size {
            return None;
        }

        for window in candle_vec.windows(self.window_size) {
            // average high and low
            let avg_high = window.iter().map(|c| c.get_high()).sum::<f64>() / window.len() as f64;
            let avg_low = window.iter().map(|c| c.get_low()).sum::<f64>() / window.len() as f64;
            let date_time= window.first().unwrap().get_time_key();
            let high= window.first().unwrap().get_high();
            let low= window.first().unwrap().get_low();

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
                return Some(LimitTraderSignal {
                    level: high,
                    date_time_key: date_time,
                    side: LimitTraderSide::Seller,
                });
            } else if lows_near && up_count > 0 && down_count > 0 {
                return Some(LimitTraderSignal {
                    level: low,
                    date_time_key: date_time,
                    side: LimitTraderSide::Buyer,
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
            CandleInstance { time_key: 1, open: 100.0, close: 99.0, high: 105.0, low: 98.0, volume: 1.0, },  // down
            CandleInstance { time_key: 2, open: 99.0, close: 100.5, high: 104.8, low: 97.5, volume: 1.0, },  // up   +
            CandleInstance { time_key: 3, open: 101.0, close: 100.0, high: 105.1, low: 99.0, volume: 1.0, }, // down +
            CandleInstance { time_key: 4, open: 101.0, close: 100.0, high: 105.05, low: 99.0, volume: 1.0, },// down +
            CandleInstance { time_key: 5, open: 101.0, close: 100.0, high: 105.75, low: 99.0, volume: 1.0, },// down
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitTraderDetectorPattern::default();
        let result = detector.detect(&map);

        assert!(result.is_some());
        let signal = result.unwrap();
        assert_eq!(signal.side, LimitTraderSide::Seller);
        assert_eq!(signal.level, 105.05);
        println!("Detected Seller at {:.2}", signal.level);
    }

    #[test]
    fn detects_limit_buyer() {
        let candles = vec![
            CandleInstance { time_key: 1, open: 95.0, close: 96.5, high: 96.5, low: 90.0, volume: 1.0, },  // up
            CandleInstance { time_key: 2, open: 96.0, close: 95.0, high: 95.0, low: 90.2, volume: 1.0, },  // down +
            CandleInstance { time_key: 3, open: 94.5, close: 96.0, high: 96.8, low: 90.1, volume: 1.0, },  // up   +
            CandleInstance { time_key: 4, open: 94.5, close: 96.0, high: 96.8, low: 90.05, volume: 1.0, }, // up   +
            CandleInstance { time_key: 5, open: 94.5, close: 96.0, high: 96.8, low: 90.75, volume: 1.0, }, // up
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitTraderDetectorPattern::default();
        let result = detector.detect(&map);

        assert!(result.is_some());
        let signal = result.unwrap();
        assert_eq!(signal.side, LimitTraderSide::Buyer);
        assert_eq!(signal.level, 90.05);
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

        let detector = LimitTraderDetectorPattern::default();
        let result = detector.detect(&map);

        assert!(result.is_none());
    }
}