use crate::candle::Candle;
use crate::analyzer::{PatternResult, SignalDirection};
use super::Pattern;

pub struct AtrSpike {
    pub period: usize,
    pub multiplier: f64,
    
    /// Precomputed ATR value, optionally calculated from a filtered set of candles (e.g. within bounds)
    pub atr: Option<f64>, 
}

impl AtrSpike {
    pub fn calc_candle_atr(candles: &[Candle], period: usize) -> Option<f64> {
        if candles.len() < period {
            return None;
        }

        let atr = candles
            .iter()
            .rev()
            .take(period)
            .map(|c| c.high - c.low)
            .sum::<f64>()
            / period as f64;

        Some(atr)
    }
}

impl Pattern for AtrSpike {
    fn name(&self) -> &str {
        "ATR Spike"
    }

    fn matches(&self, candles: &[Candle]) -> Option<PatternResult> {
        let atr = match self.atr {
            Some(val) => val,
            None => AtrSpike::calc_candle_atr(candles, self.period)?,
        };
        let last = candles.last()?;
        let range = last.high - last.low;
        let threshold = self.multiplier * atr;

        if range <= threshold {
            return None;
        }

        let confidence = ((range / atr) - self.multiplier).clamp(0.0, 1.0);

        Some(PatternResult {
            name: self.name().to_string(),
            direction: SignalDirection::Neutral,
            description: format!(
                "Volatility spike: range {:.2} > {:.2}×ATR ({:.2})",
                range, self.multiplier, atr
            ),
            confidence: Some(confidence),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::candle::Candle;

    fn make_candle(high: f64, low: f64) -> Candle {
        Candle {
            timestamp: 0,
            open: 0.0,
            close: 0.0,
            high,
            low,
        }
    }

    #[test]
    fn test_atr_calc_correct_average() {
        let candles = vec![
            make_candle(110.0, 100.0), 
            make_candle(120.0, 110.0), 
            make_candle(130.0, 120.0), 
        ];

        let atr = AtrSpike::calc_candle_atr(&candles, 3);
        assert_eq!(atr, Some(10.0));
    }

    #[test]
    fn test_atr_calc_not_enough_data() {
        let candles = vec![
            make_candle(110.0, 100.0),
            make_candle(120.0, 110.0),
        ];

        let atr = AtrSpike::calc_candle_atr(&candles, 3);
        assert_eq!(atr, None);
    }

    #[test]
    fn test_atr_calc_edge_case_flat_range() {
        let candles = vec![
            make_candle(100.0, 100.0),
            make_candle(100.0, 100.0),
            make_candle(100.0, 100.0),
        ];

        let atr = AtrSpike::calc_candle_atr(&candles, 3);
        assert_eq!(atr, Some(0.0));
    }
}