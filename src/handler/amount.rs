use crate::model::amount::{AnyExchange, ExchangeRate};
use crate::model::error::AppError;
use crate::traits::amount::Exchange;
use crate::traits::handler::StrHandler;
use std::collections::HashMap;

pub struct ExchangeHandler {
    array: Vec<AnyExchange>,
}

impl ExchangeHandler {
    pub fn new() -> ExchangeHandler {
        ExchangeHandler { array: Vec::new() }
    }

    pub fn unpack(&self) -> Vec<ExchangeRate> {
        self.array
            .iter()
            .map(|rate| match rate {
                AnyExchange::Rate(e) => e.clone(),
                AnyExchange::Bank(e) => e.exchange_rate(),
                AnyExchange::Person(e) => e.exchange_rate(),
            })
            .collect::<Vec<ExchangeRate>>()
    }

    fn get_big_rate(&self) -> Result<ExchangeRate, AppError> {
        let rates = self.unpack();
        let main_amount = Self::get_main_amount(&rates); // dollar
        let mut amount: Vec<ExchangeRate> = rates
            .into_iter()
            .filter(|x| x.from == main_amount)
            .collect::<Vec<ExchangeRate>>();
        amount.sort_by(|x, y| x.rate.partial_cmp(&y.rate).unwrap());

        match amount.last() {
            Some(last) => Ok(last.clone()),
            None => Err(AppError::Internal("The exchange rate is empty".to_string())),
        }
    }

    fn get_main_amount(rates: &[ExchangeRate]) -> String {
        let froms = rates
            .iter()
            .map(|r| r.from.to_string())
            .collect::<Vec<String>>();
        let mut map = HashMap::new();
        for form in &froms {
            map.insert(form, 1);
        }

        for form in &froms {
            let v = map[&form];
            map.insert(form, v + 1);
        }

        let max_pair = map.iter().max_by_key(|&(_, value)| value);

        match max_pair {
            Some((key, _)) => key.to_string(),
            None => panic!("Map is empty"),
        }
    }
}

impl StrHandler for ExchangeHandler {
    fn handle(&mut self, input: &str) -> Result<(), AppError> {
        let input = input.trim();
        let rate = match AnyExchange::make_from_str(input) {
            Ok(r) => r,
            Err(e) => {
                return Err(e);
            }
        };
        println!("{:?}", rate);
        println!("{:?}", rate);
        self.array.push(rate);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn creation_simple_rate() {
        let input = "dol rus 80 2026-09-09";
        let mut handler = ExchangeHandler::new();
        handler.handle(input).unwrap();
        assert_eq!(handler.array.len(), 1);
    }
    #[test]
    fn creation_bank_rate() {
        let input = "sber 0.5 84 82 dol rus 83 2026-09-09";
        let mut handler = ExchangeHandler::new();
        handler.handle(input).unwrap();
        assert_eq!(handler.array.len(), 1);
    }

    #[test]
    fn creation_person_rate() {
        let input = "dima 84 82 dol rus 83 2026-09-09";
        let mut handler = ExchangeHandler::new();
        handler.handle(input).unwrap();
        assert_eq!(handler.array.len(), 1);
    }

    #[test]
    fn err_check() {
        let input = "dol rus 83gl;gd;m gldgmdf fdslfsdl 2026-09-09";
        let mut handler = ExchangeHandler::new();
        let err = handler.handle(input);
        assert!(err.is_err());
    }

    #[test]
    fn check_main_amount() {
        let input = "dol rus 80 2026-09-09";
        let mut handler = ExchangeHandler::new();
        handler.handle(input).unwrap();

        let input = "dol belarus 180 2026-09-09";
        handler.handle(input).unwrap();

        let input = "dol euro 100 2026-09-09";
        handler.handle(input).unwrap();

        let input = "dima 84 82 dol rus 83 2026-09-09";
        handler.handle(input).unwrap();
        assert_eq!(handler.array.len(), 4);
        assert!(handler.get_big_rate().is_ok());
    }
}
