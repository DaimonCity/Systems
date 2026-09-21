use crate::model::error::AppError;

pub trait StrHandler {
    fn handle(&mut self, input: &str) -> Result<(), AppError> ;
}