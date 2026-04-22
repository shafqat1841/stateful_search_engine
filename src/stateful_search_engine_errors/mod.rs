use std::fmt::Display;

pub enum AllErros {
    IOErr(std::io::Error),
}

impl Display for AllErros {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AllErros::IOErr(value) => {
                write!(f, "Error: {}", value)
            }
            _ => {
                write!(f, "unknown error")
            }
        }
    }
}

impl From<std::io::Error> for AllErros {
    fn from(value: std::io::Error) -> Self {
        return AllErros::IOErr(value);
    }
}
