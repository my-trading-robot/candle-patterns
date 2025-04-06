use crate::candle::Candle;
use crate::analyzer::{PatternResult, SignalDirection};
use super::Pattern;

pub struct Hammer {
    pub levels: Vec<f64>,
}

impl Pattern for Hammer {
    fn name(&self) -> &str {
        "Hammer"
    }

    fn matches(&self, candles: &[Candle]) -> Option<PatternResult> {
        let last = candles.last()?;
        let body = (last.open - last.close).abs();
        let lower_wick = last.open.min(last.close) - last.low;
        let upper_wick = last.high - last.open.max(last.close);
        let is_hammer = body < lower_wick && upper_wick < body;

        if !is_hammer {
            return None;
        }

        let mut description = "Hammer detected".to_string();
        for level in &self.levels {
            if (last.low..=last.high).contains(level) {
                description = format!("Hammer near level {:.2}", level);
                break;
            }
        }

        Some(PatternResult {
            name: self.name().to_string(),
            direction: SignalDirection::Bullish,
            description,
            confidence: None,
        })
    }
}
