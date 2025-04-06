mod hammer;
mod atr_spike;
mod small_bar_approach;

pub use hammer::Hammer;
pub use atr_spike::AtrSpike;
pub use small_bar_approach::SmallBarApproach;

use crate::candle::Candle;
use crate::analyzer::PatternResult;

pub trait Pattern {
    fn name(&self) -> &str;
    fn matches(&self, candles: &[Candle]) -> Option<PatternResult>;
}
