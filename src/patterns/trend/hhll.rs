use std::collections::BTreeMap;

use crate::candle::Candle;

#[derive(Debug, PartialEq)]
pub enum TrendDirection {
    Up,
    Down,
    Sideways,
}

pub struct HHLLTrendDetector {
    pub min_confirmation_ratio: f64,
}

impl HHLLTrendDetector {
    pub fn detect_trend<T: Candle>(&self, candles: &BTreeMap<u64, T>) -> Option<TrendDirection> {
        let candle_vec: Vec<&T> = candles.values().rev().collect();

        if candle_vec.len() < 2 {
            return None;
        }

        let mut higher_highs = 0;
        let mut lower_lows = 0;

        for window in candle_vec.windows(2) {
            let curr = window[0];
            let prev = window[1];

            if curr.get_high() > prev.get_high() {
                higher_highs += 1;
            }
            if curr.get_low() < prev.get_low() {
                lower_lows += 1;
            }
        }

        let total_checks = candles.len() - 1;
        let required = (total_checks as f64 * self.min_confirmation_ratio).ceil() as usize;

        if higher_highs >= required {
            Some(TrendDirection::Up)
        } else if lower_lows >= required {
            Some(TrendDirection::Down)
        } else {
            Some(TrendDirection::Sideways)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::candle::CandleInstance;

    use super::*;

    fn run_case(candles: &[CandleInstance], ratio: f64, expected: TrendDirection) {
        let detector = HHLLTrendDetector {
            min_confirmation_ratio: ratio,
        };

        let mut map = BTreeMap::new();
        for candle in candles {
            map.insert(candle.time_key, candle.clone());
        }

        let trend = detector.detect_trend(&map);
        assert_eq!(trend, Some(expected));
    }

    #[test]
    fn detects_uptrend_with_confirmation_ratio() {
        let candles = vec![
            CandleInstance {
                time_key: 4,
                open: 1.0,
                high: 2.0,
                low: 0.5,
                close: 1.5,
            },
            CandleInstance {
                time_key: 3,
                open: 1.5,
                high: 2.1,
                low: 0.6,
                close: 1.6,
            },
            CandleInstance {
                time_key: 2,
                open: 1.6,
                high: 2.3,
                low: 0.7,
                close: 1.7,
            },
            CandleInstance {
                time_key: 1,
                open: 1.7,
                high: 2.4,
                low: 0.8,
                close: 1.8,
            },
        ];

        run_case(&candles, 1.0, TrendDirection::Down);
    }

    #[test]
    fn detects_trend_with_various_candle_counts_and_ratios() {
        let candles = vec![
            CandleInstance { time_key: 20250409, open: 172.1, high: 198.98, low: 168.0, close: 195.86 },
            CandleInstance { time_key: 20250408, open: 183.88, high: 190.335, low: 169.1, close: 169.45 },
            CandleInstance { time_key: 20250407, open: 177.87, high: 194.15, low: 174.43, close: 182.68 },
            CandleInstance { time_key: 20250404, open: 203.11, high: 203.46, low: 186.44, close: 186.6 },
            CandleInstance { time_key: 20250403, open: 210.78, high: 211.03, low: 201.25, close: 203.38 },
            CandleInstance { time_key: 20250402, open: 222.82, high: 225.5, low: 206.15, close: 207.91 },
            CandleInstance { time_key: 20250401, open: 221.42, high: 224.0, low: 218.9, close: 223.45 },
            CandleInstance { time_key: 20250331, open: 215.74, high: 225.62, low: 215.0, close: 221.1999 },
            CandleInstance { time_key: 20250328, open: 222.0, high: 223.85, low: 217.22, close: 217.22 },
            CandleInstance { time_key: 20250327, open: 221.0, high: 224.99, low: 220.5, close: 223.46 },
            CandleInstance { time_key: 20250326, open: 223.94, high: 225.02, low: 220.47, close: 221.53 },
            CandleInstance { time_key: 20250325, open: 220.5, high: 224.33, low: 220.05, close: 224.18 },
            CandleInstance { time_key: 20250324, open: 219.8, high: 221.48, low: 218.58, close: 220.74 },
            CandleInstance { time_key: 20250321, open: 213.9, high: 218.84, low: 209.2, close: 217.8799 },
            CandleInstance { time_key: 20250320, open: 216.24, high: 217.4899, low: 212.22, close: 213.7 },
            CandleInstance { time_key: 20250319, open: 213.58, high: 218.76, low: 212.62, close: 216.15 },
            CandleInstance { time_key: 20250318, open: 213.73, high: 215.15, low: 211.49, close: 213.0 },
            CandleInstance { time_key: 20250317, open: 212.86, high: 215.22, low: 209.97, close: 213.93 },
            CandleInstance { time_key: 20250314, open: 210.72, high: 213.95, low: 209.58, close: 213.29 },
            CandleInstance { time_key: 20250313, open: 216.0, high: 216.98, low: 208.42, close: 210.2 },
        ];
        
        run_case(&candles[..10], 0.8, TrendDirection::Down);
        run_case(&candles[..15], 0.6, TrendDirection::Down);
        run_case(&candles[..20], 0.5, TrendDirection::Down);
    }

    #[test]
    fn detects_trend_with_diff_count_and_ration() {
        // LOAR 1H bid
        let candles = vec![
            CandleInstance {
                time_key: 2025041019,
                open: 84.11,
                high: 86.48,
                low: 83.79,
                close: 85.97,
            },
            CandleInstance {
                time_key: 2025041018,
                open: 83.92,
                high: 85.39,
                low: 83.82,
                close: 84.22,
            },
            CandleInstance {
                time_key: 2025041017,
                open: 83.07,
                high: 84.23,
                low: 81.07,
                close: 84.12,
            },
            CandleInstance {
                time_key: 2025041016,
                open: 82.25,
                high: 83.46,
                low: 81.31,
                close: 83.085,
            },
            CandleInstance {
                time_key: 2025041015,
                open: 85.675,
                high: 85.86,
                low: 81.95,
                close: 82.12,
            },
            CandleInstance {
                time_key: 2025041014,
                open: 83.215,
                high: 87.1,
                low: 82.95,
                close: 85.85,
            },
            CandleInstance {
                time_key: 2025041013,
                open: 83.27,
                high: 83.9897,
                low: 82.31,
                close: 83.22,
            },
            CandleInstance {
                time_key: 2025040919,
                open: 83.22,
                high: 86.25,
                low: 82.77,
                close: 85.36,
            },
            CandleInstance {
                time_key: 2025040918,
                open: 83.89,
                high: 86.2042,
                low: 82.901,
                close: 83.565,
            },
            CandleInstance {
                time_key: 2025040917,
                open: 79.12,
                high: 84.4999,
                low: 78.3,
                close: 83.895,
            },
            CandleInstance {
                time_key: 2025040916,
                open: 77.125,
                high: 79.3,
                low: 77.125,
                close: 79.12,
            },
            CandleInstance {
                time_key: 2025040915,
                open: 76.605,
                high: 77.97,
                low: 76.605,
                close: 77.125,
            },
            CandleInstance {
                time_key: 2025040914,
                open: 76.97,
                high: 79.24,
                low: 76.03,
                close: 76.6,
            },
            CandleInstance {
                time_key: 2025040913,
                open: 74.77,
                high: 76.96,
                low: 74.07,
                close: 76.27,
            },
            CandleInstance {
                time_key: 2025040819,
                open: 75.24,
                high: 75.54,
                low: 73.77,
                close: 75.38,
            },
            CandleInstance {
                time_key: 2025040818,
                open: 75.71,
                high: 76.71,
                low: 75.04,
                close: 75.44,
            },
            CandleInstance {
                time_key: 2025040817,
                open: 77.04,
                high: 77.2599,
                low: 75.245,
                close: 76.03,
            },
            CandleInstance {
                time_key: 2025040816,
                open: 78.52,
                high: 78.52,
                low: 76.89,
                close: 76.89,
            },
            CandleInstance {
                time_key: 2025040815,
                open: 78.15,
                high: 79.35,
                low: 77.37,
                close: 78.63,
            },
            CandleInstance {
                time_key: 2025040814,
                open: 77.3,
                high: 79.24,
                low: 77.14,
                close: 78.15,
            },
        ];

        run_case(&candles[..8], 0.55, TrendDirection::Up);
    }
}
