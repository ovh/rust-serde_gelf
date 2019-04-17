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

use serde_value::Value;

struct KeySerializer;

#[cfg(not(feature = "ovh-ldp"))]
impl KeySerializer {
    fn format_key(xpath: &str, key: &str, _value: &Value) -> String {
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

pub struct FlatSerializer;

impl FlatSerializer {
    pub fn disassemble(xpath: &str, key: &str, value: &Value) -> BTreeMap<String, Value> {
        let mut parts = BTreeMap::new();
        match value {
            Value::Map(ref tree) => {
                for (k, v) in tree.iter() {
                    let subkey = match k {
                        Value::String(data) => format!("{}", data),
                        Value::Char(data) => format!("{}", data),
                        _ => panic!("Map keys MUST be strings or char")
                    };
                    parts.append(&mut Self::disassemble(&KeySerializer::format_key(xpath, &key, value), &subkey, v));
                };
            }
            Value::Seq(ref values) => {
                for (i, val) in values.iter().enumerate() {
                    parts.append(&mut Self::disassemble(&mut KeySerializer::format_key(xpath, key, value), &format!("{}", i), val));
                }
            }
            _ => {
                parts.insert(KeySerializer::format_key(xpath, key, value), value.clone());
            }
        };
        parts
    }
}
