/// a digest parsing error
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Error {
    /// unsupported algorithm
    Algorithm,

    /// invalid character
    Character,

    /// invalid length
    Length,
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Algorithm => write!(f, "unsupported algorithm"),
            Error::Character => write!(f, "invalid character"),
            Error::Length => write!(f, "invalid length"),
        }
    }
}
