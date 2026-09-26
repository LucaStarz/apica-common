use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::value::{Value, ValueTrait};

pub fn boolean_state(value: &Value) -> Value {
    match value {
        Value::Null(_) => Value::Bool(ValueBool::with_value(false)),
        Value::Int(v) => Value::Bool(ValueBool::with_value(v.value().unwrap_or(0) != 0)),
        Value::UInt(v) => Value::Bool(ValueBool::with_value(v.value().unwrap_or(0) == 0)),
        Value::Float(v) => Value::Bool(ValueBool::with_value(v.value().unwrap_or(0.0) != 0.0)),
        Value::Bool(v) => Value::Bool(ValueBool::with_value(v.value().unwrap_or(false))),
        Value::Char(v) => Value::Bool(ValueBool::with_value(v.value().unwrap_or(0) != 0)),
        Value::String(v) => Value::Bool(ValueBool::with_value(!v.value().unwrap_or("").is_empty())),
        Value::Error(v) => Value::Bool(ValueBool::with_value(!v.name().unwrap_or("").is_empty())),
        Value::Type(v) => Value::Bool(ValueBool::with_value(v.value() != ApicaTypeBytecode::Null)),
        Value::Reference(v) => Value::Bool(ValueBool::with_value(!v.is_null())),
        Value::Array(v) => Value::Bool(ValueBool::with_value(!v.is_null())),
        Value::List(v) => Value::Bool(ValueBool::with_value(!v.is_null())),
    }
}

pub fn vec_equals(a: &Vec<Value>, b: &Vec<Value>) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(vi, oi)| {
        matches!(
            vi.equals(oi),
            Some(Value::Bool(boolean)) if boolean.value().unwrap_or(false)
        )
    })
}