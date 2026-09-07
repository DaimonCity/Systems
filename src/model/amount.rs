use chrono::{DateTime, Utc};

pub struct ExchangeRate {
    pub first_exchange: String,
    pub second_exchange: String,
    pub rate: f64,
    pub date: DateTime<Utc>
}