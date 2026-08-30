use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
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

pub struct ValueF32 {
    value: Option<f32>,
}

impl ValueF32 {
    pub fn new() -> ValueF32 {
        ValueF32 { value: None }
    }

    pub fn with_value(value: f32) -> ValueF32 {
        ValueF32 { value: Some(value) }
    }

    pub fn value(&self) -> Option<f32> {
        self.value
    }
}

impl ValueTrait for ValueF32 {
    fn is_null(&self) -> bool {
        self.value.is_none()
    }

    fn get_type_repr(&self) -> &str {
        "f32"
    }

    fn show(&self, end: char) {
        match self.value {
            Some(v) => print!("f32<{}>{}", v, end),
            None => print!("f32<>{}", end),
        }
    }

    fn add(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() + v.value().unwrap() as f32
            ))),

            Value::I16(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() + v.value().unwrap() as f32
            ))),

            Value::I32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() + v.value().unwrap() as f32
            ))),

            Value::I64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 + v.value().unwrap() as f64
            ))),
            
            Value::U8(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() + v.value().unwrap() as f32
            ))),
            
            Value::U16(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() + v.value().unwrap() as f32
            ))),
            
            Value::U32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() + v.value().unwrap() as f32
            ))),
            
            Value::U64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 + v.value().unwrap() as f64
            ))),
            
            Value::F32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() + v.value().unwrap()
            ))),
            
            Value::F64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 + v.value().unwrap()
            ))),
            
            Value::Bool(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() + v.value().unwrap() as u8 as f32
            ))),
            
            Value::Char(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() + v.value().unwrap() as f32
            ))),

            _ => None,
        }
    }

    fn increment(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        let old_value = *val_ref;
        *val_ref += 1.0;

        Some(Value::F32(ValueF32::with_value(old_value)))
    }

    fn left_increment(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        *val_ref += 1.0;

        Some(Value::F32(ValueF32::with_value(*val_ref)))
    }

    fn subtract(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() - v.value().unwrap() as f32
            ))),
            
            Value::I16(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() - v.value().unwrap() as f32
            ))),
            
            Value::I32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() - v.value().unwrap() as f32
            ))),
            
            Value::I64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 - v.value().unwrap() as f64
            ))),
            
            Value::U8(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() - v.value().unwrap() as f32
            ))),
            
            Value::U16(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() - v.value().unwrap() as f32
            ))),
            
            Value::U32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() - v.value().unwrap() as f32
            ))),
            
            Value::U64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 - v.value().unwrap() as f64
            ))),
            
            Value::F32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() - v.value().unwrap()
            ))),
            
            Value::F64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 - v.value().unwrap()
            ))),
            
            Value::Bool(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() - v.value().unwrap() as u8 as f32
            ))),
            
            Value::Char(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() - v.value().unwrap() as f32
            ))),
            
            _ => None,
        }
    }

    fn decrement(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        let old_value = *val_ref;
        *val_ref -= 1.0;
        
        Some(Value::F32(ValueF32::with_value(old_value)))
    }

    fn left_decrement(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        *val_ref -= 1.0;
        
        Some(Value::F32(ValueF32::with_value(*val_ref)))
    }

    fn times(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() * v.value().unwrap() as f32
            ))),

            Value::I16(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() * v.value().unwrap() as f32
            ))),

            Value::I32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() * v.value().unwrap() as f32
            ))),

            Value::I64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 * v.value().unwrap() as f64
            ))),

            Value::U8(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() * v.value().unwrap() as f32
            ))),

            Value::U16(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() * v.value().unwrap() as f32
            ))),

            Value::U32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() * v.value().unwrap() as f32
            ))),

            Value::U64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 * v.value().unwrap() as f64
            ))),

            Value::F32(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() * v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::F64(ValueF64::with_value(
                self.value.unwrap() as f64 * v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() * v.value().unwrap() as u8 as f32
            ))),

            Value::Char(v) => Some(Value::F32(ValueF32::with_value(
                self.value.unwrap() * v.value().unwrap() as f32
            ))),

            _ => None,
        }
    }

    fn unary_not(&self) -> Option<Value> {
        Some(Value::Bool(ValueBool::with_value(
            match self.value { 
                Some(v) => v == 0.0,
                None => true,
            }
        )))
    }

    fn bitwise_not(&self) -> Option<Value> {
        None
    }

    fn less_than(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as f32
            ))),

            Value::I16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as f32
            ))),

            Value::I32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as f32
            ))),

            Value::I64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) < v.value().unwrap() as f64
            ))),

            Value::U8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as f32
            ))),

            Value::U16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as f32
            ))),

            Value::U32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as f32
            ))),

            Value::U64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) < v.value().unwrap() as f64
            ))),

            Value::F32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) < v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as u8 as f32
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as f32
            ))),

            _ => None,
        }
    }

    fn less_or_equal(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as f32
            ))),

            Value::I16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as f32
            ))),

            Value::I32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as f32
            ))),

            Value::I64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) <= v.value().unwrap() as f64
            ))),

            Value::U8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as f32
            ))),

            Value::U16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as f32
            ))),

            Value::U32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as f32
            ))),

            Value::U64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) <= v.value().unwrap() as f64
            ))),

            Value::F32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) <= v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as u8 as f32
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as f32
            ))),

            _ => None,
        }
    }

    fn greater_than(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as f32
            ))),

            Value::I16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as f32
            ))),

            Value::I32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as f32
            ))),

            Value::I64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) > v.value().unwrap() as f64
            ))),

            Value::U8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as f32
            ))),

            Value::U16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as f32
            ))),

            Value::U32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as f32
            ))),

            Value::U64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) > v.value().unwrap() as f64
            ))),

            Value::F32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) > v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as u8 as f32
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as f32
            ))),

            _ => None,
        }
    }

    fn greater_or_equal(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as f32
            ))),

            Value::I16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as f32
            ))),

            Value::I32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as f32
            ))),

            Value::I64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) >= v.value().unwrap() as f64
            ))),

            Value::U8(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as f32
            ))),

            Value::U16(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as f32
            ))),

            Value::U32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as f32
            ))),

            Value::U64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) >= v.value().unwrap() as f64
            ))),

            Value::F32(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap()
            ))),

            Value::F64(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as f64) >= v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as u8 as f32
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as f32
            ))),

            _ => None,
        }
    }

    fn assign(&mut self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as f32),
                    None => None,
                };
                Some(self.copy())
            },

            Value::I16(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as f32),
                    None => None,
                };
                Some(self.copy())
            },

            Value::I32(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as f32),
                    None => None,
                };
                Some(self.copy())
            },

            Value::I64(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as f32),
                    None => None,
                };
                Some(self.copy())
            },

            Value::U8(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as f32),
                    None => None,
                };
                Some(self.copy())
            },

            Value::U16(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as f32),
                    None => None,
                };
                Some(self.copy())
            },

            Value::U32(v) => {
                self.value = match v.value() { 
                    Some(val) => Some(val as f32),
                    None => None,
                };
                Some(self.copy())
            },

            Value::U64(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as f32),
                    None => None,
                };
                Some(self.copy())
            },

            Value::F32(v) => {
                self.value = v.value();
                Some(self.copy())
            },

            Value::F64(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as f32),
                    None => None,
                };
                Some(self.copy())
            },
            
            Value::Bool(v) => {
                self.value = match v.value() { 
                    Some(val) => Some(val as u32 as f32),
                    None => None,
                };
                Some(self.copy())
            }
            
            _ => None,
        }
    }

    fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = self.value {
            match to { 
                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::with_value(value as u32))),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::with_value(value.to_string()))),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::F32))),
                
                _ => None,
            }
        } else {
            match to { 
                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::new())),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::F32))),
                
                _ => None,
            }
        }
    }

    fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = self.value {
            match to { 
                ApicaTypeBytecode::Any | ApicaTypeBytecode::F32 => Some(Value::F32(ValueF32::with_value(value))),
                ApicaTypeBytecode::I8 => Some(Value::I8(ValueI8::with_value(value as i8))),
                ApicaTypeBytecode::I16 => Some(Value::I16(ValueI16::with_value(value as i16))),
                ApicaTypeBytecode::I32 => Some(Value::I32(ValueI32::with_value(value as i32))),
                ApicaTypeBytecode::I64 => Some(Value::I64(ValueI64::with_value(value as i64))),
                ApicaTypeBytecode::U8 => Some(Value::U8(ValueU8::with_value(value as u8))),
                ApicaTypeBytecode::U16 => Some(Value::U16(ValueU16::with_value(value as u16))),
                ApicaTypeBytecode::U32 => Some(Value::U32(ValueU32::with_value(value as u32))),
                ApicaTypeBytecode::U64 => Some(Value::U64(ValueU64::with_value(value as u64))),
                ApicaTypeBytecode::F64 => Some(Value::F64(ValueF64::with_value(value as f64))),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(value != 0.0))),
                
                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::F32 => Some(Value::F32(ValueF32::new())),
                ApicaTypeBytecode::I8 => Some(Value::I8(ValueI8::new())),
                ApicaTypeBytecode::I16 => Some(Value::I16(ValueI16::new())),
                ApicaTypeBytecode::I32 => Some(Value::I32(ValueI32::new())),
                ApicaTypeBytecode::I64 => Some(Value::I64(ValueI64::new())),
                ApicaTypeBytecode::U8 => Some(Value::U8(ValueU8::new())),
                ApicaTypeBytecode::U16 => Some(Value::U16(ValueU16::new())),
                ApicaTypeBytecode::U32 => Some(Value::U32(ValueU32::new())),
                ApicaTypeBytecode::U64 => Some(Value::U64(ValueU64::new())),
                ApicaTypeBytecode::F64 => Some(Value::F64(ValueF64::new())),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::new())),

                _ => None,
            }
        }
    }

    fn copy(&self) -> Value {
        match self.value { 
            Some(val) => Value::F32(ValueF32::with_value(val)),
            None => Value::F32(ValueF32::new()),
        }
    }
}