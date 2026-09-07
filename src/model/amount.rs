use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ExchangeRate {
    pub from: String,
    pub to: String,
    pub rate: f64,
    pub date:  DateTime<Utc>,
}

impl ExchangeRate {
    pub fn new(
        first_exchange: &str,
        second_exchange: &str,
        rate: f64,
        date: DateTime<Utc>,
    ) -> Self {
        Self {
            from: first_exchange.to_string(),
            to: second_exchange.to_string(),
            rate,
            date,
        }
    }

    pub fn make_from_string(input: &str) -> Result<ExchangeRate, String> {
        let input_vec = input.split(' ').collect::<Vec<&str>>();

        let first_exchange = input_vec[0].to_string();
        let second_exchange = input_vec[1].to_string();

        let rate = input_vec[2].to_string();
        let date = input_vec[3].to_string();

        let rate = f64::from_str(&rate);
        let rate = match rate {
            Ok(r) => r,
            Err(e) => return Err(format!("Invalid rate: {}", e)),
        };

        let date = dateparser::parse(&date);

        let date = match date {
            Ok(d) => d,
            Err(e) => return Err(format!("Invalid date: {}", e)),
        };

        Ok(ExchangeRate::new(
            first_exchange.as_str(),
            second_exchange.as_str(),
            rate,
            date,
        ))
    }
}

impl Display for ExchangeRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "From {} to {} with rate {} at {}",
            self.from, self.to, self.rate, self.date
        ))
    }
}
