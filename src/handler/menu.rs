use std::io::stdin;

pub struct Menu {
}

pub fn menu() {
    println!("Menu");
    println!("1. Ввод обычного курса");
    println!("2. Ввод банковского курса");
    println!("3. Ввод курса от какого-то человека");

    let mut key: String = String::new();

    match stdin().read_line(&mut key) {
        Ok(_) => {
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}