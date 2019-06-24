# serde_gelf 

[![Build Status](https://travis-ci.org/ovh/rust-serde_gelf.svg?branch=master)](https://travis-ci.org/ovh/rust-serde_gelf) 
[![Latest version](https://img.shields.io/crates/v/serde_gelf.svg)](https://crates.io/crates/serde_gelf)
[![Documentation](https://docs.rs/serde_gelf/badge.svg)](https://docs.rs/serde_gelf) 
![License](https://img.shields.io/crates/l/serde_gelf.svg)

The Graylog Extended Log Format ([GELF](http://docs.graylog.org/en/latest/pages/gelf.html#gelf-payload-specification)) is a structured log format 
representation which can be sent over network as a JSON string.

## Quickstart

Add in your `Cargo.toml`:
```toml
[dependencies]
serde-value = "0.6"
serde_derive = "1.0"
serde_gelf = "0.1"
serde_json = "1.0"
```

Create a structure which implement the `Serialize` trait: 
```rust
#[macro_use]
extern crate serde_derive;
extern crate serde_gelf;
extern crate serde_json;
extern crate serde_value;

use std::collections::BTreeMap;

#[derive(Serialize)]
struct SubFoo {
    sa: String,
    sb: isize,
}

#[derive(Serialize)]
struct Foo {
    a: u32,
    b: String,
    c: Vec<bool>,
    d: BTreeMap<String, serde_value::Value>,
    e: SubFoo,
}

fn main() {
    let foo = Foo {
        a: 15,
        b: "hello".into(),
        c: vec![true, false],
        d: {
            let mut map = BTreeMap::new();
            map.insert("k1".to_string(), serde_value::Value::F64(5.9));
            map.insert("k2".to_string(), serde_value::Value::Bool(false));
            map
        },
        e: SubFoo { sa: "test".to_string(), sb: 5 },
    };
    println!("{}", serde_json::to_string_pretty(& serde_gelf::to_flat_dict(&foo).unwrap()).unwrap());
}
```
**Output**:
```json
{
  "_a": 15,
  "_b": "hello",
  "_c_0": true,
  "_c_1": false,
  "_d_k1": 5.9,
  "_d_k2": false,
  "_e_sa": "test",
  "_e_sb": 5
}
```

## OVH Logs Data Platform (LDP)

To send special type like number or boolean, [LDP](https://docs.ovh.com/gb/en/logs-data-platform/) uses suffixes as [naming convention](https://docs.ovh.com/gb/en/logs-data-platform/field-naming-conventions/) to force ES type:

| Suffix            | ES Type  | About                                                                                                                              |
|-------------------|----------|------------------------------------------------------------------------------------------------------------------------------------|
| _double           | double   | Unsigned number                                                                                                                    |
| _float            | double   | Floating value in double in java representation : double-precision 64-bit IEEE 754 floating point                                  |
| _long             | long     | 64 bit signed long type,which has a minimum value of -263 and a maximum value of 263-1                                             |
| _bool             | boolean  | Expected values: "true" or "false".WARNING : GELF does not support boolean types you will have to send "true" or "false" in String |
| Everything else   | String   | Anything else will be considered a string                                                                                          |

To enable suffixes, update `Cargo.toml` and set the *ovh-ldp* feature:
```toml
serde_gelf = { version = "0.1", features = ["ovh-ldp"] }
# or
[dependencies.serde_gelf]
version = "0.1"
features = ["ovh-ldp"]
```
Now the output of the previous example will be:
```json
{
  "_a_double": 15,
  "_b": "hello",
  "_c_0_bool": true,
  "_c_1_bool": false,
  "_d_k1_float": 5.9,
  "_d_k2_bool": false,
  "_e_sa": "test",
  "_e_sb_long": 5
}
```

## Macros

This library provides a macro `gelf_record!` to create a gelf record according 
to the GELF Payload Specification.

To enable macros, just activate the macros on crate import:

```rust
#[macro_use]
extern crate serde_gelf;
extern crate serde_json;

fn main() {
    let rec = gelf_record!("hello");
    println!("{}", serde_json::to_string_pretty(&rec).unwrap());
}
```
**Output**:
```json
{
  "facility": "src",
  "file": "examples/src/main.rs",
  "host": "myhostname",
  "level": 1,
  "_levelname": "Alert",
  "line": 11,
  "short_message": "hello",
  "timestamp": 1554980878.241851,
  "version": "1.1"
}
```

## License

Licensed under [BSD 3-Clause License](./LICENSE) or (https://opensource.org/licenses/BSD-3-Clause)