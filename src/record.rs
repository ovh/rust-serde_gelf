use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_value::Value;

use crate::level::GelfLevel;
use crate::ser::GelfField;

pub trait GelfRecordBuilder {
    fn new() -> Self;
    fn set_message(self, short_message: String) -> Self;
    fn set_level(self, level: GelfLevel) -> Self;
    fn set_timestamp(self, timestamp: f64) -> Self;
    fn add_additional_fields(self, additional_fields: BTreeMap<String, Value>) -> Self;
    fn extend_additional_fields(self, additional_fields: BTreeMap<String, serde_value::Value>) -> Self;
    fn set_facility(self, facility: String) -> Self;
    fn set_line(self, line: u32) -> Self;
    fn set_file(self, line: String) -> Self;
    fn set_full_message(self, full_message: Option<String>) -> Self;
}

pub trait GelfRecordSetter {
    fn message_mut(&mut self) -> &mut String;
    fn timestamp_mut(&mut self) -> &mut f64;
    fn level_mut(&mut self) -> &mut GelfLevel;
    fn additional_fields_mut(&mut self) -> &mut BTreeMap<String, Value>;
    fn facility_mut(&mut self) -> &mut String;
    fn line_mut(&mut self) -> &mut u32;
    fn file_mut(&mut self) -> &mut String;
    fn full_message_mut(&mut self) -> &mut Option<String>;
}

pub trait GelfRecordGetter {
    fn message(&self) -> String;
    fn timestamp(&self) -> f64;
    fn level(&self) -> GelfLevel;
    fn additional_fields(&self) -> BTreeMap<String, Value>;
    fn facility(&self) -> String;
    fn line(&self) -> u32;
    fn file(&self) -> String;
    fn version() -> &'static str { "1.1" }
    fn full_message(&self) -> Option<String> { None }
}


#[derive(Serialize, Debug, Clone)]
pub struct GelfRecord {
    facility: String,
    file: String,
    host: String,
    level: u32,
    #[serde(rename = "_levelname")]
    levelname: String,
    line: u32,
    short_message: String,
    #[serde(default = "now")]
    timestamp: f64,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_message: Option<String>,
    #[serde(flatten)]
    additional_fields: BTreeMap<String, serde_value::Value>,
}

pub fn now() -> f64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.as_secs() as f64 + f64::from(now.subsec_nanos()) / 1e9
}

impl GelfRecordBuilder for GelfRecord {
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

    fn set_message(mut self, short_message: String) -> Self {
        self.short_message = short_message;
        self
    }
    fn set_level(mut self, level: GelfLevel) -> Self {
        self.level = level as u32;
        self.levelname = String::from(level);
        self
    }

    fn set_timestamp(mut self, timestamp: f64) -> Self {
        self.timestamp = timestamp;
        self
    }

    fn add_additional_fields(mut self, additional_fields: BTreeMap<String, serde_value::Value>) -> Self {
        match serde_value::to_value(&additional_fields) {
            Ok(value) => {
                self.additional_fields.extend(GelfField::new("", "", &value).disassemble());
                self
            }
            Err(_) => self
        }
    }

    fn extend_additional_fields(mut self, additional_fields: BTreeMap<String, serde_value::Value>) -> Self {
        self.additional_fields.extend(additional_fields);
        self
    }
    fn set_facility(mut self, facility: String) -> Self {
        self.facility = facility;
        self
    }

    fn set_line(mut self, line: u32) -> Self {
        self.line = line;
        self
    }

    fn set_file(mut self, file: String) -> Self {
        self.file = file;
        self
    }

    fn set_full_message(mut self, full_message: Option<String>) -> Self {
        self.full_message = full_message;
        self
    }
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
