use std::num::ParseFloatError;
use crate::model::newtype::{BankNameError, NameError};
use anyhow::Error;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum AppError {
    BankName(BankNameError),
    Io(IoError),
    Name(NameError),
    ParseFloat(ParseFloatError),
    DateParse(Error),
    Internal(String),
}

impl std::error::Error for AppError {}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BankName(e) => {
                write!(f, "BankNameError: {}", e)
            }
            AppError::ParseFloat(e) => {
                write!(f, "ParseFloatError: {}", e)
            }
            AppError::DateParse(e) => {
                write!(f, "DateParseError: {}", e)
            }
            AppError::Internal(e) => {
                write!(f, "InternalError: {}", e)
            }
            AppError::Name(e) => {
                write!(f, "NameError: {}", e)
            }
            AppError::Io(e) => {
                write!(f, "IoError: {}", e)
            }
        }
    }
}

macro_rules! err_impl {
    ($name:ident, $error:ident) => {
        impl From<$name> for AppError {
            fn from(e: $name) -> Self {
                AppError::$error(e)
            }
        }
    };
}

err_impl!(BankNameError, BankName);
err_impl!(ParseFloatError, ParseFloat);
err_impl!(Error, DateParse);
err_impl!(NameError, Name);
err_impl!(IoError, Io);
