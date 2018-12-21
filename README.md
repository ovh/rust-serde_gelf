# serde_gelf

[![Build Status](https://travis-ci.org/cdumay/serde_gelf.svg?branch=master)](https://travis-ci.org/cdumay/serde_gelf)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](./LICENSE)

Gelf serialization using serde

## Quickstart

Add in your `Cargo.toml`:
```toml
[dependencies]
serde_gelf = "0.1"
serde_derive = "1.0"
serde-value = "0.5"
```

Create a structure which implement the `Serialize` trait: 
```rust
extern crate serde_gelf;
extern crate serde_value;

#[macro_use]
extern crate serde_derive;

use serde_value::Value;
use std::collections::HashMap;

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
    d: HashMap<String, Value>,
    e: SubFoo,
}

fn main() {
    let foo = Foo {
        a: 15,
        b: "hello".into(),
        c: vec![true, false],
        d: {
            let mut map = HashMap::new();
            map.insert("k1".to_string(), Value::F64(5.9));
            map.insert("k2".to_string(), Value::Bool(false));
            map
        },
        e: SubFoo { sa: "test".to_string(), sb: 5 },
    };
    println!("{}", to_string_pretty(&foo).unwrap());
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

To send special type like number or boolean, LDP uses suffixes to force ES type:

| Suffix            | ES Type  | About                                                                                                                              |
|-------------------|----------|------------------------------------------------------------------------------------------------------------------------------------|
| _double           | double   | Unsigned number                                                                                                                    |
| _float            | double   | Floating value in double in java representation : double-precision 64-bit IEEE 754 floating point                                  |
| _long             | long     | 64 bit signed long type,which has a minimum value of -263 and a maximum value of 263-1                                             |
| _bool             | boolean  | Expected values: "true" or "false".WARNING : GELF does not support boolean types you will have to send "true" or "false" in String |
| Everything else   | String   | Anything else will be considered a string                                                                                          |

To enable suffixes, update `Cargo.toml` and set the *ldp* feature:
```toml
serde_gelf = { version = "0.1", features = ["ldp"] }
# or
[dependencies.serde_gelf]
version = "0.1"
features = ["ldp"]
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

## License

Licensed under [MIT license LICENSE](./LICENSE) or (http://opensource.org/licenses/MIT)