pub mod analyzer;
pub mod candle;
mod how_candle_crosses_level;
pub mod patterns;
pub use how_candle_crosses_level::*;
mod instrument_types;
pub mod stop_loss;
pub use instrument_types::*;
mod atr;
pub use atr::*;
mod dt_utils;
pub use dt_utils::*;

pub fn in_range(value: f64, lower_bound: f64, upper_bound: f64) -> bool {
    value >= lower_bound && value <= upper_bound
}

pub fn get_bounds(value: f64, tolerance_percent: f64) -> (f64, f64) {
    let tolerance = value * (tolerance_percent / 100.0);
    let lower_bound = value - tolerance;
    let upper_bound = value + tolerance;
    
    (lower_bound, upper_bound)
}