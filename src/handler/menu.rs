use crate::model::error::AppError;
use std::io::stdin;
use crate::handler::amount::ExchangeHandler;
use crate::traits::handler::StrHandler;

pub struct Menu {}

impl Menu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show_main_menu(&self) {
        self.clear();
        println!("===== Menu =====");
        println!("1. Ввод курсов");
        println!("2. Считать файл");
        println!("3. Сохранить в файл");
        println!("4. Выход");
    }

    pub fn show_rates(&self) {
        self.clear();
        println!("1. Ввести обычный курс");
        println!("2. Ввести банковский курс");
        println!("3. Ввести курс человека");
        println!("4. Назад");
    }

    pub fn show_schema(&self, option: u8) -> Result<(), AppError> {
        self.clear();
        println!("===== Schema =====");
        match option {
            1 => println!("<amount_name> <amount_name> <rate value> <date>"),
            2 => println!(
                "<bank name> <fee value> <rate sell> <rate buy> <amount_name> <amount_name> <rate value> <date>"
            ),
            3 => println!(
                "<person name> <rate sell> <rate buy> <amount_name> <amount_name> <rate value> <date>"
            ),
            _ => return Err(AppError::InternalError("Unknown option".to_string())),
        }
        Ok(())
    }

    pub fn clear(&self) {
        clearscreen::clear().expect("Не удалось очистить экран");
    }

    pub fn schema(&mut self, option: u8, handler: &mut ExchangeHandler) -> Result<(), AppError> {
        self.show_schema(option)?;
        let mut input = self.get_input();
        loop {
            if let Err(e) = input {
                println!("{}", e);
                input = self.get_input();
                continue;
            } else {
                break;
            }
        }
        let input = input?;
        handler.handle(&input)?;
        Ok(())
    }


    pub fn flow_from_main(&mut self, option: u8, handler: &mut ExchangeHandler) -> Result<bool, AppError> {
        match option {
            1 => self.schema(option, handler)?,
            2 => self.schema(option, handler)?,
            3 => self.schema(option, handler)?,
            4 => return Ok(true),
            _ => return Err(AppError::InternalError("Unknown option".to_string())),
        }

        Ok(false)
    }

    pub fn get_input(&self) -> Result<String, AppError> {
        let mut input: String = String::new();

        if let Err(e) = stdin().read_line(&mut input) {
            return Err(AppError::IoError(e));
        }
        Ok(input)
    }
}
