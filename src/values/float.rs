use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::int::ValueInt;
use crate::values::string::ValueString;
use crate::values::uint::ValueUInt;
use crate::values::value::{Value, ValueTrait};
use crate::values::value_type::ValueType;

#[derive(Clone, Copy)]
pub struct ValueFloat {
    value: Option<f64>,
}

impl ValueFloat {
    pub fn new() -> ValueFloat {
        ValueFloat { value: None }
    }

    pub fn with_value(value: f64) -> ValueFloat {
        ValueFloat { value: Some(value) }
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }
}

impl ValueTrait for ValueFloat {
    fn is_null(&self) -> bool {
        self.value.is_none()
    }

    fn get_type_repr(&self) -> String {
        String::from("float")
    }

    fn show(&self, end: char) {
        match self.value {
            Some(v) => print!("float<{}>{}", v, end),
            None => print!("float<>{}", end),
        }
    }

    fn repr(&self) -> String {
        match self.value { 
            Some(v) => format!("float<{}>", v),
            None => String::from("float<>"),
        }
    }

    fn add(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() + v.value().unwrap() as f64
            ))),
            
            Value::UInt(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() + v.value().unwrap() as f64
            ))),
            
            Value::Float(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() + v.value().unwrap(),
            ))),

            Value::Bool(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() + v.value().unwrap() as u8 as f64
            ))),

            Value::Char(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() + v.value().unwrap() as f64
            ))),

            _ => None,
        }
    }

    fn increment(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        let old_value = *val_ref;
        *val_ref += 1.0;

        Some(Value::Float(ValueFloat::with_value(old_value)))
    }

    fn left_increment(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        *val_ref += 1.0;

        Some(Value::Float(ValueFloat::with_value(*val_ref)))
    }
    
    fn subtract(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() - v.value().unwrap() as f64
            ))),

            Value::UInt(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() - v.value().unwrap() as f64
            ))),

            Value::Float(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() - v.value().unwrap(),
            ))),

            Value::Bool(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() - v.value().unwrap() as u8 as f64
            ))),

            Value::Char(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() - v.value().unwrap() as f64
            ))),

            _ => None,
        }
    }

    fn decrement(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        let old_value = *val_ref;
        *val_ref -= 1.0;
        
        Some(Value::Float(ValueFloat::with_value(old_value)))
    }

    fn left_decrement(&mut self) -> Option<Value> {
        let val_ref = self.value.as_mut().unwrap();
        *val_ref -= 1.0;
        
        Some(Value::Float(ValueFloat::with_value(*val_ref)))
    }

    fn times(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() * v.value().unwrap() as f64
            ))),

            Value::UInt(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() * v.value().unwrap() as f64
            ))),

            Value::Float(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() * v.value().unwrap(),
            ))),

            Value::Bool(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() * v.value().unwrap() as u8 as f64
            ))),

            Value::Char(v) => Some(Value::Float(ValueFloat::with_value(
                self.value.unwrap() * v.value().unwrap() as f64
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

    fn bitwise_or(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 | v.value().unwrap()
            ))),

            Value::UInt(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 | v.value().unwrap() as i64
            ))),

            Value::Float(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 | v.value().unwrap() as i64,
            ))),

            Value::Bool(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 | v.value().unwrap() as i64
            ))),

            Value::Char(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 | v.value().unwrap() as i64
            ))),

            _ => None,
        }
    }

    fn bitwise_xor(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 ^ v.value().unwrap()
            ))),

            Value::UInt(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 ^ v.value().unwrap() as i64
            ))),

            Value::Float(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 ^ v.value().unwrap() as i64,
            ))),

            Value::Bool(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 ^ v.value().unwrap() as i64
            ))),

            Value::Char(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 ^ v.value().unwrap() as i64
            ))),

            _ => None,
        }
    }

    fn bitwise_and(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 & v.value().unwrap()
            ))),

            Value::UInt(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 & v.value().unwrap() as i64
            ))),

            Value::Float(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 & v.value().unwrap() as i64,
            ))),

            Value::Bool(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 & v.value().unwrap() as i64
            ))),

            Value::Char(v) => Some(Value::Int(ValueInt::with_value(
                self.value.unwrap() as i64 & v.value().unwrap() as i64
            ))),

            _ => None,
        }
    }

    fn less_than(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as f64
            ))),

            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as f64
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap(),
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as u8 as f64
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() < v.value().unwrap() as f64
            ))),

            _ => None,
        }
    }

    fn less_or_equal(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as f64
            ))),

            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as f64
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap(),
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as u8 as f64
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() <= v.value().unwrap() as f64
            ))),

            _ => None,
        }
    }

    fn greater_than(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as f64
            ))),

            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as f64
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap(),
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as u8 as f64
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() > v.value().unwrap() as f64
            ))),

            _ => None,
        }
    }

    fn greater_or_equal(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Int(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as f64
            ))),

            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as f64
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap(),
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as u8 as f64
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                self.value.unwrap() >= v.value().unwrap() as f64
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
                if self.is_null() { v.is_null() } else { self.value.unwrap() == v.value().unwrap() as f64 }
            ))),
            
            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() == v.value().unwrap() as f64 }
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                self.value == v.value
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() == v.value().unwrap() as u8 as f64 }
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() == v.value().unwrap() as f64 }
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
                if self.is_null() { v.is_null() } else { self.value.unwrap() != v.value().unwrap() as f64 }
            ))),

            Value::UInt(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { v.is_null() } else { self.value.unwrap() != v.value().unwrap() as f64 }
            ))),

            Value::Float(v) => Some(Value::Bool(ValueBool::with_value(
                self.value != v.value
            ))),

            Value::Bool(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { !v.is_null() } else { self.value.unwrap() != v.value().unwrap() as u8 as f64 }
            ))),

            Value::Char(v) => Some(Value::Bool(ValueBool::with_value(
                if self.is_null() { !v.is_null() } else { self.value.unwrap() != v.value().unwrap() as f64 }
            ))),

            _ => None,
        }
    }

    fn left_shift(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn right_shift(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn assign(&mut self, other: &Value) -> Option<Value> {
        match other {
            Value::Null(_) => {
                self.value = None;
                Some(Value::Float(self.clone()))
            },
            
            Value::Int(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as f64),
                    None => None,
                };
                Some(Value::Float(self.clone()))
            },

            Value::UInt(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as f64),
                    None => None,
                };
                Some(Value::Float(self.clone()))
            },

            Value::Float(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val),
                    None => None,
                };
                Some(Value::Float(self.clone()))
            },

            Value::Bool(v) => {
                self.value = match v.value() {
                    Some(val) => Some(val as u64 as f64),
                    None => None,
                };
                Some(Value::Float(self.clone()))
            }

            _ => None,
        }
    }

    fn convert(&self, to: &ValueType, is_nullable: bool) -> Option<Value> {
        if let Some(value) = self.value {
            match to.value() {
                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::with_value(value as u32))),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::with_value(value.to_string()))),
                ApicaTypeBytecode::Type => Some(Value::Type(Box::new(ValueType::new(ApicaTypeBytecode::Float, is_nullable)))),

                _ => None,
            }
        } else {
            match to.value() {
                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::new())),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),
                ApicaTypeBytecode::Type => Some(Value::Type(Box::new(ValueType::new(ApicaTypeBytecode::Float, is_nullable)))),

                _ => None,
            }
        }
    }

    fn auto_convert(&self, to: &ValueType, _is_nullable: bool) -> Option<Value> {
        if let Some(value) = self.value {
            match to.value() {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Float => Some(Value::Float(ValueFloat::with_value(value))),
                ApicaTypeBytecode::Int => Some(Value::Int(ValueInt::with_value(value as i64))),
                ApicaTypeBytecode::UnsignedInt => Some(Value::UInt(ValueUInt::with_value(value as u64))),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(value != 0.0))),

                _ => None,
            }
        } else {
            match to.value() {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Float => Some(Value::Float(ValueFloat::new())),
                ApicaTypeBytecode::Int => Some(Value::Int(ValueInt::new())),
                ApicaTypeBytecode::UnsignedInt => Some(Value::UInt(ValueUInt::new())),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::new())),

                _ => None,
            }
        }
    }
}