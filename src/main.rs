use crate::application::App;

mod model;
mod handler;
mod traits;
mod application;
pub mod serde;

fn main() {
    // let _ = exchange_rate_handler();
    let mut app = App::new();

    app.start();
}

