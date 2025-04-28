mod atr_spike;
mod hammer;
pub mod level_bounce;
mod small_bar_approach;
mod trend;
mod retest;
mod pressure_buildup;

use std::collections::BTreeMap;
pub use atr_spike::AtrSpike;
pub use hammer::Hammer;
pub use small_bar_approach::SmallBarApproach;
pub use trend::*;
pub use retest::*;
pub use pressure_buildup::*;

use crate::analyzer::PatternResult;
use crate::candle::*;

mod limit_trader;
pub use limit_trader::*;

pub trait Pattern<TCandle: Candle> {
    fn matches(&self, candles: &BTreeMap<u64, TCandle>, level: f64) -> Option<PatternResult>;
}
