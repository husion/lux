use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum LuxError {
    EmptyInput,
    MismatchedLengths { wavelengths: usize, values: usize },
    NonMonotonicWavelengths,
    InvalidGridSpec,
    UnsupportedObserver(&'static str),
    MissingObserver,
    ParseError(&'static str),
}

impl Display for LuxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "input cannot be empty"),
            Self::MismatchedLengths {
                wavelengths,
                values,
            } => write!(
                f,
                "wavelength/value length mismatch: wavelengths={}, values={}",
                wavelengths, values
            ),
            Self::NonMonotonicWavelengths => {
                write!(f, "wavelengths must be strictly increasing")
            }
            Self::InvalidGridSpec => write!(f, "invalid wavelength grid specification"),
            Self::UnsupportedObserver(name) => write!(f, "unsupported observer: {}", name),
            Self::MissingObserver => write!(f, "an observer is required for this operation"),
            Self::ParseError(message) => write!(f, "parse error: {}", message),
        }
    }
}

impl std::error::Error for LuxError {}

pub type LuxResult<T> = Result<T, LuxError>;
