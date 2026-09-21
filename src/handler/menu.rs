use crate::handler::amount::ExchangeHandler;
use crate::model::error::AppError;
use crate::traits::handler::StrHandler;
use std::io::stdin;

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
        println!("===== Rates =====");
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
            _ => return Err(AppError::Internal("Unknown option".to_string())),
        }
        Ok(())
    }

    pub fn clear(&self) {
        clearscreen::clear().expect("Не удалось очистить экран");
    }

    pub fn schema(&mut self, option: u8, handler: &mut ExchangeHandler) -> Result<(), AppError> {
        self.show_schema(option)?;
        let input = self.input();
        handler.handle(&input)?;
        Ok(())
    }

    pub fn flow_from_main(
        &mut self,
        option: u8,
        handler: &mut ExchangeHandler,
    ) -> Result<bool, AppError> {
        match option {
            1 => self.flow_to_rate(handler)?,
            2 => {}
            3 => {}
            4 => return Ok(true),
            _ => return Err(AppError::Internal("Unknown option".to_string())),
        }

        Ok(false)
    }

    pub fn flow_to_rate(&mut self, handler: &mut ExchangeHandler) -> Result<(), AppError> {
        self.show_rates();
        let option = self.get_u8();
        match option {
            1 => self.schema(option, handler)?,
            2 => self.schema(option, handler)?,
            3 => self.schema(option, handler)?,
            4 => {}
            _ => return Err(AppError::Internal("Unknown option".to_string())),
        }
        Ok(())
    }

    fn get_u8(&mut self) -> u8 {
        let mut num = self.input().trim().parse::<u8>();
        while let Err(e) = num {
            println!("{}", e);
            num = self.input().trim().parse::<u8>();
            continue;
        }
        num.unwrap()
    }

    pub fn input(&mut self) -> String {
        let mut input = self.raw_input();
        while let Err(e) = input {
            println!("{}", e);
            input = self.raw_input();
            continue;
        }
        input.unwrap()
    }

    fn raw_input(&self) -> Result<String, AppError> {
        let mut input: String = String::new();

        if let Err(e) = stdin().read_line(&mut input) {
            return Err(AppError::Io(e));
        }
        Ok(input)
    }
}
