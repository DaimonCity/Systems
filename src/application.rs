use crate::handler::amount::ExchangeHandler;
use crate::handler::menu;
use crate::model::error::AppError;
use crate::traits::handler::StrHandler;

pub struct App {
    menu: menu::Menu,
    exchange_handler: ExchangeHandler,
}

impl App {
    pub fn new() -> Self {
        Self {
            menu: menu::Menu::new(),
            exchange_handler: ExchangeHandler::new(),
        }
    }

    pub fn start(&mut self) {
        self.app_loop()
    }

    pub fn schema(&mut self, option: u8) -> Result<(), AppError> {
        self.menu.show_schema(option)?;
        let mut input = self.menu.get_input();
        loop {
            if let Err(e) = input {
                println!("{}", e);
                input = self.menu.get_input();
                continue;
            } else {
                break;
            }
        }
        let input = input?;
        self.exchange_handler.handle(&input)?;
        Ok(())
    }

    pub fn flow_from_main(&mut self, option: u8) -> Result<(), AppError> {
        if option <= 0 || option >= 5 {
            return Err(AppError::InternalError("Unknown option".to_string()));
        }
        self.schema(option)?;

        Ok(())
    }

    fn app_loop(&mut self) {
        loop {
            self.menu.show_main_menu();
            let key = self.menu.get_input();
            if let Err(e) = key {
                println!("{}", e);
                continue;
            }
            let key = key.unwrap();

            if let Ok(n) = key.parse()
                && let Err(e) = self.flow_from_main(n)
            {
                println!("{}", e);
            }
        }
    }
}
