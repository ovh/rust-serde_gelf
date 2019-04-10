use std::collections::BTreeMap;

use serde_value::Value;

struct KeySerializer;

#[cfg(not(feature = "ovh-ldp"))]
impl KeySerializer {
    fn format_key(xpath: &str, key: &str, value: &Value) -> String {
        match (xpath, key) {
            (_, "") => String::new(),
            ("", k) => format!("_{}", k),
            (x, k) => format!("{}_{}", x, k)
        }
    }
}

#[cfg(feature = "ovh-ldp")]
impl KeySerializer {
    fn _schema_suffix(value: &Value) -> &str {
        match *value {
            Value::Bool(_) => "_bool",
            Value::U8(_) | Value::U16(_) | Value::U32(_) | Value::U64(_) => "_double",
            Value::I8(_) | Value::I16(_) | Value::I32(_) | Value::I64(_) => "_long",
            Value::F32(_) | Value::F64(_) => "_float",
            _ => ""
        }
    }
    fn format_key(xpath: &str, key: &str, value: &Value) -> String {
        match (xpath, key) {
            (_, "") => String::new(),
            ("", k) => format!("_{}{}", k, KeySerializer::_schema_suffix(value)),
            (x, k) => format!("{}_{}{}", x, k, KeySerializer::_schema_suffix(value)),
        }
    }
}

#[derive(Debug)]
pub struct GelfField<'a> {
    xpath: &'a str,
    key: &'a str,
    value: Value,
}

impl<'a> GelfField<'a> {
    pub fn new(xpath: &'a str, key: &'a str, value: &Value) -> GelfField<'a> {
        GelfField { xpath, key, value: value.clone() }
    }
    pub fn disassemble(&self) -> BTreeMap<String, Value> {
        let mut parts = BTreeMap::new();
        match self.value {
            Value::Map(ref tree) => {
                for (k, v) in tree.iter() {
                    let key = match k {
                        Value::String(data) => format!("{}", data),
                        Value::Char(data) => format!("{}", data),
                        _ => panic!("Map keys MUST be strings, bytes or char")
                    };
                    parts.append(&mut GelfField::new(
                        &KeySerializer::format_key(self.xpath, self.key, &self.value),
                        &key,
                        v,
                    ).disassemble());
                };
            }
            Value::Seq(ref values) => {
                for (i, val) in values.iter().enumerate() {
                    parts.append(&mut GelfField::new(
                        &KeySerializer::format_key(self.xpath, self.key, &self.value),
                        &format!("{}", i),
                        val,
                    ).disassemble());
                }
            }
            _ => {
                parts.insert(
                    KeySerializer::format_key(self.xpath, self.key, &self.value),
                    self.value.clone(),
                );
            }
        };
        parts
    }
}



