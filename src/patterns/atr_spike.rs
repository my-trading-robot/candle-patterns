use std::collections::BTreeMap;
use super::Pattern;
use crate::analyzer::{PatternResult, SignalDirection};
use crate::candle::Candle;

pub struct AtrSpike {
    pub period: usize,
    pub multiplier: f64,
    /// Precomputed ATR value, optionally calculated from a filtered set of candles (e.g. within bounds)
    pub atr: Option<f64>,
}

impl AtrSpike {
    pub fn calc_candle_atr(candles: &BTreeMap<u64, impl Candle>, period: usize) -> Option<f64> {
        if candles.len() < period {
            return None;
        }

        let atr = candles
            .iter()
            .rev()
            .take(period)
            .map(|(_, c)| c.get_high() - c.get_low())
            .sum::<f64>()
            / period as f64;

        Some(atr)
    }
}

impl<TCandle: Candle> Pattern<TCandle> for AtrSpike {
    fn matches(&self, candles: &BTreeMap<u64, TCandle>) -> Option<PatternResult> {
        let atr = match self.atr {
            Some(val) => val,
            None => AtrSpike::calc_candle_atr(candles, self.period)?,
        };
        let (_, last) = candles.iter().last()?;
        let range = last.get_high() - last.get_low();
        let threshold = self.multiplier * atr;

        if range <= threshold {
            return None;
        }

        let confidence = ((range / atr) - self.multiplier).clamp(0.0, 1.0);

        Some(PatternResult {
            name: "ATR Spike".to_string(),
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
    use crate::candle::CandleInstance;

    fn make_candle(high: f64, low: f64) -> CandleInstance {
        CandleInstance {
            time_key: 0,
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
        let candles: BTreeMap<u64, CandleInstance> =
            candles.into_iter().enumerate().map(|(i, c)| (c.time_key + i as u64, c)).collect();
        
        let atr = AtrSpike::calc_candle_atr(&candles, 3);
        assert_eq!(atr, Some(10.0));
    }

    #[test]
    fn test_atr_calc_not_enough_data() {
        let candles = vec![make_candle(110.0, 100.0), make_candle(120.0, 110.0)];
        let candles: BTreeMap<u64, CandleInstance> =
            candles.into_iter().map(|c| (c.time_key, c)).collect();
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
        let candles: BTreeMap<u64, CandleInstance> =
            candles.into_iter().enumerate().map(|(i, c)| (c.time_key + i as u64, c)).collect();
        let atr = AtrSpike::calc_candle_atr(&candles, 3);
        assert_eq!(atr, Some(0.0));
    }
}
