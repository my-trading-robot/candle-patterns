# 📊 candle_patterns

**A Rust library for detecting candlestick chart patterns in historical price data.**  
Built for speed, flexibility, and integration with custom indicators like ATR, support/resistance levels, and filtered candle sets.

---

## ✨ Features

- 🔍 Detects classic candlestick patterns (e.g. Hammer, ATR Spike)
- 🧱 Modular pattern system via traits
- 📏 Support for precomputed or dynamic levels (support/resistance)
- 🔧 Extensible with your own custom indicators
- 📈 Confidence scoring per signal (0.0–1.0)
- 💾 Lightweight data model

---

## 📦 Usage

```rust
use candle_patterns::candle::Candle;
use candle_patterns::analyzer::CandleAnalyzer;
use candle_patterns::patterns::{Hammer, AtrSpike};

fn main() {
    let candles = vec![
        Candle { timestamp: 1, open: 100.0, high: 105.0, low: 95.0, close: 101.0 },
        // ... more candles
    ];

    let mut analyzer = CandleAnalyzer::new();

    analyzer.register_pattern(Hammer {
        levels: vec![100.0, 105.0],
    });

    analyzer.register_pattern(AtrSpike {
        period: 14,
        multiplier: 1.5,
        atr: None, // or Some(precomputed_value)
    });

    let results = analyzer.analyze(&candles);

    for result in results {
        println!(
            "[{}] {} — Direction: {:?}, Confidence: {:.2}",
            result.name,
            result.description,
            result.direction,
            result.confidence.unwrap_or(0.0)
        );
    }
}
