use std::io::stdin;
use crate::model::amount::ExchangeRate;

fn exchange_rate_handler() {
    loop {
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {
                eprintln!("Error reading input");
                continue;
            }
        };
        let input = input.trim();

        if input.is_empty() {
            eprintln!("You did not enter a exchange rate");
            continue;
        }
        
        let exchange_rate = ExchangeRate::make_from_string(input);
        
        let exchange_rate = match exchange_rate {
            Ok(exchange_rate) => {
                exchange_rate
            },
            Err(e) => {
                eprintln!("{}", e);
                continue
            }
        };
        
        println!("{}", exchange_rate);
        println!("{:?}", exchange_rate);
    }

}
