// Copyright 2019-present, OVH SAS
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! # Serde GELF
//!
//! The Graylog Extended Log Format (GELF) is a log format that avoids the shortcomings of classic
//! log formats. GELF is a great choice for logging from within applications. There are libraries
//! and appenders for many programming languages and logging frameworks so it is easy to implement.
//! You could use GELF to send every exception as a log message to your Graylog cluster.
//!
//! ```json
//! {
//!     "facility": "src",
//!     "file": "examples/src/main.rs",
//!     "host": "myDesk",
//!     "level": 1,
//!     "_levelname": "Alert",
//!     "line": 21,
//!     "short_message": "Message with the default level",
//!     "timestamp": 1554907321.6123526,
//!     "version": "1.1"
//! }
//! ```
//!
//! ## Quickstart
//!
//! You can start using it by first adding it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serde_derive = "1.0"
//! serde_gelf = "0.1"
//! ```
//!
//! Then, create a structure which implement the `serde::Serialize` trait:
//!
//! ```rust
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_gelf;
//!
//! #[derive(Serialize)]
//! struct Foo {
//!     a: u32,
//!     b: String,
//! }
//!
//! fn main() {
//!     let foo = Foo { a: 15, b: "hello".into() };
//!     println!("{:?}", serde_gelf::to_flat_dict(&foo).unwrap());
//! }
//! ```
//! **Output**:
//! ```text
//! {"_a": U32(15), "_b": String("hello")}
//! ```
#![doc(
    html_logo_url = "https://eu.api.ovh.com/images/com-square-bichro.png",
    html_favicon_url = "https://www.ovh.com/favicon.ico",
)]
#![deny(warnings, missing_docs)]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_value;

pub use level::GelfLevel;
pub use record::{GelfRecord, GelfRecordBuilder, GelfRecordGetter, GelfRecordSetter};

mod ser;
mod record;
mod level;

#[macro_use]
mod macros;

/// Transform any serializable object into a single level hashmap of key / value.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate serde_gelf;
///
/// #[derive(Serialize)]
/// struct SubFoo {
///     c: bool,
///     d: String,
/// }
///
/// #[derive(Serialize)]
/// struct Foo {
///     a: u32,
///     b: SubFoo,
/// }
///
/// fn main() {
///     let foo = Foo { a: 15, b: SubFoo { c: true, d: "hello".into() }};
///     println!("{:?}", serde_gelf::to_flat_dict(&foo).unwrap());
/// }
/// ```
/// **Output**:
/// ```text
/// {"_a": U32(15), "_b_c": Bool(true), "_b_d": String("hello")}
/// ```
pub fn to_flat_dict<S: ?Sized>(value: &S) -> Result<std::collections::BTreeMap<String, serde_value::Value>, serde_value::SerializerError> where S: serde::Serialize {
    Ok(ser::FlatSerializer::disassemble("", "", &serde_value::to_value(value)?))
}