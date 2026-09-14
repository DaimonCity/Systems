use std::error::Error;
use crate::model::newtype::BankNameError;

#[derive(Debug)]
pub enum AppError {
    BankNameError(BankNameError),
}

impl Error for AppError {}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BankNameError(e) => {
                write!(f, "BankNameError: {}", e)
            }
        }
    }
}

macro_rules! err_impl {
    ($name:ident, $type:ty) => {
        impl From<$name> for AppError {
            fn from(e: $name) -> Self {
                AppError::$name(e)
            }
        }
    };
}

err_impl!(BankNameError, BankNameError);
