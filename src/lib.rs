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
//! serde_gelf = "0.1"
//! serde_derive = "1.0"
//! ```
//!
//! Then, create a structure which implement the `serde::Serialize` trait:
//!
//! ```rust
//! extern crate serde_gelf;
//! 
//! #[macro_use]
//! extern crate serde_derive;
//! 
//! #[derive(Serialize)]
//! struct Foo {
//!     a: u32,
//!     b: String,
//! }
//!
//! fn main() {
//!     let foo = Foo { a: 15, b: "hello".into() };
//!     println!("{}", serde_gelf::to_string_pretty(&foo).unwrap());
//! }
//! ```
//! **Output**:
//! ```json
//! {
//!   "_a": 15,
//!   "_b": "hello"
//! }
//! ```
#![deny(warnings)]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_value;

#[doc(inline)]
pub use self::ser::{to_string, to_string_pretty};

pub mod ser;
pub mod error;
pub mod record;
pub mod level;

#[macro_use]
mod macros;