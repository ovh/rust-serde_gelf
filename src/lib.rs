extern crate serde;
extern crate serde_value;
extern crate serde_json;

mod ser;

#[derive(Debug)]
pub enum Error{
    ValueSerializerError(serde_value::SerializerError),
    JsonSerializerError(serde_json::Error),
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

pub fn to_string_pretty<T: ?Sized>(value: &T) -> Result<String> where T: serde::Serialize {
    let f = ser::GelfField::new("", "", &serde_value::to_value(value)?);
    Ok(serde_json::to_string_pretty(&f.disassemble())?)
}

pub fn to_string<T: ?Sized>(value: &T) -> Result<String> where T: serde::Serialize {
    let f = ser::GelfField::new("", "", &serde_value::to_value(value)?);
    Ok(serde_json::to_string(&f.disassemble())?)
}
