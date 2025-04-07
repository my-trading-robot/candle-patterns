pub trait Candle {
    fn get_time_key(&self) -> u64;
    fn get_open(&self) -> f64;
    fn get_high(&self) -> f64;
    fn get_low(&self) -> f64;
    fn get_close(&self) -> f64;
}

#[derive(Debug, Clone)]
pub struct CandleInstance {
    pub time_key: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl Candle for CandleInstance {
    fn get_time_key(&self) -> u64 {
        self.time_key
    }

    fn get_open(&self) -> f64 {
        self.open
    }

    fn get_high(&self) -> f64 {
        self.high
    }

    fn get_low(&self) -> f64 {
        self.low
    }

    fn get_close(&self) -> f64 {
        self.close
    }
}
