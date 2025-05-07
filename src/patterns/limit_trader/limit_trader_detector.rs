use std::collections::BTreeMap;

use crate::{candle::Candle, round_to_precision};

pub const LTD_DEFAULT_TOLERANCE: u32 = 2;
pub const LTD_MIN_WINDOW_SIZE: usize = 3;

#[derive(Debug, Clone)]
pub struct LimitTraderDetectorPattern {
    pub accuracy: u32,
    pub points_tolerance: u32,
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

impl LimitTraderSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            LimitTraderSide::Buyer => "Buyer",
            LimitTraderSide::Seller => "Seller",
        }
    }
}

impl LimitTraderDetectorPattern {
    pub fn new(accuracy: u32, tolerance: u32, window_size: usize) -> Self {
        Self { accuracy, points_tolerance: tolerance, window_size }
    }

    pub fn calc_points_tolerance(&self) -> f64 {
        crate::math::calc_points_tolerance(self.accuracy, self.points_tolerance)
    }

    pub fn detect<T: Candle>(&self, candles: &BTreeMap<u64, T>) -> Option<LimitTraderSignal> {
        let candle_vec: Vec<&T> = candles.values().rev().collect();

        if candle_vec.len() < self.window_size {
            return None;
        }

        for window in candle_vec.windows(self.window_size) {
            
            let avg_high = round_to_precision(
                window.iter().map(|c| c.get_high()).sum::<f64>() / window.len() as f64,
                self.accuracy,
            );

            let avg_low = round_to_precision(
                window.iter().map(|c| c.get_low()).sum::<f64>() / window.len() as f64,
                self.accuracy,
            );

            // let max_high = window.iter().map(|c| c.get_high()).fold(f64::MIN, f64::max);
            // let min_high = window.iter().map(|c| c.get_high()).fold(f64::MAX, f64::min);
            // let max_low = window.iter().map(|c| c.get_low()).fold(f64::MIN, f64::max);
            // let min_low = window.iter().map(|c| c.get_low()).fold(f64::MAX, f64::min);

            let date_time= window.first().unwrap().get_time_key();
            // check highs near avg_high
            let highs_near = window.iter().all(|c| {
                let distance = (c.get_high() - avg_high).abs();
                distance <= self.calc_points_tolerance()
            });

            // check lows near avg_low
            let lows_near = window.iter().all(|c| {
                let distance = (c.get_low() - avg_low).abs();
                distance <= self.calc_points_tolerance()
            });

            // check mixed candle directions
            let up_count = window.iter().filter(|c| c.get_close() > c.get_open()).count();
            let down_count = window.iter().filter(|c| c.get_close() < c.get_open()).count();

            if highs_near && up_count > 0 && down_count > 0 {
                return Some(LimitTraderSignal {
                    level: avg_high,
                    date_time_key: date_time,
                    side: LimitTraderSide::Seller,
                });
            } else if lows_near && up_count > 0 && down_count > 0 {
                return Some(LimitTraderSignal {
                    level: avg_low,
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
        let signal_level = 105.09;
        let accuracy = 2;
        let candles = vec![
            CandleInstance { time_key: 1, open: 100.0, close: 99.0, high: 105.0, low: 98.0, volume: 1.0, },  // down
            CandleInstance { time_key: 2, open: 99.0, close: 100.5, high: 105.09, low: 97.5, volume: 1.0, },  // up   +
            CandleInstance { time_key: 3, open: 101.0, close: 100.0, high: 105.10, low: 99.0, volume: 1.0, }, // down +
            CandleInstance { time_key: 4, open: 101.0, close: 100.0, high: 105.08, low: 99.0, volume: 1.0, },// down +
            CandleInstance { time_key: 5, open: 101.0, close: 100.0, high: 105.75, low: 99.0, volume: 1.0, },// down
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitTraderDetectorPattern::new(
            accuracy, 
            LTD_DEFAULT_TOLERANCE, 
            LTD_MIN_WINDOW_SIZE, 
        );
        let result = detector.detect(&map);

        assert!(result.is_some());
        let signal = result.unwrap();
        assert_eq!(signal.side, LimitTraderSide::Seller);
        println!("Detected {} at {signal:#?} at level: {signal_level:.digits$}", 
            signal.side.as_str(), 
            digits = accuracy as usize
        );
        assert!((signal.level - signal_level).abs() <= f64::EPSILON);
    }

    #[test]
    fn detects_limit_buyer() {
        let signal_level = 90.05;
        let accuracy = 2;
        let candles = vec![
            CandleInstance { time_key: 1, open: 95.0, close: 96.5, high: 96.5, low: 90.0, volume: 1.0, },  // up
            CandleInstance { time_key: 2, open: 96.0, close: 95.0, high: 94.8, low: 90.04, volume: 1.0, },  // down +
            CandleInstance { time_key: 3, open: 94.5, close: 96.0, high: 96.8, low: 90.06, volume: 1.0, },  // up   +
            CandleInstance { time_key: 4, open: 94.5, close: 96.0, high: 99.8, low: 90.05, volume: 1.0, }, // up   +
            CandleInstance { time_key: 5, open: 94.5, close: 96.0, high: 96.8, low: 90.75, volume: 1.0, }, // up
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitTraderDetectorPattern::new(
            accuracy, 
            LTD_DEFAULT_TOLERANCE, 
            LTD_MIN_WINDOW_SIZE, 
        );
        let result = detector.detect(&map);

        assert!(result.is_some());
        let signal = result.unwrap();
        assert_eq!(signal.side, LimitTraderSide::Buyer);
        println!("Detected {} at {signal:#?} at level: {signal_level:.digits$}", 
            signal.side.as_str(), 
            digits = accuracy as usize
        );
        assert!((signal.level - signal_level).abs() <= f64::EPSILON);
    }


    #[test]
    fn no_detection_when_not_enough_window_size() {
        let accuracy = 1;
        let candles = vec![
            CandleInstance { time_key: 1, open: 100.0, close: 99.0, high: 102.5, low: 92.0, volume: 1.0,  },  // down
            CandleInstance { time_key: 2, open: 99.0, close: 100.5, high: 106.8, low: 97.5, volume: 1.0,  },  // up
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitTraderDetectorPattern::new(
            accuracy, 
            LTD_DEFAULT_TOLERANCE, 
            LTD_MIN_WINDOW_SIZE, 
        );
        let result = detector.detect(&map);

        assert!(result.is_none());
    }


    #[test]
    fn no_detection_when_tolerance_is_to_low() {
        let accuracy = 1;
        let candles = vec![
            CandleInstance { time_key: 1, open: 100.0, close: 99.0, high: 102.5, low: 92.0, volume: 1.0,  },  // down
            CandleInstance { time_key: 2, open: 99.0, close: 100.5, high: 106.8, low: 97.5, volume: 1.0,  },  // up
            CandleInstance { time_key: 3, open: 101.0, close: 100.0, high: 108.1, low: 100.0, volume: 1.0,  }, // down
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitTraderDetectorPattern::new(
            accuracy, 
            LTD_DEFAULT_TOLERANCE, 
            LTD_MIN_WINDOW_SIZE, 
        );
        let result = detector.detect(&map);

        assert!(result.is_none());
    }

    #[test]
    fn detects_limit_seller_ai() {
        let signal_level = 22.5999;
        let accuracy = 4;
        let candles = vec![
            CandleInstance { time_key: 5, open: 22.4, close: 22.45, high: 22.5999, low: 22.345, volume: 676159.0, },     // up +
            CandleInstance { time_key: 4, open: 22.48, close: 22.405, high: 22.5998, low: 22.4, volume: 261406.0, },     // down +
            CandleInstance { time_key: 3, open: 22.5751, close: 22.49, high: 22.6001, low: 22.4659, volume: 239247.0, }, // down +
            CandleInstance { time_key: 2, open: 22.52, close: 22.5785, high: 22.705, low: 22.5, volume: 308007.0, },     // up
            CandleInstance { time_key: 1, open: 22.5193, close: 22.52, high: 22.62, low: 22.3705, volume: 271893.0, },   // up
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitTraderDetectorPattern::new(
            accuracy, 
            LTD_DEFAULT_TOLERANCE, 
            LTD_MIN_WINDOW_SIZE, 
        );
        let result = detector.detect(&map);

        assert!(result.is_some());
        let signal = result.unwrap();
        assert_eq!(signal.side, LimitTraderSide::Seller);
        println!("Detected {} at {signal:#?} at level: {signal_level:.digits$}", 
            signal.side.as_str(), 
            digits = accuracy as usize
        );
        assert!((signal.level - signal_level).abs() <= f64::EPSILON);
    }

    #[test]
    fn detects_limit_seller_aal() {
        let signal_level = 10.5899;
        let accuracy = 4;
        let candles = vec![
            CandleInstance { time_key: 2025050115, open: 10.095, close: 10.0393, high: 10.18, low: 10.03, volume: 11184939.0 },
            CandleInstance { time_key: 2025050116, open: 10.0319, close: 10.0691, high: 10.085, low: 9.98, volume: 7048979.0 },
            CandleInstance { time_key: 2025050117, open: 10.065, close: 10.04, high: 10.1, low: 10.04, volume: 4477227.0 },
            CandleInstance { time_key: 2025050118, open: 10.0491, close: 10.055, high: 10.1, low: 10.02, volume: 4371536.0 },
            CandleInstance { time_key: 2025050119, open: 10.055, close: 10.03, high: 10.13, low: 10.01, volume: 8752285.0 }, //
            CandleInstance { time_key: 2025050213, open: 10.25, close: 10.365, high: 10.5899, low: 10.25, volume: 8460939.0 }, // up +
            CandleInstance { time_key: 2025050214, open: 10.36, close: 10.5049, high: 10.5899, low: 10.3552, volume: 13083697.0 }, // up +
            CandleInstance { time_key: 2025050215, open: 10.5111, close: 10.51, high: 10.5900, low: 10.48, volume: 15149711.0 }, // down +
            CandleInstance { time_key: 2025050216, open: 10.51, close: 10.61, high: 10.6900, low: 10.505, volume: 13946636.0 }, // up
            CandleInstance { time_key: 2025050217, open: 10.605, close: 10.5499, high: 10.6194, low: 10.52, volume: 5957688.0 }, // down
            CandleInstance { time_key: 2025050218, open: 10.55, close: 10.495, high: 10.6, low: 10.49, volume: 4828634.0 }, // down
            CandleInstance { time_key: 2025050219, open: 10.49, close: 10.53, high: 10.5493, low: 10.48, volume: 9949184.0 }, // up
        ];

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle);
        }

        let detector = LimitTraderDetectorPattern::new(
            accuracy, 
            LTD_DEFAULT_TOLERANCE, 
            LTD_MIN_WINDOW_SIZE, 
        );
        let result = detector.detect(&map);

        assert!(result.is_some());
        let signal = result.unwrap();
        assert_eq!(signal.side, LimitTraderSide::Seller);
        println!("Detected {} at {signal:#?} at level: {signal_level:.digits$}", 
            signal.side.as_str(), 
            digits = accuracy as usize
        );
        assert!((signal.level - signal_level).abs() <= f64::EPSILON);
    }
    
}