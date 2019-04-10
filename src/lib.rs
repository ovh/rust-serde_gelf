#![deny(warnings)]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_value;

pub mod ser;
pub mod error;
pub mod record;
pub mod level;

pub fn to_string_pretty<T: ?Sized>(value: &T) -> error::Result<String> where T: serde::Serialize {
    let f = ser::GelfField::new("", "", &serde_value::to_value(value)?);
    Ok(serde_json::to_string_pretty(&f.disassemble())?)
}

pub fn to_string<T: ?Sized>(value: &T) -> error::Result<String> where T: serde::Serialize {
    let f = ser::GelfField::new("", "", &serde_value::to_value(value)?);
    Ok(serde_json::to_string(&f.disassemble())?)
}

#[macro_export]
macro_rules! gelf {
    ($data:expr) => {{
        match serde_value::to_value($data) {
            Ok(value) => Ok($crate::ser::GelfField::new("", "", &value).disassemble()),
            Err(err) => Err(err)
        }
    }};
}

#[macro_export]
macro_rules! gelf_record {
    (level: $level:expr, extra: $extra:expr, $($arg:tt)+ ) => {{
        $crate::record::GelfRecord::new()
            .set_facility(module_path!().to_string())
            .set_file(file!().to_string())
            .set_line(line!())
            .set_level($level)
            .set_message(format_args!($($arg)+).to_string())
            .add_additional_fields($extra.clone())
    }};
    (level: $level:expr, $($arg:tt)+ ) => {gelf_record!(level: $level, extra: &BTreeMap::new(), $($arg)+)};
    (extra: $extra:expr, $($arg:tt)+ ) => {gelf_record!(level: $crate::level::GelfLevel::default(), extra: $extra, $($arg)+)};
    ($($arg:tt)+) => {gelf_record!(level: $crate::level::GelfLevel::default(), extra: &BTreeMap::new(), $($arg)+)};
}