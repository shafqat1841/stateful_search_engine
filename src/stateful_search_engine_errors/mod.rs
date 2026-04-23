use std::{fmt::Display, str::Utf8Error};

pub enum AllErros {
    IOErr(std::io::Error),
    Utf8Error(Utf8Error),
    EmptyQueryErr(String),
    EmptyPathErr(String),
    WrongPathErr(String),
}

impl Display for AllErros {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AllErros::IOErr(value) => {
                write!(f, "Error: {}", value)
            }
            AllErros::Utf8Error(value) => {
                write!(f, "Error: {}", value)
            }
            AllErros::EmptyQueryErr(value) => {
                write!(f, "Error: {}", value)
            },
            AllErros::EmptyPathErr(value) => {
                write!(f, "Error: {}", value)
            },
            AllErros::WrongPathErr(value) => {
                write!(f, "Error: {}", value)
            },
        }
    }
}

impl From<std::io::Error> for AllErros {
    fn from(value: std::io::Error) -> Self {
        return AllErros::IOErr(value);
    }
}

impl From<Utf8Error> for AllErros {
    fn from(value: Utf8Error) -> Self {
        AllErros::Utf8Error(value)
    }
}
