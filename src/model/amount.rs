use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::model::error::AppError;
use crate::model::newtype::{BankName, Name};
use crate::traits::amount::Exchange;

#[derive(Debug)]
pub struct BankExchange {
    bank_name: BankName,
    fee: f64,
    rate_sell: f64,
    rate_buy: f64,
    exchange_rate: ExchangeRate
}

#[derive(Debug)]
pub struct PersonExchange {
    person_name: Name,
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

}

#[derive(Debug)]
pub enum AnyExchange {
    Rate(ExchangeRate),
    Bank(BankExchange),
    Person(PersonExchange)
}
impl Exchange for AnyExchange {
    fn from(&self) -> String {
        match self {
            AnyExchange::Rate(r) => r.from(),
            AnyExchange::Bank(b) => b.from(),
            AnyExchange::Person(p) => p.from(),
        }
    }
    fn to(&self) -> String {
        match self {
            AnyExchange::Rate(r) => r.to(),
            AnyExchange::Bank(b) => b.to(),
            AnyExchange::Person(p) => p.to(),
        }
    }
    fn rate(&self) -> f64 {
        match self {
            AnyExchange::Rate(r) => r.rate(),
            AnyExchange::Bank(b) => b.rate(),
            AnyExchange::Person(p) => p.rate()
        }
    }
    fn date(&self) -> DateTime<Utc> {
        match self {
            AnyExchange::Rate(r) => r.date(),
            AnyExchange::Bank(b) => b.date(),
            AnyExchange::Person(p) => p.date()
        }
    }

    fn exchange_rate(&self) -> ExchangeRate {
        match self {
            AnyExchange::Rate(r) => r.exchange_rate(),
            AnyExchange::Bank(b) => b.exchange_rate(),
            AnyExchange::Person(p) => p.exchange_rate()
        }
    }

    fn make_from_str(input: &str) -> Result<AnyExchange, AppError> {
        Self::make_from_string(input)
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

    fn exchange_rate(&self) -> ExchangeRate {
        self.clone()
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

    fn exchange_rate(&self) -> ExchangeRate {
        self.exchange_rate.clone()
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

impl Exchange for PersonExchange {
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

    fn exchange_rate(&self) -> ExchangeRate {
        self.exchange_rate.clone()
    }

    fn make_from_str(input: &str) -> Result<Self, AppError>
    where
        Self: Sized
    {
        let input_vec = input.split(' ').collect::<Vec<&str>>();

        let name: Name = input_vec[0].parse()?;
        let rate_sell: f64 = input_vec[1].parse()?;
        let rate_buy: f64 = input_vec[2].parse()?;

        let first_exchange = input_vec[3].to_string();
        let second_exchange = input_vec[4].to_string();

        let rate = input_vec[5].to_string();
        let date = input_vec[6].to_string();

        let rate = f64::from_str(&rate)?;

        let date = dateparser::parse(&date)?;

        let rate = ExchangeRate::new(
            first_exchange.as_str(),
            second_exchange.as_str(),
            rate,
            date,
        );
        Ok(Self::new(
            name,
            rate_sell,
            rate_buy,
            rate
        ))
    }
}

impl AnyExchange {
    pub fn make_from_string(input: &str) -> Result<AnyExchange, AppError> {
        let mut errors = Vec::with_capacity(5);

        match BankExchange::make_from_str(input) {
            Ok(r) => {
                return Ok(AnyExchange::Bank(r));
            }
            Err(e) => {
                errors.push(e)
            }
        }

        match ExchangeRate::make_from_str(input) {
            Ok(r) => {
                return Ok(AnyExchange::Rate(r));
            }
            Err(e) => {
                errors.push(e)
            }
        }

        match PersonExchange::make_from_str(input) {
            Ok(r) => {
                return Ok(AnyExchange::Person(r));
            }
            Err(e) => {
                errors.push(e)
            }
        }

        let errors = errors.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n");
        Err(AppError::InternalError(
            format!("Parse Exchange error: {}", errors)
        ))
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
        if input_vec.len() != 4 {return Err(AppError::Internal("Invalid size".to_string()))}

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


impl PersonExchange {
    pub fn new(
        person_name: Name,
        rate_sell: f64,
        rate_buy: f64,
        exchange_rate: ExchangeRate
    ) -> Self {
        Self {
            person_name,
            rate_sell,
            rate_buy,
            exchange_rate
        }
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
