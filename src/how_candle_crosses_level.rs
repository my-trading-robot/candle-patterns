use crate::candle::Candle;

#[derive(Debug, Clone, Copy)]
pub enum HowCandleCrossesLevel {
    CandleIsBelow { distance: f64 },
    CandleTouchesBelow,
    CandleIsAbove { distance: f64 },
    CandleTouchesAbove,
    BodyIsBelow,
    BodyIsAbove,
    BodyCrossesTheLevel,
}

impl HowCandleCrossesLevel {
    pub fn from_candle_and_level(c: &impl Candle, level: f64) -> Self {
        let high = c.get_high();
        if high < level {
            return Self::CandleIsBelow {
                distance: level - high,
            };
        }

        if high == level {
            return Self::CandleTouchesBelow;
        }

        let low = c.get_low();
        if level < low {
            return Self::CandleIsAbove {
                distance: low - level,
            };
        }

        if level == low {
            return Self::CandleTouchesAbove;
        }

        let open = c.get_open();
        let close = c.get_close();

        if level < open && level < close {
            return Self::BodyIsAbove;
        }

        if level > open && level > close {
            return Self::BodyIsBelow;
        }

        Self::BodyCrossesTheLevel
    }

    pub fn is_candle_touches_the_level(&self) -> bool {
        match self {
            HowCandleCrossesLevel::CandleTouchesBelow => true,
            HowCandleCrossesLevel::CandleTouchesAbove => true,
            _ => false,
        }
    }

    pub fn is_candle_crosses_the_level(&self) -> bool {
        match self {
            HowCandleCrossesLevel::CandleTouchesBelow => false,
            HowCandleCrossesLevel::CandleTouchesAbove => false,
            _ => true,
        }
    }

    pub fn is_above_or_touches_above(&self) -> bool {
        match self {
            HowCandleCrossesLevel::CandleIsAbove { distance: _ } => true,
            HowCandleCrossesLevel::CandleTouchesAbove => true,
            _ => false,
        }
    }

    pub fn is_below_or_touches_below(&self) -> bool {
        match self {
            HowCandleCrossesLevel::CandleIsBelow { distance: _ } => true,
            HowCandleCrossesLevel::CandleTouchesBelow => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::candle::CandleInstance;

    use super::HowCandleCrossesLevel;

    #[test]
    fn test_candle_lower_than_level_case() {
        let c = CandleInstance {
            time_key: 0,
            open: 5.0,
            close: 6.0,
            high: 7.0,
            low: 4.0,
        };

        let candle_position = HowCandleCrossesLevel::from_candle_and_level(&c, 8.0);

        if let HowCandleCrossesLevel::CandleIsBelow { distance } = candle_position {
            assert_eq!(distance, 1.0,)
        } else {
            panic!("Candle can not be at state {:?}", candle_position);
        }
    }

    #[test]
    fn test_candle_body_lower_than_level_case() {
        let c = CandleInstance {
            time_key: 0,
            open: 5.0,
            close: 6.0,
            high: 7.0,
            low: 4.0,
        };

        let candle_position = HowCandleCrossesLevel::from_candle_and_level(&c, 6.5);

        if let HowCandleCrossesLevel::BodyIsBelow = candle_position {
        } else {
            panic!("Candle can not be at state {:?}", candle_position);
        }

        let c = CandleInstance {
            time_key: 0,
            open: 6.0,
            close: 5.0,
            high: 7.0,
            low: 4.0,
        };

        let candle_position = HowCandleCrossesLevel::from_candle_and_level(&c, 6.5);

        if let HowCandleCrossesLevel::BodyIsBelow = candle_position {
        } else {
            panic!("Candle can not be at state {:?}", candle_position);
        }
    }

    #[test]
    fn test_candle_body_above_than_level_case() {
        let c = CandleInstance {
            time_key: 0,
            open: 5.0,
            close: 6.0,
            high: 7.0,
            low: 4.0,
        };

        let candle_position = HowCandleCrossesLevel::from_candle_and_level(&c, 4.5);

        if let HowCandleCrossesLevel::BodyIsAbove = candle_position {
        } else {
            panic!("Candle can not be at state {:?}", candle_position);
        }

        let c = CandleInstance {
            time_key: 0,
            open: 6.0,
            close: 5.0,
            high: 7.0,
            low: 4.0,
        };

        let candle_position = HowCandleCrossesLevel::from_candle_and_level(&c, 4.5);

        if let HowCandleCrossesLevel::BodyIsAbove = candle_position {
        } else {
            panic!("Candle can not be at state {:?}", candle_position);
        }
    }

    #[test]
    fn test_candle_touches_lower_than_level_case() {
        let c = CandleInstance {
            time_key: 0,
            open: 5.0,
            close: 6.0,
            high: 7.0,
            low: 4.0,
        };

        let candle_position = HowCandleCrossesLevel::from_candle_and_level(&c, 7.0);

        if let HowCandleCrossesLevel::CandleTouchesBelow = candle_position {
        } else {
            panic!("Candle can not be at state {:?}", candle_position);
        }
    }

    #[test]
    fn test_candle_higher_than_level_case() {
        let c = CandleInstance {
            time_key: 0,
            open: 5.0,
            close: 6.0,
            high: 7.0,
            low: 4.0,
        };

        let candle_position = HowCandleCrossesLevel::from_candle_and_level(&c, 3.0);

        if let HowCandleCrossesLevel::CandleIsAbove { distance } = candle_position {
            assert_eq!(distance, 1.0);
        } else {
            panic!("Candle can not be at state {:?}", candle_position);
        }
    }

    #[test]
    fn test_candle_touches_higher_than_level_case() {
        let c = CandleInstance {
            time_key: 0,
            open: 5.0,
            close: 6.0,
            high: 7.0,
            low: 4.0,
        };

        let candle_position = HowCandleCrossesLevel::from_candle_and_level(&c, 4.0);

        if let HowCandleCrossesLevel::CandleTouchesAbove = candle_position {
        } else {
            panic!("Candle can not be at state {:?}", candle_position);
        }
    }

    #[test]
    fn test_candle_body_crosses_the_level() {
        let c = CandleInstance {
            time_key: 0,
            open: 5.0,
            close: 6.0,
            high: 7.0,
            low: 4.0,
        };

        let candle_position = HowCandleCrossesLevel::from_candle_and_level(&c, 5.5);

        if let HowCandleCrossesLevel::BodyCrossesTheLevel = candle_position {
        } else {
            panic!("Candle can not be at state {:?}", candle_position);
        }
    }
}
