//! Simple error "handling" and a custom error type

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Type alias for `dyn Error` to unify different error types
pub type DynError = Box<dyn Error>;

/// Custom error consisting of just a message i.e. string
#[derive(Debug)]
pub struct CustomError(Cow<'static, str>);

impl From<&'static str> for CustomError {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for CustomError {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for CustomError {}
