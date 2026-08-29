use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::f32::ValueF32;
use crate::values::f64::ValueF64;
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

pub struct ValueI16 {
    value: Option<i16>,
}

impl ValueI16 {
    pub fn new() -> ValueI16 {
        ValueI16 { value: None }
    }

    pub fn with_value(value: i16) -> ValueI16 {
        ValueI16 { value: Some(value) }
    }

    pub fn value(&self) -> Option<i16> {
        self.value
    }
}

impl ValueTrait for ValueI16 {
    fn is_null(&self) -> bool {
        self.value.is_none()
    }

    fn get_type_repr(&self) -> &str {
        "i16"
    }

    fn show(&self, end: char) {
        match self.value {
            Some(v) => print!("i16<{}>{}", v, end),
            None => print!("i16<>{}", end),
        }
    }

    fn add(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() + v.value().unwrap() as i16
            ))),

            Value::I16(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() + v.value().unwrap()
            ))),

            Value::I32(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 + v.value().unwrap()
            ))),

            Value::I64(v) => Some(Value::I64(ValueI64::with_value(
                self.value.unwrap() as i64 + v.value().unwrap()
            ))),

            Value::U8(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() + v.value().unwrap() as i16
            ))),

            Value::U16(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() + v.value().unwrap() as i16
            ))),

            Value::U32(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 + v.value().unwrap() as i32
            ))),

            Value::U64(v) => Some(Value::I64(ValueI64::with_value(
                self.value.unwrap() as i64 + v.value().unwrap() as i64
            ))),

            Value::F32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() as f32 + v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 + v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() + v.value().unwrap() as i16
            ))),

            Value::Char(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 + v.value().unwrap() as i32
            ))),

            _ => None,
        }
    }

    fn increment(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        let old_value = *val_ref;
        *val_ref += 1;

        Some(Value::I16(ValueI16::with_value(old_value)))
    }

    fn left_increment(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        *val_ref += 1;

        Some(Value::I16(ValueI16::with_value(*val_ref)))
    }

    fn subtract(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() - v.value().unwrap() as i16
            ))),

            Value::I16(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() - v.value().unwrap()
            ))),

            Value::I32(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 - v.value().unwrap()
            ))),

            Value::I64(v) => Some(Value::I64(ValueI64::with_value(
                self.value.unwrap() as i64 - v.value().unwrap()
            ))),

            Value::U8(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() - v.value().unwrap() as i16
            ))),

            Value::U16(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() - v.value().unwrap() as i16
            ))),

            Value::U32(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 - v.value().unwrap() as i32
            ))),

            Value::U64(v) => Some(Value::I64(ValueI64::with_value(
                self.value.unwrap() as i64 - v.value().unwrap() as i64
            ))),

            Value::F32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() as f32 - v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 - v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() - v.value().unwrap() as i16
            ))),

            Value::Char(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 - v.value().unwrap() as i32
            ))),

            _ => None,
        }
    }

    fn decrement(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        let old_value = *val_ref;
        *val_ref -= 1;
        
        Some(Value::I16(ValueI16::with_value(old_value)))
    }

    fn left_decrement(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        *val_ref -= 1;
        
        Some(Value::I16(ValueI16::with_value(*val_ref)))
    }

    fn times(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 * v.value().unwrap() as i32
            ))),

            Value::I16(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 * v.value().unwrap() as i32
            ))),

            Value::I32(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 * v.value().unwrap()
            ))),

            Value::I64(v) => Some(Value::I64(ValueI64::with_value(
                self.value.unwrap() as i64 * v.value().unwrap()
            ))),

            Value::U8(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 * v.value().unwrap() as i32
            ))),

            Value::U16(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 * v.value().unwrap() as i32
            ))),

            Value::U32(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 * v.value().unwrap() as i32
            ))),

            Value::U64(v) => Some(Value::I64(ValueI64::with_value(
                self.value.unwrap() as i64 * v.value().unwrap() as i64
            ))),

            Value::F32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() as f32 * v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 * v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::I16(ValueI16::with_value(
                self.value.unwrap() * v.value().unwrap() as i16
            ))),

            Value::Char(v) => Some(Value::I32(ValueI32::with_value(
                self.value.unwrap() as i32 * v.value().unwrap() as i32
            ))),

            _ => None,
        }
    }

    fn unary_not(&self) -> Option<Value> {
        Some(Value::Bool(ValueBool::with_value(
            match self.value {
                Some(v) => v == 0,
                None => true,
            }
        )))
    }

    fn bitwise_not(&self) -> Option<Value> {
        Some(Value::I16(ValueI16::with_value(
            !self.value.unwrap()
        )))
    }

    fn less_than(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as i16
            ))),

            Value::I16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap()
            ))),

            Value::I32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i32) < v.value().unwrap()
            ))),

            Value::I64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i64) < v.value().unwrap()
            ))),

            Value::U8(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u16) < v.value().unwrap() as u16
            ))),

            Value::U16(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u16) < v.value().unwrap()
            ))),

            Value::U32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) < v.value().unwrap()
            ))),

            Value::U64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u64) < v.value().unwrap()
            ))),

            Value::F32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f32) < v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) < v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as i16
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) < v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn less_or_equal(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as i16
            ))),

            Value::I16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap()
            ))),

            Value::I32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i32) <= v.value().unwrap()
            ))),

            Value::I64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i64) <= v.value().unwrap()
            ))),

            Value::U8(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u16) <= v.value().unwrap() as u16
            ))),

            Value::U16(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u16) <= v.value().unwrap()
            ))),

            Value::U32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) <= v.value().unwrap()
            ))),

            Value::U64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u64) <= v.value().unwrap()
            ))),

            Value::F32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f32) <= v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) <= v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as i16
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) <= v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn greater_than(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as i16
            ))),

            Value::I16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap()
            ))),

            Value::I32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i32) > v.value().unwrap()
            ))),

            Value::I64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i64) > v.value().unwrap()
            ))),

            Value::U8(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u16) > v.value().unwrap() as u16
            ))),

            Value::U16(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u16) > v.value().unwrap()
            ))),

            Value::U32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) > v.value().unwrap()
            ))),

            Value::U64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u64) > v.value().unwrap()
            ))),

            Value::F32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f32) > v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) > v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as i16
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) > v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn greater_or_equal(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as i16
            ))),

            Value::I16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap()
            ))),

            Value::I32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i32) >= v.value().unwrap()
            ))),

            Value::I64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i64) >= v.value().unwrap()
            ))),

            Value::U8(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u16) >= v.value().unwrap() as u16
            ))),

            Value::U16(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u16) >= v.value().unwrap()
            ))),

            Value::U32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) >= v.value().unwrap()
            ))),

            Value::U64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u64) >= v.value().unwrap()
            ))),

            Value::F32(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f32) >= v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) >= v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as i16
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) >= v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = self.value {
            match to { 
                ApicaTypeBytecode::String => Some(Value::String(ValueString::with_value(value.to_string()))),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::I16))),
                
                _ => None,
            }
        } else {
            match to { 
                ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::Type))),
                
                _ => None,
            }
        }
    }

    fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = self.value {
            match to { 
                ApicaTypeBytecode::Any | ApicaTypeBytecode::I16 => Some(Value::I16(ValueI16::with_value(value))),
                ApicaTypeBytecode::I8 => Some(Value::I8(ValueI8::with_value(value as i8))),
                ApicaTypeBytecode::I32 => Some(Value::I32(ValueI32::with_value(value as i32))),
                ApicaTypeBytecode::I64 => Some(Value::I64(ValueI64::with_value(value as i64))),
                ApicaTypeBytecode::U8 => Some(Value::U8(ValueU8::with_value(value as u8))),
                ApicaTypeBytecode::U16 => Some(Value::U16(ValueU16::with_value(value as u16))),
                ApicaTypeBytecode::U32 => Some(Value::U32(ValueU32::with_value(value as u32))),
                ApicaTypeBytecode::U64 => Some(Value::U64(ValueU64::with_value(value as u64))),
                ApicaTypeBytecode::F32 => Some(Value::F32(ValueF32::with_value(value as f32))),
                ApicaTypeBytecode::F64 => Some(Value::F64(ValueF64::with_value(value as f64))),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(value != 0))),
                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::with_value(value as u32))),
                
                _ => None,
            }
        } else {
            match to { 
                ApicaTypeBytecode::Any | ApicaTypeBytecode::I16 => Some(Value::I16(ValueI16::new())),
                ApicaTypeBytecode::I8 => Some(Value::I8(ValueI8::new())),
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
                
                _ => None,
            }
        }
    }

    fn copy(&self) -> Value {
        match self.value { 
            Some(val) => Value::I16(ValueI16::with_value(val)),
            None => Value::I16(ValueI16::new()),
        }
    }
}