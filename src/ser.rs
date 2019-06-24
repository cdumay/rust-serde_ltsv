use std::collections::BTreeMap;
use std::ops::Deref;

use serde::Serialize;
use serde_value::Value;

use crate::result::{LtsvError, LtsvResult};

struct LtsvSerializer;

impl LtsvSerializer {
    fn key_to_string(value: &Value) -> LtsvResult<String> {
        match value {
            Value::Seq(_) => Err(LtsvError::InvalidInput(format!("Key {:?} cannot be a sequence", value))),
            Value::Newtype(_) => Err(LtsvError::InvalidInput(format!("Key {:?} cannot be an object", value))),
            Value::Unit => Err(LtsvError::InvalidInput(format!("Key {:?} cannot be a Unit", value))),
            Value::Map(_) => Err(LtsvError::InvalidInput(format!("Key {:?} cannot be a map", value))),
            Value::Option(_) => Err(LtsvError::InvalidInput(format!("Key {:?} cannot be an option", value))),
            Value::Bool(_) => Err(LtsvError::InvalidInput(format!("Key {:?} cannot be a boolean", value))),
            Value::String(data) => Ok(data.clone()),
            Value::U8(data) => Ok(format!("{}", data)),
            Value::U16(data) => Ok(format!("{}", data)),
            Value::U32(data) => Ok(format!("{}", data)),
            Value::U64(data) => Ok(format!("{}", data)),
            Value::I8(data) => Ok(format!("{}", data)),
            Value::I16(data) => Ok(format!("{}", data)),
            Value::I32(data) => Ok(format!("{}", data)),
            Value::I64(data) => Ok(format!("{}", data)),
            Value::F32(data) => Ok(format!("{}", data)),
            Value::F64(data) => Ok(format!("{}", data)),
            Value::Bytes(data) => Ok(String::from_utf8(data.clone())?),
            Value::Char(data) => Ok(format!("{}", data)),
        }
    }
    fn value_to_string(value: &Value) -> LtsvResult<String> {
        match value {
            Value::Seq(_) => Err(LtsvError::InvalidInput(format!("Value cannot be a sequence ({:?})", value))),
            Value::Newtype(_) => Err(LtsvError::InvalidInput(format!("Value cannot be an object ({:?})", value))),
            Value::Unit => Err(LtsvError::InvalidInput(format!("Value cannot be a Unit ({:?})", value))),
            Value::Map(_) => Err(LtsvError::InvalidInput(format!("Value cannot be a map ({:?})", value))),
            Value::Option(_) => Err(LtsvError::InvalidInput(format!("Value cannot be an option ({:?})", value))),
            Value::String(data) => Ok(data.clone()),
            Value::U8(data) => Ok(format!("{}", data)),
            Value::U16(data) => Ok(format!("{}", data)),
            Value::U32(data) => Ok(format!("{}", data)),
            Value::U64(data) => Ok(format!("{}", data)),
            Value::I8(data) => Ok(format!("{}", data)),
            Value::I16(data) => Ok(format!("{}", data)),
            Value::I32(data) => Ok(format!("{}", data)),
            Value::I64(data) => Ok(format!("{}", data)),
            Value::F32(data) => Ok(format!("{}", data)),
            Value::F64(data) => Ok(format!("{}", data)),
            Value::Bytes(data) => Ok(String::from_utf8(data.clone())?),
            Value::Char(data) => Ok(format!("{}", data)),
            Value::Bool(data) => Ok(format!("{}", data)),
        }
    }
    fn from_map(data: &BTreeMap<Value, Value>) -> LtsvResult<String> {
        let mut out: Vec<String> = Vec::new();
        for (k, v) in data.iter() {
            out.push(format!(
                "{}:{}",
                LtsvSerializer::key_to_string(k)?,
                LtsvSerializer::value_to_string(v)?
            ));
        }
        Ok(out.join("\t"))
    }
    fn from_sequence(data: &Vec<Value>) -> LtsvResult<String> {
        let mut out: Vec<String> = Vec::new();

        for (idx, value) in data.iter().enumerate() {
            out.push(format!(
                "{}:{}",
                LtsvSerializer::key_to_string(&Value::U64(idx as u64))?,
                LtsvSerializer::value_to_string(value)?
            ));
        }
        Ok(out.join("\t"))
    }
    fn from_object(data: &Box<Value>) -> LtsvResult<String> {
        match data.deref() {
            Value::Map(map) => LtsvSerializer::from_map(map),
            _ => Err(LtsvError::InvalidInput(format!("Invalid object {:?}", data))),
        }
    }
}

/// Serialize the given data structure as a String of LTSV.
///
/// # Example
///
/// ```rust
/// #[derive(Serialize)]
/// struct Foo {
///    a: String,
///    b: i8,
///    c: bool,
/// }
///
/// let foo = Foo { a: "Test".into(), b:8, c: false };
/// println!("{}", serde_ltsv::to_string(&foo).unwrap());
/// ```
/// **Output**:
/// ```text
/// a:Test	b:8	c:false
/// ```
#[inline]
pub fn to_string<T: ?Sized>(value: &T) -> LtsvResult<String> where T: Serialize {
    let obj = serde_value::to_value(value)?;
    match obj {
        Value::Map(data) => LtsvSerializer::from_map(&data),
        Value::Newtype(data) => Ok(LtsvSerializer::from_object(&data)?),
        Value::Seq(data) => Ok(LtsvSerializer::from_sequence(&data)?),
        _ => Err(LtsvError::InvalidInput("Invalid value MUST be a Map, an object or a sequence".into()))
    }
}