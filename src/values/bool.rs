use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::char::ValueChar;
use crate::values::float::ValueFloat;
use crate::values::int::ValueInt;
use crate::values::string::ValueString;
use crate::values::uint::ValueUInt;
use crate::values::value::{Value, ValueTrait};
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct ValueBool {
    value: Option<bool>,
}

impl ValueBool {
    pub fn new() -> ValueBool {
        ValueBool { value: None }
    }

    pub fn with_value(value: bool) -> ValueBool {
        ValueBool { value: Some(value) }
    }

    pub fn value(&self) -> Option<bool> {
        self.value
    }
}

impl ValueTrait for ValueBool {
    fn is_null(&self) -> bool {
        self.value.is_none()
    }

    fn get_type_repr(&self) -> String {
        String::from("bool")
    }

    fn show(&self, end: char) {
        match self.value {
            Some(v) => print!("bool<{}>{}", v, end),
            None => print!("bool<>{}", end),
        }
    }

    fn repr(&self) -> String {
        match self.value { 
            Some(v) => format!("bool<{}>", v),
            None => String::from("bool<>"),
        }
    }

    fn add(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 + v.value().unwrap()
            ))),
            
            Value::UInt(v) => Some(Value::UInt(ValueUInt::with_value(
                self.value.unwrap() as u64 + v.value().unwrap()
            ))),

            Value::Float(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() as u8 as f64 + v.value().unwrap(),
            ))),

            Value::Bool(v) => Some(Value::UInt(ValueUInt::with_value(
                self.value.unwrap() as u64 + v.value().unwrap() as u64
            ))),

            Value::Char(v) => Some(Value::Char(ValueChar::with_value(
                self.value.unwrap() as u32 + v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn increment(&mut self) -> Option<Value> {
        None
    }

    fn left_increment(&mut self) -> Option<Value> {
        None
    }

    fn subtract(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 - v.value().unwrap()
            ))),

            Value::UInt(v) => Some(Value::UInt(ValueUInt::with_value(
                self.value.unwrap() as u64 - v.value().unwrap()
            ))),

            Value::Float(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() as u8 as f64 - v.value().unwrap(),
            ))),

            Value::Bool(v) => Some(Value::UInt(ValueUInt::with_value(
                self.value.unwrap() as u64 - v.value().unwrap() as u64
            ))),

            Value::Char(v) => Some(Value::Char(ValueChar::with_value(
                self.value.unwrap() as u32 - v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn decrement(&mut self) -> Option<Value> {
        None
    }

    fn left_decrement(&mut self) -> Option<Value> {
        None
    }

    fn times(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 * v.value().unwrap()
            ))),
            
            Value::UInt(v) => Some(Value::UInt(ValueUInt::with_value(
                self.value.unwrap() as u64 * v.value().unwrap()
            ))),
            
            Value::Float(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() as u8 as f64 * v.value().unwrap(),
            ))),

            Value::Bool(v) => Some(Value::UInt(ValueUInt::with_value(
                self.value.unwrap() as u64 * v.value().unwrap() as u64
            ))),

            Value::Char(v) => Some(Value::Char(ValueChar::with_value(
                self.value.unwrap() as u32 * v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn unary_not(&self) -> Option<Value> {
        Some(Value::Bool(ValueBool::with_value(
            match self.value {
                Some(v) => !v,
                None => true
            }
        )))
    }

    fn bitwise_not(&self) -> Option<Value> {
        Some(Value::Bool(ValueBool::with_value(
            !self.value.unwrap()
        )))
    }

    fn bitwise_or(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 | v.value().unwrap()
            ))),
            
            Value::UInt(v) => Some(Value::UInt(ValueUInt::with_value(
                self.value.unwrap() as u64 | v.value().unwrap()
            ))),
            
            Value::Float(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 | v.value().unwrap() as i64
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() | v.value().unwrap()
            ))),

            Value::Char(v) => Some(Value::Char(ValueChar::with_value(
                self.value.unwrap() as u32 | v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn bitwise_xor(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 ^ v.value().unwrap()
            ))),

            Value::UInt(v) => Some(Value::UInt(ValueUInt::with_value(
                self.value.unwrap() as u64 ^ v.value().unwrap()
            ))),

            Value::Float(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 ^ v.value().unwrap() as i64
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() ^ v.value().unwrap()
            ))),

            Value::Char(v) => Some(Value::Char(ValueChar::with_value(
                self.value.unwrap() as u32 ^ v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn bitwise_and(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 & v.value().unwrap()
            ))),

            Value::UInt(v) => Some(Value::UInt(ValueUInt::with_value(
                self.value.unwrap() as u64 & v.value().unwrap()
            ))),

            Value::Float(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 & v.value().unwrap() as i64
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() & v.value().unwrap()
            ))),

            Value::Char(v) => Some(Value::Char(ValueChar::with_value(
                self.value.unwrap() as u32 & v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn less_than(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i64) < v.value().unwrap()
            ))),
            
            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u64) < v.value().unwrap()
            ))),
            
            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u8 as f64) < v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u8) < (v.value().unwrap() as u8)
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) < v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn less_or_equal(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i64) <= v.value().unwrap()
            ))),

            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u64) <= v.value().unwrap()
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u8 as f64) <= v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u8) <= (v.value().unwrap() as u8)
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) <= v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn greater_than(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i64) > v.value().unwrap()
            ))),

            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u64) > v.value().unwrap()
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u8 as f64) > v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u8) > (v.value().unwrap() as u8)
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) > v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn greater_or_equal(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as i64) >= v.value().unwrap()
            ))),

            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u64) >= v.value().unwrap()
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u8 as f64) >= v.value().unwrap()
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u8) >= (v.value().unwrap() as u8)
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                (self.value.unwrap() as u32) >= v.value().unwrap()
            ))),

            _ => None,
        }
    }

    fn equals(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Null(_) => Some(Value::Bool(ValueBool::with_value(
                self.is_null()
            ))),
            
            Value::Int(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() as i64 == v.value().unwrap() }
            ))),

            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() as u64 == v.value().unwrap() }
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() as u8 as f64 == v.value().unwrap() }
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value == v.value
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() as u32 == v.value().unwrap() }
            ))),

            _ => None,
        }
    }

    fn not_equals(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Null(_) => Some(Value::Bool(ValueBool::with_value(
                !self.is_null()
            ))),

            Value::Int(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() as i64 != v.value().unwrap() }
            ))),

            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() as u64 != v.value().unwrap() }
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() as u8 as f64 != v.value().unwrap() }
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value != v.value
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { !v.is_null() } else { self.value.unwrap() as u32 != v.value().unwrap() }
            ))),

            _ => None,
        }
    }

    fn assign(&mut self, other: &Value) -> Option<Value> {
        match other {
            Value::Null(_) => {
                self.value = None;
                Some(Value::Bool(self.clone()))
            },
            
            Value::Int(v) => {
                self.value = match v.value() { 
                    Some(val) => Some(val != 0),
                    None => None,
                };
                Some(Value::Bool(self.clone()))
            },
            
            Value::UInt(v) => {
                self.value = match v.value() { 
                    Some(val) => Some(val != 0),
                    None => None,
                };
                Some(Value::Bool(self.clone()))
            },
            
            Value::Float(v) => {
                self.value = match v.value() { 
                    Some(val) => Some(val != 0.0),
                    None => None,
                };
                Some(Value::Bool(self.clone()))
            },
            
            Value::Bool(v) => {
                self.value = v.value();
                Some(Value::Bool(self.clone()))
            },
            
            Value::Char(v) => {
                self.value = match v.value() { 
                    Some(val) => Some(val != 0),
                    None => None,
                };
                Some(Value::Bool(self.clone()))
            },
            
            _ => None,
        }
    }

    fn convert(&self, to: &ValueType, is_nullable: bool) -> Option<Value> {
        if let Some(value) = self.value {
            match to.value() {
                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::with_value(value as u32))),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::with_value(String::from(if value { "true" } else { "false" })))),
                ApicaTypeBytecode::Type => Some(Value::Type(Box::new(ValueType::new(ApicaTypeBytecode::Bool, is_nullable)))),

                _ => None,
            }
        } else {
            match to.value() {
                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::new())),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),
                ApicaTypeBytecode::Type => Some(Value::Type(Box::new(ValueType::new(ApicaTypeBytecode::Bool, is_nullable)))),

                _ => None,
            }
        }
    }

    fn auto_convert(&self, to: &ValueType, _is_nullable: bool) -> Option<Value> {
        if let Some(value) = self.value {
            match to.value() {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(value))),
                ApicaTypeBytecode::Int => Some(Value::Int(ValueInt::with_value(value as i64))),
                ApicaTypeBytecode::UnsignedInt => Some(Value::UInt(ValueUInt::with_value(value as u64))),
                ApicaTypeBytecode::Float => Some(Value::Float(ValueFloat::with_value(value as u8 as f64))),

                _ => None,
            }
        } else {
            match to.value() {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::new())),
                ApicaTypeBytecode::Int => Some(Value::Int(ValueInt::new())),
                ApicaTypeBytecode::UnsignedInt => Some(Value::UInt(ValueUInt::new())),
                ApicaTypeBytecode::Float => Some(Value::Float(ValueFloat::new())),

                _ => None,
            }
        }
    }
}