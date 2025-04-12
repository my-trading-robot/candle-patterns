use std::collections::BTreeMap;
use crate::candle::Candle;
use crate::patterns::Pattern;

#[derive(Debug, Clone)]
pub enum PatternType {
    Retest,
    PressureBuildup,
    AtrSpike,
    Hammer,
    SmallBarApproach,
}

#[derive(Debug, Clone)]
pub struct PatternResult {
    pub name: String,
    pub direction: SignalDirection,
    pub description: String,

    /// Represents the strength or reliability of a pattern match (0.0 to 1.0).
    ///
    /// - Signal filtering: Ignore weak signals (e.g. `confidence < 0.6`)
    /// - Signal ranking: Sort matches by strength to prioritize
    /// - Visualization: Stronger signals = brighter or more intense display
    /// - Backtesting weight: Higher confidence = higher expected edge
    ///
    /// `None` means confidence is not applicable or not calculated for that pattern.
    pub confidence: Option<f64>,
    pub pattern_type: PatternType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SignalDirection {
    Bullish,
    Bearish,
    Neutral,
}

pub struct CandleAnalyzer<TCandle: Candle> {
    patterns: Vec<Box<dyn Pattern<TCandle>>>,
}

impl<TCandle: Candle> CandleAnalyzer<TCandle> {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn register_pattern<P: Pattern<TCandle> + 'static>(&mut self, pattern: P) {
        self.patterns.push(Box::new(pattern));
    }

    pub fn analyze(&self, candles: &BTreeMap<u64, TCandle>) -> Vec<PatternResult> {
        self.patterns
            .iter()
            .filter_map(|p| p.matches(candles))
            .collect()
    }
}
