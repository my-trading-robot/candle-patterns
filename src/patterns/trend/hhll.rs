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

        let highs: Vec<f64> = candles.iter().map(|c| c.get_high()).collect();
        let lows: Vec<f64> = candles.iter().map(|c| c.get_low()).collect();

        let mut higher_highs = 0;
        let mut lower_lows = 0;

        for i in 1..candles.len() {
            if highs[i] > highs[i - 1] {
                higher_highs += 1;
            }
            if lows[i] < lows[i - 1] {
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
    fn detects_uptrend_with_percent() {
        let candles = vec![
            CandleInstance { time_key: 1, open: 1.0, high: 2.0, low: 0.5, close: 1.5 },
            CandleInstance { time_key: 2, open: 1.5, high: 2.1, low: 0.6, close: 1.6 },
            CandleInstance { time_key: 3, open: 1.6, high: 2.3, low: 0.7, close: 1.7 },
            CandleInstance { time_key: 4, open: 1.7, high: 2.4, low: 0.8, close: 1.8 },
        ];

        let detector = HHLLTrendDetector {
            min_confirmation_ratio: 0.5,
        };

        let trend = detector.detect_trend(&candles);
        assert_eq!(trend, Some(TrendDirection::Up));
    }
}
