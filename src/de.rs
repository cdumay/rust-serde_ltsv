use std::collections::BTreeMap;

use serde::Deserialize;
use serde_value::Value;

use crate::result::{LtsvError, LtsvResult};

fn try_detect_type(raw: &str) -> Value {
    if let Ok(data) = raw.parse::<bool>() {
        return Value::Bool(data);
    }
    if let Ok(data) = raw.parse::<u64>() {
        return Value::U64(data);
    }
    if let Ok(data) = raw.parse::<i64>() {
        return Value::I64(data);
    }
    if let Ok(data) = raw.parse::<f64>() {
        return Value::F64(data);
    }
    Value::String(raw.to_string())
}

/// Deserialize an instance of type `T` from a string of LTSV text.
///
/// # Example
///
/// ```rust
/// #[derive(Deserialize, Debug)]
/// struct Foo {
///    a: String,
///    b: i8,
///    c: bool,
/// }
/// let line = "a:Test\tb:8\tc:false";
/// let foo: Foo = serde_ltsv::from_str(&line).unwrap();
/// println!("{:?}", &foo);
/// ```
/// **Output**:
/// ```text
/// Foo { a: "Test", b: 8, c: false }
/// ```
#[inline]
pub fn from_str<'a, T>(value: &'a str) -> LtsvResult<T> where T: Deserialize<'a> {
    let mut out = BTreeMap::new();
    for part in value.split('\t') {
        let mut pair = part.splitn(2, ':');
        let k = pair.next();
        let v = pair.next();
        match (k, v) {
            (None, None) => return Err(LtsvError::InvalidInput("Missing name and value for a LTSV record".into())),
            (Some(_), None) | (None, Some(_)) => return Err(LtsvError::InvalidInput(format!("Invalid input: [{:?}]", &value))),
            (Some(name), Some(value)) => {
                out.insert(Value::String(name.into()), try_detect_type(value));
            }
        };
    }
    Ok(Value::Map(out).deserialize_into()?)
}
