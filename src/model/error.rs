use std::num::ParseFloatError;
use crate::model::newtype::{BankNameError, NameError};
use anyhow::Error;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum AppError {
    BankNameError(BankNameError),
    IoError(IoError),
    NameError(NameError),
    ParseFloatError(ParseFloatError),
    DateParseError(Error),
    InternalError(String),
}

impl std::error::Error for AppError {}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BankNameError(e) => {
                write!(f, "BankNameError: {}", e)
            }
            AppError::ParseFloatError(e) => {
                write!(f, "ParseFloatError: {}", e)
            }
            AppError::DateParseError(e) => {
                write!(f, "DateParseError: {}", e)
            }
            AppError::InternalError(e) => {
                write!(f, "InternalError: {}", e)
            }
            AppError::NameError(e) => {
                write!(f, "NameError: {}", e)
            }
            AppError::IoError(e) => {
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

err_impl!(BankNameError, BankNameError);
err_impl!(ParseFloatError, ParseFloatError);
err_impl!(Error, DateParseError);
err_impl!(NameError, NameError);
err_impl!(IoError, IoError);
