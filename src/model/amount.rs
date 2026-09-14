use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::model::error::AppError;
use crate::model::newtype::BankName;
use crate::traits::amount::Exchange;

#[derive(Debug)]
pub struct BankExchange {
    bank_name: BankName,
    fee: f64,
    rate_sell: f64,
    rate_buy: f64,
    exchange_rate: ExchangeRate
}

#[derive(Debug, Clone)]
pub struct ExchangeRate {
    pub from: String,
    pub to: String,
    pub rate: f64,
    pub date:  DateTime<Utc>,
}

impl BankExchange {
    pub fn new(
        bank_name: BankName,
        fee: f64,
        rate_sell: f64,
        rate_buy: f64,
        exchange_rate: ExchangeRate
    ) -> BankExchange {
        Self {
            bank_name,
            fee,
            rate_sell,
            rate_buy,
            exchange_rate
        }
    }
    
    pub fn exchange_rate(&self) -> ExchangeRate {
        self.exchange_rate.clone()
    }

    pub fn ref_exchange_rate(&self) -> &ExchangeRate {
        &self.exchange_rate
    }
}

impl Exchange for ExchangeRate {
    fn from(&self) -> String {
        self.from.clone()
    }

    fn to(&self) -> String {
        self.to.clone()
    }

    fn rate(&self) -> f64 {
        self.rate
    }

    fn date(&self) -> DateTime<Utc> {
        self.date
    }

    fn make_from_str(input: &str) -> Result<Self, AppError> {
        Self::make_from_string(input)
    }
}
impl Exchange for BankExchange {
    fn from(&self) -> String {
        self.exchange_rate.from.clone()
    }

    fn to(&self) -> String {
        self.exchange_rate.to.clone()
    }

    fn rate(&self) -> f64 {
        self.exchange_rate.rate
    }

    fn date(&self) -> DateTime<Utc> {
        self.exchange_rate.date
    }

    fn make_from_str(input: &str) -> Result<Self, AppError> {
        let input_vec = input.split(' ').collect::<Vec<&str>>();

        let bank_name: BankName = input_vec[0].parse()?;
        let fee: f64 = input_vec[1].parse()?;
        let rate_sell: f64 = input_vec[2].parse()?;
        let rate_buy: f64 = input_vec[3].parse()?;

        let first_exchange = input_vec[4].to_string();
        let second_exchange = input_vec[5].to_string();

        let rate = input_vec[6].to_string();
        let date = input_vec[7].to_string();

        let rate = f64::from_str(&rate)?;

        let date = dateparser::parse(&date)?;

        let rate = ExchangeRate::new(
            first_exchange.as_str(),
            second_exchange.as_str(),
            rate,
            date,
        );
        Ok(BankExchange::new(
            bank_name,
            fee,
            rate_sell,
            rate_buy,
            rate
        ))
    }
}

#[derive(Debug)]
pub enum AnyExchange {
    Rate(ExchangeRate),
    Bank(BankExchange),
}

impl Exchange for AnyExchange {
    fn from(&self) -> String {
        match self {
            AnyExchange::Rate(r) => r.from(),
            AnyExchange::Bank(b) => b.from(),
        }
    }
    fn to(&self) -> String {
        match self {
            AnyExchange::Rate(r) => r.to(),
            AnyExchange::Bank(b) => b.to(),
        }
    }
    fn rate(&self) -> f64 {
        match self {
            AnyExchange::Rate(r) => r.rate(),
            AnyExchange::Bank(b) => b.rate(),
        }
    }
    fn date(&self) -> DateTime<Utc> {
        match self {
            AnyExchange::Rate(r) => r.date(),
            AnyExchange::Bank(b) => b.date(),
        }
    }

    fn make_from_str(input: &str) -> Result<AnyExchange, AppError> {
        Self::make_from_string(input)
    }
}

impl AnyExchange {
    pub fn make_from_string(input: &str) -> Result<AnyExchange, AppError> {
        if let Ok(r) = BankExchange::make_from_str(input) {
            return Ok(AnyExchange::Bank(r));
        }
        
        if let Ok(r) = ExchangeRate::make_from_str(input) {
            return Ok(AnyExchange::Rate(r));
        }
        
        Err(AppError::InternalError("Parse error".to_string()))
    }
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

    pub fn make_from_string(input: &str) -> Result<ExchangeRate, AppError> {
        let input_vec = input.split(' ').collect::<Vec<&str>>();

        let first_exchange = input_vec[0].to_string();
        let second_exchange = input_vec[1].to_string();

        let rate = input_vec[2].to_string();
        let date = input_vec[3].to_string();

        let rate = f64::from_str(&rate)?;

        let date = dateparser::parse(&date)?;

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
