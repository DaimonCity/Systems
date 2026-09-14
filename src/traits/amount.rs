use chrono::{DateTime, Utc};

pub trait Exchange {
    fn from(&self) -> String;
    fn to(&self) -> String;
    fn rate(&self) -> f64;
    fn date(&self) ->  DateTime<Utc>;
}