// Copyright 2019-present, OVH SAS
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_value::Value;

use crate::level::GelfLevel;
use crate::to_flat_dict;

/// Builder for [`GelfRecord`](struct.GelfRecord.html).
///
/// # Examples
///
/// ```rust
/// use serde_gelf::{GelfRecord, GelfLevel};
///
/// let rec = GelfRecord::new()
///     .set_file("main.rs".into())
///     .set_facility("main".into())
///     .set_level(GelfLevel::Notice)
///     .set_line(50)
///     .set_message("Hello".into());
/// ```
pub trait GelfRecordBuilder {
    /// Construct new GelfRecordBuilder.
    fn new() -> Self;
    /// Set `GelfRecord.short_message`.
    fn set_message(self, short_message: String) -> Self;
    /// Set `GelfRecord.level`.
    fn set_level(self, level: GelfLevel) -> Self;
    /// Set `GelfRecord.timestamp`.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use serde_gelf::GelfRecord;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let rec = GelfRecord::new()
    ///     .set_timestamp({
    ///         let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    ///          now.as_secs() as f64 + f64::from(now.subsec_nanos()) / 1e9
    /// });
    ///
    /// ```
    fn set_timestamp(self, timestamp: f64) -> Self;
    /// Extend a non-flatten dict to `GelfRecord.additional_fields`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use serde_gelf::{GelfRecord, GelfLevel};
    ///
    /// let mut extra = BTreeMap::new();
    /// extra.insert("integer".into(),  serde_value::Value::I8(10));
    ///
    /// let rec = GelfRecord::new()
    ///     .extend_additional_fields(extra);
    /// ```
    fn add_additional_fields(self, additional_fields: BTreeMap<Value, Value>) -> Self;
    /// Extend a already flatten dict to `GelfRecord.additional_fields`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use serde_gelf::{GelfRecord, GelfLevel, to_flat_dict};
    ///
    /// let mut extra = BTreeMap::new();
    /// extra.insert("integer".into(),  serde_value::Value::I8(10));
    ///
    /// let rec = GelfRecord::new()
    ///     .extend_additional_fields(to_flat_dict(&extra).unwrap());
    /// ```
    fn extend_additional_fields(self, additional_fields: BTreeMap<Value, Value>) -> Self;
    /// Set `GelfRecord.facility`.
    fn set_facility(self, facility: String) -> Self;
    /// Set `GelfRecord.line`.
    fn set_line(self, line: u32) -> Self;
    /// Set `GelfRecord.file`.
    fn set_file(self, file: String) -> Self;
    /// Set `GelfRecord.full_message`.
    fn set_full_message(self, full_message: Option<String>) -> Self;
}

/// Trait to update [`GelfRecord`](struct.GelfRecord.html) attributes.
pub trait GelfRecordSetter {
    /// Return the `GelfRecord.message` as mutable.
    fn message_mut(&mut self) -> &mut String;
    /// Return the `GelfRecord.timestamp` as mutable.
    fn timestamp_mut(&mut self) -> &mut f64;
    /// Return the `GelfRecord.level` as mutable.
    fn level_mut(&mut self) -> &mut GelfLevel;
    /// Return the `GelfRecord.additional_fields` as mutable.
    fn additional_fields_mut(&mut self) -> &mut BTreeMap<String, Value>;
    /// Return the `GelfRecord.facility` as mutable.
    fn facility_mut(&mut self) -> &mut String;
    /// Return the `GelfRecord.line` as mutable.
    fn line_mut(&mut self) -> &mut u32;
    /// Return the `GelfRecord.full_message` as mutable.
    fn file_mut(&mut self) -> &mut String;
    /// Return the `GelfRecord.full_message` as mutable.
    fn full_message_mut(&mut self) -> &mut Option<String>;
}

/// Trait to access to [`GelfRecord`](struct.GelfRecord.html) attributes.
pub trait GelfRecordGetter {
    /// Return the `GelfRecord.message` attribute.
    fn message(&self) -> String;
    /// Return the `GelfRecord.timestamp` attribute.
    fn timestamp(&self) -> f64;
    /// Return the `GelfRecord.level` attribute.
    fn level(&self) -> GelfLevel;
    /// Return the `GelfRecord.additional_fields` attribute.
    fn additional_fields(&self) -> BTreeMap<Value, Value>;
    /// Return the `GelfRecord.facility` attribute.
    fn facility(&self) -> String;
    /// Return the `GelfRecord.line`attribute.
    fn line(&self) -> u32;
    /// Return the `GelfRecord.file` attribute.
    fn file(&self) -> String;
    /// Return the `GelfRecord.version` attribute.
    fn version() -> &'static str { "1.1" }
    /// Return the `GelfRecord.full_message` attribute.
    fn full_message(&self) -> Option<String> { None }
}

/// Structure which represent a log record.
#[derive(Serialize, Debug, Clone)]
pub struct GelfRecord {
    /// Source of the message that can i.e. the module path which created the log entry.
    facility: String,
    /// The file (with path if you want) that caused the log entry.
    file: String,
    /// The name of the host, source or application that sent this message.
    host: String,
    /// The level equal to the standard syslog levels.
    level: u32,
    #[serde(rename = "_levelname")]
    levelname: String,
    /// The line in a file that caused the log entry.
    line: u32,
    /// A short descriptive message.
    short_message: String,
    /// Seconds since UNIX epoch with optional decimal places for milliseconds.
    #[serde(default = "now")]
    timestamp: f64,
    /// GELF spec version.
    version: String,
    /// A long message that can i.e. contain a backtrace.
    #[serde(skip_serializing_if = "Option::is_none")]
    full_message: Option<String>,
    /// Every field you send and prefix with an underscore (_) will be treated as an additional
    /// field. Allowed characters in field names are any word character (letter, number,
    /// underscore), dashes and dots. The verifying regular expression is: ^[\w\.\-]*$.
    #[serde(flatten)]
    additional_fields: BTreeMap<Value, Value>,
}

/// Default timestamp in seconds since UNIX epoch with optional decimal places for milliseconds.
fn now() -> f64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.as_secs() as f64 + f64::from(now.subsec_nanos()) / 1e9
}

impl GelfRecordBuilder for GelfRecord {
    /// Construct new GelfRecord.
    fn new() -> GelfRecord {
        GelfRecord {
            facility: "main".to_string(),
            file: "main.rs".to_string(),
            host: hostname::get_hostname().unwrap_or("localhost".to_string()),
            level: GelfLevel::Alert as u32,
            levelname: String::from(GelfLevel::Alert),
            line: 0,
            short_message: "".to_string(),
            timestamp: now(),
            version: "1.1".to_string(),
            additional_fields: BTreeMap::new(),
            full_message: None,
        }
    }

    /// Set `GelfRecord.short_message`.
    fn set_message(mut self, short_message: String) -> Self {
        self.short_message = short_message;
        self
    }
    /// Set `GelfRecord.level`.
    fn set_level(mut self, level: GelfLevel) -> Self {
        self.level = level as u32;
        self.levelname = String::from(level);
        self
    }
    /// Set `GelfRecord.timestamp`.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use serde_gelf::GelfRecord;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let rec = GelfRecord::new()
    ///     .set_timestamp({
    ///         let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    ///          now.as_secs() as f64 + f64::from(now.subsec_nanos()) / 1e9
    /// });
    ///
    /// ```
    fn set_timestamp(mut self, timestamp: f64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Extend a already flatten dict to `GelfRecord.additional_fields`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use serde_gelf::{GelfRecord, GelfLevel, to_flat_dict};
    ///
    /// let mut extra = BTreeMap::new();
    /// extra.insert("integer".into(),  serde_value::Value::I8(10));
    ///
    /// let rec = GelfRecord::new()
    ///     .extend_additional_fields(to_flat_dict(&extra).unwrap());
    /// ```
    fn add_additional_fields(mut self, additional_fields: BTreeMap<Value, Value>) -> Self {
        match serde_value::to_value(&additional_fields) {
            Ok(value) => {
                self.additional_fields.extend( to_flat_dict(&value).unwrap());
                self
            }
            Err(_) => self
        }
    }

    /// Extend a already flatten dict to `GelfRecord.additional_fields`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use serde_gelf::{GelfRecord, GelfLevel, to_flat_dict};
    ///
    /// let mut extra = BTreeMap::new();
    /// extra.insert("integer".into(),  serde_value::Value::I8(10));
    ///
    /// let rec = GelfRecord::new()
    ///     .extend_additional_fields(to_flat_dict(&extra).unwrap());
    /// ```
    fn extend_additional_fields(mut self, additional_fields: BTreeMap<Value, Value>) -> Self {
        self.additional_fields.extend(additional_fields);
        self
    }
    /// Set `GelfRecord.facility`.
    fn set_facility(mut self, facility: String) -> Self {
        self.facility = facility;
        self
    }

    /// Set `GelfRecord.line`.
    fn set_line(mut self, line: u32) -> Self {
        self.line = line;
        self
    }

    /// Set `GelfRecord.file`.
    fn set_file(mut self, file: String) -> Self {
        self.file = file;
        self
    }

    /// Set `GelfRecord.full_message`.
    fn set_full_message(mut self, full_message: Option<String>) -> Self {
        self.full_message = full_message;
        self
    }
}

impl GelfRecordGetter for GelfRecord {
    /// Return the `GelfRecord.message` attribute.
    fn message(&self) -> String { self.short_message.clone() }
    /// Return the `GelfRecord.timestamp` attribute.
    fn timestamp(&self) -> f64 { self.timestamp }
    /// Return the `GelfRecord.level` attribute.
    fn level(&self) -> GelfLevel { GelfLevel::from(self.level) }
    /// Return the `GelfRecord.additional_fields` attribute.
    fn additional_fields(&self) -> BTreeMap<Value, Value> { self.additional_fields.clone() }
    /// Return the `GelfRecord.facility` attribute.
    fn facility(&self) -> String { self.facility.clone() }
    /// Return the `GelfRecord.line`attribute.
    fn line(&self) -> u32 { self.line }
    /// Return the `GelfRecord.file` attribute.
    fn file(&self) -> String { self.file.clone() }
    /// Return the `GelfRecord.version` attribute.
    fn version() -> &'static str { "1.1" }
    /// Return the `GelfRecord.full_message` attribute.
    fn full_message(&self) -> Option<String> { self.full_message.clone() }
}

impl<'a> From<&log::Record<'a>> for GelfRecord {
    fn from(record: &log::Record) -> GelfRecord {
        GelfRecord::new()
            .set_facility(record.target().to_string())
            .set_file(record.module_path().unwrap_or("").to_string())
            .set_level(GelfLevel::from(record.level()))
            .set_line(record.line().unwrap_or(0))
            .set_message(format!("{}", record.args()))
    }
}

impl<R: GelfRecordGetter> From<&R> for GelfRecord {
    fn from(record: &R) -> GelfRecord {
        GelfRecord::new()
            .set_file(record.file())
            .set_facility(record.facility())
            .set_level(record.level())
            .set_line(record.line())
            .set_timestamp(record.timestamp())
            .set_message(record.message())
            .set_full_message(record.full_message())
            .add_additional_fields(record.additional_fields())
    }
}
