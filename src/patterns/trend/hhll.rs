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
    pub fn detect_trend<T: Candle>(&self, candles: &[T]) -> Option<TrendDirection> {
        if candles.len() < 2 {
            return None;
        }
        let mut higher_highs = 0;
        let mut lower_lows = 0;

        for i in 1..candles.len() {
            let curr = &candles[i - 1];
            let prev = &candles[i];

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

    #[test]
    fn detects_uptrend_with_confirmation_ratio() {
        let candles = vec![
            CandleInstance { time_key: 4, open: 1.0, high: 2.0, low: 0.5, close: 1.5 },
            CandleInstance { time_key: 3, open: 1.5, high: 2.1, low: 0.6, close: 1.6 },
            CandleInstance { time_key: 2, open: 1.6, high: 2.3, low: 0.7, close: 1.7 },
            CandleInstance { time_key: 1, open: 1.7, high: 2.4, low: 0.8, close: 1.8 },
        ];

        let detector = HHLLTrendDetector {
            min_confirmation_ratio: 1.0,
        };

        let trend = detector.detect_trend(&candles);
        assert_eq!(trend, Some(TrendDirection::Down));
    }

    #[test]
    fn detects_uptrend_with_diff_count_and_ration() {
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

        // 1
        let detector = HHLLTrendDetector {
            min_confirmation_ratio: 0.8,
        };

        let trend = detector.detect_trend(&candles[..10]);
        assert_eq!(trend, Some(TrendDirection::Down));

        // 2
        let detector = HHLLTrendDetector {
            min_confirmation_ratio: 0.6,
        };
        let trend = detector.detect_trend(&candles[..15]);
        assert_eq!(trend, Some(TrendDirection::Down));

        // 3
        let detector = HHLLTrendDetector {
            min_confirmation_ratio: 0.5,
        };
        let trend = detector.detect_trend(&candles[..20]);
        assert_eq!(trend, Some(TrendDirection::Down));
    }

   
}
