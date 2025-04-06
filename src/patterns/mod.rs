mod hammer;
mod atr_spike;

pub use hammer::Hammer;
pub use atr_spike::AtrSpike;

use crate::candle::Candle;
use crate::analyzer::PatternResult;

pub trait Pattern {
    fn name(&self) -> &str;
    fn matches(&self, candles: &[Candle]) -> Option<PatternResult>;
}
