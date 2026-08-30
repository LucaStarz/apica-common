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
use crate::values::value_type::ValueType;

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

    fn assign(&mut self, _other: &Value) -> Option<Value> {
        Some(self.copy())
    }
    
    fn convert(&self, _to: ApicaTypeBytecode) -> Option<Value> {
        None // null is AUTOMATICALLY converted
    }

    fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        Some(match to {
            ApicaTypeBytecode::Any | ApicaTypeBytecode::Null => Value::Null(ValueNull::new()),
            ApicaTypeBytecode::I8 => Value::I8(ValueI8::new()),
            ApicaTypeBytecode::I16 => Value::I16(ValueI16::new()),
            ApicaTypeBytecode::I32 => Value::I32(ValueI32::new()),
            ApicaTypeBytecode::I64 => Value::I64(ValueI64::new()),
            ApicaTypeBytecode::U8 => Value::U8(ValueU8::new()),
            ApicaTypeBytecode::U16 => Value::U16(ValueU16::new()),
            ApicaTypeBytecode::U32 => Value::U32(ValueU32::new()),
            ApicaTypeBytecode::U64 => Value::U64(ValueU64::new()),
            ApicaTypeBytecode::F32 => Value::F32(ValueF32::new()),
            ApicaTypeBytecode::F64 => Value::F64(ValueF64::new()),
            ApicaTypeBytecode::Bool => Value::Bool(ValueBool::new()),
            ApicaTypeBytecode::Char => Value::Char(ValueChar::new()),
            ApicaTypeBytecode::String => Value::String(ValueString::new()),
            ApicaTypeBytecode::Error => Value::Error(Box::new(ValueError::new())),
            ApicaTypeBytecode::Type => Value::Type(ValueType::with_type(ApicaTypeBytecode::Null)),
        })
    }

    fn copy(&self) -> Value {
        Value::Null(ValueNull::new())
    }
}