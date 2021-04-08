use std::{
    fmt,
    fmt::{Display, Formatter},
};

pub struct Error {
    inner: InnerError,
}

pub(crate) struct InnerError {
    pub kind: Kind,
    pub message: String,
}

#[derive(Debug)]
pub(crate) enum Kind {
    Reqwest,
    Business,
}

impl Error {
    pub fn from_str(s: &str) -> Error {
        Error {
            inner: InnerError {
                kind: Kind::Business,
                message: s.to_string(),
            },
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self {
            inner: InnerError {
                kind: Kind::Reqwest,
                message: format!("{}", e),
            },
        }
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self {
            inner: InnerError {
                kind: Kind::Business,
                message: s.to_string(),
            },
        }
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self {
            inner: InnerError {
                kind: Kind::Business,
                message: s.to_string(),
            },
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.inner.kind, self.inner.message)
    }
}
