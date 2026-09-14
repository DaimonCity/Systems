use crate::handler::amount::exchange_rate_handler;

mod model;
mod handler;
pub mod traits;

fn main() {
    exchange_rate_handler()
}
