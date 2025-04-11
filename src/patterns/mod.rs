mod atr_spike;
mod hammer;
pub mod level_bounce;
mod small_bar_approach;
mod trend;
mod retest;
mod pressure_buildup;

pub use atr_spike::AtrSpike;
pub use hammer::Hammer;
pub use small_bar_approach::SmallBarApproach;
pub use trend::*;
pub use retest::*;
pub use pressure_buildup::*;

use crate::analyzer::PatternResult;
use crate::candle::*;

pub trait Pattern<TCandle: Candle> {
    fn matches(&self, candles: &[TCandle]) -> Option<PatternResult>;
}
