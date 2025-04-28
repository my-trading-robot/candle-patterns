use std::collections::BTreeMap;

use crate::{analyzer::PatternResult, candle::Candle, patterns::Pattern};

const LPD_TOLERANCE_PERCENT: f64 = 0.8;
const LPD_MIN_DEPTH: usize = 3;

#[derive(Debug, Clone)]
pub struct LimitPlayerDetectorPattern {
    pub tolerance_percent: f64,
    pub min_depth: usize,
}
pub struct LimitPlayerSignal {
    pub level: f64,
    pub touches: usize,
}

impl<TCandle: Candle> Pattern<TCandle> for LimitPlayerDetectorPattern {
    fn matches(&self, candles: &BTreeMap<u64, TCandle>, level: f64) -> Option<PatternResult> {
        
        todo!()
    }
}
impl Default for LimitPlayerDetectorPattern {
    fn default() -> Self {
        Self {
            tolerance_percent: LPD_TOLERANCE_PERCENT,
            min_depth: LPD_MIN_DEPTH,
        }
    }
}

impl LimitPlayerDetectorPattern {
    pub fn detect<T: Candle>(&self, candles: &BTreeMap<u64, T>) -> Option<f64> {
        let candle_vec: Vec<&T> = candles.values().rev().collect();

        if candle_vec.len() < LPD_MIN_DEPTH {
            return None;
        }

        todo!()
    }


}