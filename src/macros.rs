// Copyright 2019-present, OVH SAS
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

/// Construct a `BTreeMap<String, serde_json::Value>` from any structure which implement
/// `serde::Serialize`.
///
/// ```rust
/// #[macro_use]
/// extern crate serde_derive;
/// #[macro_use]
/// extern crate serde_gelf;
/// extern crate serde_json;
/// 
/// #[derive(Serialize)]
/// struct Foo {
///     a: u32,
///     b: String,
/// }
/// 
/// fn main() {
///     let foo = gelf!(Foo { a: 15, b: "hello".into() }).unwrap();
///     println!("{}", serde_json::to_string_pretty(&foo).unwrap());
/// }
/// ```
/// **Output**:
/// ```json
/// {
///   "_a": 15,
///   "_b": "hello"
/// }
/// ```
#[macro_export]
macro_rules! gelf {
    ($data:expr) => {{
        match serde_value::to_value($data) {
            Ok(value) => Ok($crate::ser::GelfField::new("", "", &value).disassemble()),
            Err(err) => Err(err)
        }
    }};
}
/// Construct a `serde_gelf::record::GelfRecord`, a struct which follow the
/// [`GELF Payload Specification`](http://docs.graylog.org/en/3.0/pages/gelf.html#gelf-payload-specification).
///
/// ```rust
/// #[macro_use]
/// extern crate serde_gelf;
/// extern crate serde_json;
///
/// fn main() {
///     let rec = gelf_record!("hello");
///     println!("{}", serde_json::to_string_pretty(&rec).unwrap());
/// }
/// ```
/// **Output**:
/// ```json
/// {
///   "facility": "src",
///   "file": "examples/src/main.rs",
///   "host": "cdumay-desk",
///   "level": 1,
///   "_levelname": "Alert",
///   "line": 11,
///   "short_message": "hello",
///   "timestamp": 1554980878.241851,
///   "version": "1.1"
/// }
/// ```
#[macro_export]
macro_rules! gelf_record {
    (level: $level:expr, extra: $extra:expr, $($arg:tt)+ ) => {{
        use std::collections::BTreeMap;
        use $crate::record::GelfRecordBuilder;

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