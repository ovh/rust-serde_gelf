#[derive(Debug)]
pub enum Error {
    ValueSerializerError(serde_value::SerializerError),
    JsonSerializerError(serde_json::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ValueSerializerError(err) => std::fmt::Display::fmt(&err, f),
            Error::JsonSerializerError(err) => std::fmt::Display::fmt(&err, f),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<serde_value::SerializerError> for Error {
    fn from(err: serde_value::SerializerError) -> Error {
        Error::ValueSerializerError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonSerializerError(err)
    }
}

