use pest::error::Error;

use crate::myparser::{into_diagnostics, Rule};

#[derive(Debug)]
pub enum MyError {
    PestError(Box<Error<Rule>>),
    KeymapError(String),
}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::PestError(e) => write!(f, "{}", into_diagnostics(e)),
            MyError::KeymapError(e) => write!(f, "{}", e),
        }
    }
}

impl From<Error<Rule>> for MyError {
    fn from(e: Error<Rule>) -> Self {
        MyError::PestError(Box::new(e))
    }
}

impl From<String> for MyError {
    fn from(e: String) -> Self {
        MyError::KeymapError(e)
    }
}
