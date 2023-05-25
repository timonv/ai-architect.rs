use core::fmt;

use super::Rule;

#[derive(Debug)]
pub enum ParseError {
    RuleError(String),
    MissingRootPackage(String),
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        ParseError::RuleError(e.to_string())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::RuleError(ref s) => write!(f, "RuleError: {}", s),
            ParseError::MissingRootPackage(ref s) => write!(f, "MissingRootPackage: {}", s),
        }
    }
}
