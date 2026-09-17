use crate::model::amount::{AnyExchange, ExchangeRate};
use crate::model::error::AppError;
use std::collections::HashMap;
use std::io::stdin;
use crate::traits::amount::Exchange;

pub fn exchange_rate_handler() -> Result<(), AppError> {
    let mut array: Vec<AnyExchange> = Vec::with_capacity(10);

    loop {
        println!("Enter exchange rate OR Q for exit: ");
        println!("#1 Schema: <amount_name> <amount_name> <rate value> <date> ");
        println!("#2 Schema: <bank name> <fee value> <rate sell> <rate buy> #1 Schema");
        println!("#3 Schema: <person name> <rate sell> <rate buy> #1 Schema");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error reading input");
                continue;
            }
        };
        let input = input.trim();

        if input == "Q" {
            break;
        }

        let rate = match AnyExchange::make_from_string(input) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error input, {}", e);
                continue;
            }
        };

        println!("{:?}", rate);
        println!("{:?}", rate);
        array.push(rate);

    }

    println!("{}", get_big_rate(&unpack(&array)));
    Ok(())
}

pub fn unpack(rates: &[AnyExchange]) -> Vec<ExchangeRate> {
    rates
        .iter()
        .map(|rate| match rate {
            AnyExchange::Rate(e) => e.clone(),
            AnyExchange::Bank(e) => e.exchange_rate(),
            AnyExchange::Person(e) => e.exchange_rate()
        })
        .collect::<Vec<ExchangeRate>>()
}

pub fn get_big_rate(rates: &[ExchangeRate]) -> &ExchangeRate {
    let main_amount = get_main_amount(rates); // dollar
    let mut amount: Vec<&ExchangeRate> = rates
        .iter()
        .filter(|x| x.from == main_amount)
        .collect::<Vec<&ExchangeRate>>();
    amount.sort_by(|x, y| x.rate.partial_cmp(&y.rate).unwrap());
    match amount.last() {
        None => {
            panic!("No exchange rate available");
        }
        Some(a) => a,
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
