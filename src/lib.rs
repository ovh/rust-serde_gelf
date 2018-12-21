extern crate serde;
extern crate serde_value;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

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

//
//fn main() {
//    use serde_value::Value;
//    use std::collections::HashMap;
//
//    #[derive(Serialize)]
//    struct SubFoo {
//        sa: String,
//        sb: isize,
//    }
//
//    #[derive(Serialize)]
//    struct Foo {
//        a: u32,
//        b: String,
//        c: Vec<bool>,
//        d: HashMap<String, Value>,
//        e: SubFoo,
//    }
//
//    let foo = Foo {
//        a: 15,
//        b: "hello".into(),
//        c: vec![true, false],
//        d: {
//            let mut map = HashMap::new();
//            map.insert("k1".to_string(), Value::F64(5.9));
//            map.insert("k2".to_string(), Value::Bool(false));
//            map
//        },
//        e: SubFoo { sa: "test".to_string(), sb: 5 },
//    };
//    let _ = to_string_pretty(&foo);
////    println!("{}", to_string_pretty(&foo).unwrap());
////    println!("{}", to_string(&foo).unwrap());
//}