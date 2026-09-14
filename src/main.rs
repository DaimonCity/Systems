use crate::handler::amount::exchange_rate_handler;

mod model;
mod handler;
pub mod traits;

fn main() {
    let _ = exchange_rate_handler();
}
