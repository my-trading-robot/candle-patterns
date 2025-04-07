use crate::{HowCandleCrossesLevel, candle::Candle, stop_loss::Luft};

pub struct BsuBpiIndex {
    pub bsu_index: usize,
    pub bpu_1_index: usize,
    pub bpu_2_index: usize,
}

// Returns BSU index

pub fn find_bpu_bsu(candles: &[impl Candle], level: f64, luft: Luft) -> Option<BsuBpiIndex> {
    if candles.len() < 3 {
        return None;
    }

    let mut bsu_index = None;

    for (index, candle) in candles.iter().enumerate() {
        let high = candle.get_high();
        let low = candle.get_low();
        if high == level || low == level {
            bsu_index = Some(index);
            break;
        }
    }

    let bsu_index = bsu_index?;

    for i in bsu_index + 1..candles.len() - 1 {
        let bpu_1_to_check = candles.get(i).unwrap();
        let bpu_2_to_check = candles.get(i + 1).unwrap();

        if are_candles_bpu1_and_bpu2(bpu_1_to_check, bpu_2_to_check, level, luft) {
            return Some(BsuBpiIndex {
                bsu_index,
                bpu_1_index: i,
                bpu_2_index: i + 1,
            });
        }
    }

    None
}

fn are_candles_bpu1_and_bpu2(
    bpu_1: &impl Candle,
    bpu_2: &impl Candle,
    level: f64,
    luft: Luft,
) -> bool {
    let bpu_1_touch = HowCandleCrossesLevel::from_candle_and_level(bpu_1, level);
    if !bpu_1_touch.is_candle_touches_the_level() {
        return false;
    }

    let bpu_2_touch = HowCandleCrossesLevel::from_candle_and_level(bpu_2, level);

    let distance = match bpu_2_touch {
        HowCandleCrossesLevel::CandleIsAbove { distance } => {
            if bpu_1_touch.is_below_or_touches_below() {
                return false;
            }
            distance
        }
        HowCandleCrossesLevel::CandleIsBelow { distance } => {
            if bpu_1_touch.is_above_or_touches_above() {
                return false;
            }
            distance
        }
        HowCandleCrossesLevel::CandleTouchesAbove => {
            if bpu_1_touch.is_below_or_touches_below() {
                return false;
            }
            0.0
        }
        HowCandleCrossesLevel::CandleTouchesBelow => {
            if bpu_1_touch.is_above_or_touches_above() {
                return false;
            }
            0.0
        }
        _ => return false,
    };

    distance <= luft.to_value()
}

#[cfg(test)]
mod tests {
    use crate::candle::CandleInstance;

    // Visualizing: https://docs.google.com/spreadsheets/d/1TwL6ZAY_sV8klmcXW8NnwQ0ev4bN6P0-ybIXRed-CRM/edit?gid=889640400#gid=889640400
    #[test]
    fn have_bsu_and_bpu1_and_bpu2_below() {
        //test1
        let candles = vec![
            CandleInstance {
                time_key: 00,
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
                time_key: 00,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
            CandleInstance {
                time_key: 00,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
        ];

        let result = super::find_bpu_bsu(&candles, 7.0, 0.02.into()).unwrap();

        assert_eq!(0, result.bsu_index);
        assert_eq!(2, result.bpu_1_index);
        assert_eq!(3, result.bpu_2_index);
    }

    #[test]
    fn have_bsu_above_and_bpu1_and_bpu2_below() {
        //test2
        let candles = vec![
            CandleInstance {
                time_key: 00,
                high: 10.0,
                open: 8.0,
                close: 9.0,
                low: 7.0,
            },
            CandleInstance {
                time_key: 01,
                high: 6.0,
                open: 4.0,
                close: 5.0,
                low: 3.0,
            },
            CandleInstance {
                time_key: 00,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
            CandleInstance {
                time_key: 00,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
        ];

        let result = super::find_bpu_bsu(&candles, 7.0, 0.02.into()).unwrap();

        assert_eq!(0, result.bsu_index);
        assert_eq!(2, result.bpu_1_index);
        assert_eq!(3, result.bpu_2_index);
    }

    #[test]
    fn have_bsu_above_and_bpu1_below_and_bpu2_above() {
        //test3
        let candles = vec![
            CandleInstance {
                time_key: 00,
                high: 10.0,
                open: 8.0,
                close: 9.0,
                low: 7.0,
            },
            CandleInstance {
                time_key: 01,
                high: 6.0,
                open: 4.0,
                close: 5.0,
                low: 3.0,
            },
            CandleInstance {
                time_key: 00,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
            CandleInstance {
                time_key: 00,
                high: 10.0,
                open: 8.0,
                close: 9.0,
                low: 7.0,
            },
        ];

        let result = super::find_bpu_bsu(&candles, 7.0, 0.02.into());
        assert!(result.is_none());
    }

    #[test]
    fn have_bsu_and_bpu1_and_bpu2_with_luft_below() {
        //test1
        let candles = vec![
            CandleInstance {
                time_key: 00,
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
                time_key: 00,
                high: 6.8,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
        ];

        let result = super::find_bpu_bsu(&candles, 7.0, 0.5.into()).unwrap();

        assert_eq!(0, result.bsu_index);
        assert_eq!(2, result.bpu_1_index);
        assert_eq!(3, result.bpu_2_index);
    }

    #[test]
    fn have_bsu_and_bpu1_and_bpu2_with_luft_not_enough_below() {
        //test1
        let candles = vec![
            CandleInstance {
                time_key: 00,
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
                time_key: 00,
                high: 7.0,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
            CandleInstance {
                time_key: 00,
                high: 6.8,
                open: 5.0,
                close: 6.0,
                low: 4.0,
            },
        ];

        let result = super::find_bpu_bsu(&candles, 7.0, 0.05.into());
        assert!(result.is_none());
    }
}
