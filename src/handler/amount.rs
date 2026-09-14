use crate::model::amount::ExchangeRate;
use std::collections::HashMap;
use std::io::stdin;

pub fn exchange_rate_handler() {
    let mut array = Vec::with_capacity(10);

    loop {
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error reading input");
                continue;
            }
        };
        let input = input.trim();

        if input.is_empty() {
            break;
        }

        let exchange_rate = ExchangeRate::make_from_string(input);

        let exchange_rate = match exchange_rate {
            Ok(exchange_rate) => exchange_rate,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        println!("{}", exchange_rate);
        println!("{:?}", exchange_rate);
        array.push(exchange_rate);
    }

    println!("{}", get_big_rate(&array));
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
