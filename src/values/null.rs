use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::error::ValueError;
use crate::values::f32::ValueF32;
use crate::values::f64::ValueF64;
use crate::values::i16::ValueI16;
use crate::values::i32::ValueI32;
use crate::values::i64::ValueI64;
use crate::values::i8::ValueI8;
use crate::values::string::ValueString;
use crate::values::u16::ValueU16;
use crate::values::u32::ValueU32;
use crate::values::u64::ValueU64;
use crate::values::u8::ValueU8;
use crate::values::value::{Value, ValueTrait};
use crate::values::vtype::ValueType;

pub struct ValueNull {

}

impl ValueNull {
    pub fn new() -> ValueNull {
        ValueNull {}
    }
}

impl ValueTrait for ValueNull {
    fn is_null(&self) -> bool {
        true
    }

    fn get_type_repr(&self) -> &str {
        "null"
    }

    fn show(&self, end: char) {
        print!("null<>{}", end);
    }

    fn add(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn increment(&mut self) -> Option<Value> {
        None
    }

    fn left_increment(&mut self) -> Option<Value> {
        None
    }

    fn subtract(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn decrement(&mut self) -> Option<Value> {
        None
    }

    fn left_decrement(&mut self) -> Option<Value> {
        None
    }

    fn times(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn unary_not(&self) -> Option<Value> {
        None
    }

    fn bitwise_not(&self) -> Option<Value> {
        None
    }

    fn less_than(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn less_or_equal(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn greater_than(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn greater_or_equal(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn convert(&self, _to: ApicaTypeBytecode) -> Option<Value> {
        None // null is AUTOMATICALLY converted
    }

    fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        match to {
            ApicaTypeBytecode::Any | ApicaTypeBytecode::Null => Some(Value::Null(ValueNull::new())),
            ApicaTypeBytecode::I8 => Some(Value::I8(ValueI8::new())),
            ApicaTypeBytecode::I16 => Some(Value::I16(ValueI16::new())),
            ApicaTypeBytecode::I32 => Some(Value::I32(ValueI32::new())),
            ApicaTypeBytecode::I64 => Some(Value::I64(ValueI64::new())),
            ApicaTypeBytecode::U8 => Some(Value::U8(ValueU8::new())),
            ApicaTypeBytecode::U16 => Some(Value::U16(ValueU16::new())),
            ApicaTypeBytecode::U32 => Some(Value::U32(ValueU32::new())),
            ApicaTypeBytecode::U64 => Some(Value::U64(ValueU64::new())),
            ApicaTypeBytecode::F32 => Some(Value::F32(ValueF32::new())),
            ApicaTypeBytecode::F64 => Some(Value::F64(ValueF64::new())),
            ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::new())),
            ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::new())),
            ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),
            ApicaTypeBytecode::Error => Some(Value::Error(Box::new(ValueError::new()))),
            ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::Null))),

            _ => None,
        }
    }
}