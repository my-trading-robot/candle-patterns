use std::collections::BTreeMap;
use super::Pattern;
use crate::analyzer::{PatternResult, SignalDirection};
use crate::candle::Candle;

pub struct Hammer {
    pub levels: Vec<f64>,
}

impl<TCandle: Candle> Pattern<TCandle> for Hammer {
    fn matches(&self, candles: &BTreeMap<u64, TCandle>) -> Option<PatternResult> {
        let (_, last) = candles.iter().last()?;
        let body = (last.get_open() - last.get_close()).abs();
        let lower_wick = last.get_open().min(last.get_close()) - last.get_low();
        let upper_wick = last.get_high() - last.get_open().max(last.get_close());
        let is_hammer = body < lower_wick && upper_wick < body;

        if !is_hammer {
            return None;
        }

        let mut description = "Hammer detected".to_string();
        for level in &self.levels {
            if (last.get_low()..=last.get_high()).contains(level) {
                description = format!("Hammer near level {:.2}", level);
                break;
            }
        }

        Some(PatternResult {
            name: "Hammer".to_string(),
            direction: SignalDirection::Bullish,
            description,
            confidence: None,
        })
    }
}
