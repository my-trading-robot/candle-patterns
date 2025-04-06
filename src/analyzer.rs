use crate::candle::Candle;
use crate::patterns::Pattern;

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
    /// - ML / strategy input: Use confidence as a feature in decision logic
    ///
    /// `None` means confidence is not applicable or not calculated for that pattern.
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum SignalDirection {
    Bullish,
    Bearish,
    Neutral,
}

pub struct CandleAnalyzer {
    patterns: Vec<Box<dyn Pattern>>,
}

impl CandleAnalyzer {
    pub fn new() -> Self {
        Self { patterns: Vec::new() }
    }

    pub fn register_pattern<P: Pattern + 'static>(&mut self, pattern: P) {
        self.patterns.push(Box::new(pattern));
    }

    pub fn analyze(&self, candles: &[Candle]) -> Vec<PatternResult> {
        self.patterns
            .iter()
            .filter_map(|p| p.matches(candles))
            .collect()
    }
}
