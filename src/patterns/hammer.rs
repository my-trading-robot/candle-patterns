use super::Pattern;
use crate::analyzer::{PatternResult, PatternType, SignalDirection};
use crate::candle::Candle;
use std::collections::BTreeMap;

pub struct Hammer {
    pub levels: Vec<f64>,
}

impl<TCandle: Candle> Pattern<TCandle> for Hammer {
    fn matches(&self, candles: &BTreeMap<u64, TCandle>, level: f64) -> Option<PatternResult> {
        let (_, last) = candles.iter().last()?;
        let body = (last.get_open() - last.get_close()).abs();
        let lower_wick = last.get_open().min(last.get_close()) - last.get_low();
        let upper_wick = last.get_high() - last.get_open().max(last.get_close());
        let is_hammer = body < lower_wick && upper_wick < body;

        if !is_hammer {
            return None;
        }

        let mut description = "Hammer detected".to_string();
        if (last.get_low()..=last.get_high()).contains(&level) {
            description = format!("Hammer near level {:.2}", level);
        }

        Some(PatternResult {
            name: "Hammer".to_string(),
            direction: SignalDirection::Bullish,
            description,
            confidence: None,
            pattern_type: PatternType::Hammer,
        })
    }
}
