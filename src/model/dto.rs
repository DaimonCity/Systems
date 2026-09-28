use crate::model::amount::{AnyExchange, BankExchange, ExchangeRate, PersonExchange};
use crate::model::error::AppError;
use crate::model::newtype::{BankName, Name};
use crate::traits::amount::Exchange;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct AnyExchangesDto(pub Vec<AnyExchangeDto>);

#[derive(Serialize, Deserialize)]
pub enum AnyExchangeDto {
    Rate(ExchangeDto),
    Bank(BankExchangeDto),
    Person(PersonExchangeDto),
}

impl TryFrom<AnyExchangeDto> for AnyExchange {
    type Error = AppError;

    fn try_from(value: AnyExchangeDto) -> Result<Self, Self::Error> {
        match value {
            AnyExchangeDto::Rate(r) => Ok(AnyExchange::Rate(r.into())),
            AnyExchangeDto::Bank(b) => Ok(AnyExchange::Bank(b.try_into()?)),
            AnyExchangeDto::Person(p) => Ok(AnyExchange::Person(p.try_into()?)),
        }
    }
}

impl TryFrom<AnyExchange> for AnyExchangeDto {
    type Error = AppError;

    fn try_from(value: AnyExchange) -> Result<Self, Self::Error> {
        match value {
            AnyExchange::Rate(r) => Ok(AnyExchangeDto::Rate(r.into())),
            AnyExchange::Bank(b) => Ok(AnyExchangeDto::Bank(b.into())),
            AnyExchange::Person(p) => Ok(AnyExchangeDto::Person(p.into())),
        }
    }
}
impl AnyExchangesDto {
    pub fn new(values: Vec<AnyExchange>) -> Result<Self, AppError> {
        let dto = values
            .into_iter()
            .map(AnyExchangeDto::try_from)
            .collect::<Result<Vec<AnyExchangeDto>, AppError>>()?;
        Ok(AnyExchangesDto(dto))
    }
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeDto {
    pub from: String,
    pub to: String,
    pub rate: f64,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct BankExchangeDto {
    bank_name: String,
    fee: f64,
    rate_sell: f64,
    rate_buy: f64,
    exchange_rate: ExchangeDto,
}

#[derive(Serialize, Deserialize)]
pub struct PersonExchangeDto {
    person_name: String,
    rate_sell: f64,
    rate_buy: f64,
    exchange_rate: ExchangeDto,
}

impl TryFrom<BankExchangeDto> for BankExchange {
    type Error = AppError;

    fn try_from(value: BankExchangeDto) -> Result<Self, Self::Error> {
        Ok(Self::new(
            BankName::from_str(&value.bank_name)?,
            value.fee,
            value.rate_sell,
            value.rate_buy,
            value.exchange_rate.into(),
        ))
    }
}
impl From<BankExchange> for BankExchangeDto {
    fn from(value: BankExchange) -> Self {
        Self {
            bank_name: value.bank_name(),
            fee: value.fee(),
            rate_sell: value.rate_sell(),
            rate_buy: value.rate_buy(),
            exchange_rate: value.exchange_rate().into(),
        }
    }
}

impl TryFrom<PersonExchangeDto> for PersonExchange {
    type Error = AppError;

    fn try_from(value: PersonExchangeDto) -> Result<Self, Self::Error> {
        Ok(Self::new(
            Name::from_str(&value.person_name)?,
            value.rate_sell,
            value.rate_buy,
            value.exchange_rate.into(),
        ))
    }
}

impl From<PersonExchange> for PersonExchangeDto {
    fn from(value: PersonExchange) -> Self {
        Self {
            person_name: value.name(),
            rate_sell: value.rate_sell(),
            rate_buy: value.rate_buy(),
            exchange_rate: value.exchange_rate().into(),
        }
    }
}

impl From<ExchangeDto> for ExchangeRate {
    fn from(value: ExchangeDto) -> Self {
        Self::new(&value.from, &value.to, value.rate, value.date)
    }
}

impl From<ExchangeRate> for ExchangeDto {
    fn from(value: ExchangeRate) -> Self {
        Self {
            from: value.from,
            to: value.to,
            rate: value.rate,
            date: value.date,
        }
    }
}
