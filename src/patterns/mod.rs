mod atr_spike;
mod hammer;
pub mod level_bounce;
mod small_bar_approach;

pub use atr_spike::AtrSpike;
pub use hammer::Hammer;
pub use small_bar_approach::SmallBarApproach;

use crate::analyzer::PatternResult;
use crate::candle::*;

pub trait Pattern<TCandle: Candle> {
    fn matches(&self, candles: &[TCandle]) -> Option<PatternResult>;
}
