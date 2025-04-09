use crate::candle::Candle;

const LEVEL_TOLERANCE_PERCENT: f64 = 2.0;
const NEAR_RETEST_PERIOD: usize = 10;

pub fn has_near_retest(candles: &[impl Candle], level: f64) -> bool {
    if candles.len() < 3 {
        return false;
    }

    //let mut candles = candles.to_vec();
    //candles.sort_by(|a, b| a.get_time_key().cmp(&b.get_time_key()));

    let mut bumps_count = 0;

    for (index, candle) in candles.iter().enumerate() {
        let bump_dir = bumped_into_level(candle, level, LEVEL_TOLERANCE_PERCENT);
        
        if bump_dir.is_some() {
            bumps_count += 1;
        }

        if index == 0 && bump_dir.is_none() {
            return false;
        }

        if index == candles.len() - 2 && bump_dir.is_some() {
            // candle before last candle also bumped into level so we are near level
            return false;
        }

        if index > NEAR_RETEST_PERIOD {
            return false;
        }
    }

    bumps_count >= 2
}

fn bumped_into_level(
    candle: &impl Candle,
    level: f64,
    tolerance_percent: f64,
) -> Option<BumpDirection> {
    let tolerance_percent = tolerance_percent / 100.0;
    let tolerance_lower = level * (1.0 - tolerance_percent);
    let tolerance_upper = level + (level * tolerance_percent);

    if in_range(candle.get_high(), tolerance_lower, tolerance_upper) {
        return Some(BumpDirection::FromBelow);
    }

    if in_range(candle.get_low(), tolerance_lower, tolerance_upper) {
        return Some(BumpDirection::FromAbove);
    }

    None
}

fn in_range(value: f64, lower_bound: f64, upper_bound: f64) -> bool {
    value >= lower_bound && value <= upper_bound
}

pub enum BumpDirection {
    FromBelow,
    FromAbove,
}

#[cfg(test)]
mod tests {
    use crate::candle::CandleInstance;

    #[test]
    fn has_near_retest_1() {
        let candles = vec![
            CandleInstance {
                time_key: 0,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
            CandleInstance {
                time_key: 01,
                high: 6.0,
                open: 4.0,
                close: 5.0,
                low: 3.0,
            },
            CandleInstance {
                time_key: 02,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
            CandleInstance {
                time_key: 03,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
        ];

        let result = super::has_near_retest(&candles, 7.0);

        assert!(!result);
    }
}
