use crate::handler::amount::ExchangeHandler;
use crate::handler::menu;
use std::io::stdin;

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

    fn app_loop(&mut self) {
        loop {
            self.menu.show_main_menu();
            let key = self.menu.input();
           
            let option: Result<u8, _> = key.trim().parse();
            if let Err(e) = option {
                println!("{}", e);
                continue;
            }
            let option = option.unwrap();
            
            let flow = self.menu.flow_from_main(option, &mut self.exchange_handler);

            if let Err(e) = flow
            {
                println!("{}", e);
                stdin().read_line(&mut String::new()).ok();
            } else if flow.unwrap() {
                break;
            }
        }
    }
}
