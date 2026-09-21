use crate::model::error::AppError;
use std::io::stdin;
use std::process::Command;

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

    pub fn clear(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cls").status().ok();
        } else {
            Command::new("clear").status().ok();
        }
    }

    pub fn get_input(&self) -> Result<String, AppError> {
        let mut input: String = String::new();

        if let Err(e) = stdin().read_line(&mut input) {
            return Err(AppError::IoError(e));
        }
        Ok(input)
    }
}
