use std::str::FromStr;

pub struct BankName(String);

impl FromStr for BankName {
    type Err = BankNameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 {
            return Err(BankNameError::MinUnneeded);
        }

        if s.len() > 50 {
            return Err(BankNameError::MaxExceeded);
        }
        Ok(BankName(s.to_string()))
    }
}


#[derive(Debug)]
pub enum BankNameError{
    MaxExceeded,
    MinUnneeded
}

impl std::fmt::Display for BankNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BankNameError::MaxExceeded => {
                write!(f, "Maximum bank name exceeded")
            }
            BankNameError::MinUnneeded => {
                write!(f, "Minimum bank name unneeded")
            }
        }
    }
}