use chrono::{DateTime, Utc};
use crate::model::amount::ExchangeRate;
use crate::model::error::AppError;

pub trait Exchange {
    fn from(&self) -> String;
    fn to(&self) -> String;
    fn rate(&self) -> f64;
    fn date(&self) ->  DateTime<Utc>;
    
    fn exchange_rate(&self) -> ExchangeRate;
    
    fn make_from_str(input: &str) -> Result<Self, AppError> where Self: Sized;
}