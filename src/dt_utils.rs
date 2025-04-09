use chrono_tz::{America::New_York, Tz};
use rust_extensions::{chrono::Timelike, date_time::DateTimeAsMicroseconds};

pub enum UsMarketMoment {
    DayOff,
    PreMarket,
    PostMarket,
    Working,
}

impl UsMarketMoment {
    pub fn from(dt: DateTimeAsMicroseconds) -> Self {
        use rust_extensions::chrono::Datelike;
        let eastern_time: rust_extensions::chrono::DateTime<Tz> =
            dt.to_chrono_utc().with_timezone(&New_York);

        match eastern_time.weekday() {
            rust_extensions::chrono::Weekday::Sat => return Self::DayOff,
            rust_extensions::chrono::Weekday::Sun => return Self::DayOff,
            _ => {}
        }

        let time = eastern_time.time();

        let now_time = time.hour() * 100 + time.minute();

        if 0930 <= now_time && now_time < 1600 {
            return Self::Working;
        }

        if 0400 <= now_time && now_time < 930 {
            return Self::PreMarket;
        }
        if 1600 <= now_time && now_time < 2000 {
            return Self::PostMarket;
        }

        Self::DayOff
    }

    pub fn is_working(&self) -> bool {
        if let Self::Working = self {
            return true;
        }

        false
    }
}
