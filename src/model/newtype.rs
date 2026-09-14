use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct BankName(String);
#[derive(Debug)]
pub struct Name(String);

macro_rules! impl_str_newtype {
    ($newtype:ident, $error:ident) => {
        impl Display for $newtype {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        
        impl FromStr for $newtype {
            type Err = $error;
            
            fn from_str(input: &str) -> Result<Self, Self::Err> {
                if input.len() < 3 {
                    return Err($error::MinUnneeded);
                }

                if input.len() > 50 {
                    return Err($error::MaxExceeded);
                }
                
                Ok(Self(input.to_string()))
            }
        }
    };
}

impl_str_newtype!(Name, NameError);
impl_str_newtype!(BankName, BankNameError);

macro_rules! impl_error_newtype {
    ($error:ident) => {
        impl Display for $error {
            
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $error::MaxExceeded => {
                        write!(f, "Maximum name exceeded")
                    }
                    $error::MinUnneeded => {
                        write!(f, "Minimum name unneeded")
                    } 
                }
            }
        }
    };
}



#[derive(Debug)]
pub enum BankNameError {
    MaxExceeded,
    MinUnneeded,
}

#[derive(Debug)]
pub enum NameError {
    MaxExceeded,
    MinUnneeded
}

impl_error_newtype!(BankNameError);
impl_error_newtype!(NameError);