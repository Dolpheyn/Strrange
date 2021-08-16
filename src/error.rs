use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum StrrangeError {
    Str(String),
}

impl Display for StrrangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(s) => write!(f, "{}", s),
        }
    }
}

impl Error for StrrangeError {}
