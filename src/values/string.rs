use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::value::{Value, ValueTrait};
use crate::values::value_type::ValueType;

pub struct ValueString {
    value: Option<String>,
}

impl ValueString {
    pub fn new() -> ValueString {
        ValueString { value: None }
    }

    pub fn with_value(value: String) -> ValueString {
        ValueString { value: Some(value) }
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

impl ValueTrait for ValueString {
    fn is_null(&self) -> bool {
        self.value.is_none()
    }

    fn get_type_repr(&self) -> &str {
        "string"
    }

    fn show(&self, end: char) {
        match self.value.as_deref() {
            Some(v) => print!("string<{}>{}", v, end),
            None => print!("string<>{}", end),
        }
    }

    fn add(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::I16(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::I32(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::I64(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::U8(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::U16(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::U32(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::U64(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::F32(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::F64(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::Bool(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::Char(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), char::from_u32(v.value().unwrap()).unwrap_or('�'))
            ))),
            
            Value::String(v) => Some(Value::String(ValueString::with_value(
                format!("{}{}", self.value.as_ref().unwrap(), v.value().unwrap())
            ))),
            
            Value::Error(v) => match v.details() { 
                Some(details) => Some(Value::String(ValueString::with_value(
                    format!("{}{}: {}", self.value.as_ref().unwrap(), v.name().unwrap(), details)
                ))),
                None => Some(Value::String(ValueString::with_value(
                    format!("{}{}", self.value.as_ref().unwrap(), v.name().unwrap())
                )))
            },
            
            Value::Type(v) => Some(Value::String(ValueString::with_value(
                format!("{}<{}>", self.value.as_ref().unwrap(), v.value().unwrap().repr()))
            )),

            _ => None,
        }
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

    fn times(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(v) => {
                let val = v.value().unwrap();
                let s = if val < 0 {
                    String::from("")
                } else {
                    self.value.as_ref().unwrap().repeat(v.value().unwrap() as usize)
                };

                Some(Value::String(ValueString::with_value(s)))
            },

            Value::I16(v) => {
                let val = v.value().unwrap();
                let s = if val < 0 {
                    String::from("")
                } else {
                    self.value.as_ref().unwrap().repeat(v.value().unwrap() as usize)
                };

                Some(Value::String(ValueString::with_value(s)))
            },

            Value::I32(v) => {
                let val = v.value().unwrap();
                let s = if val < 0 {
                    String::from("")
                } else {
                    self.value.as_ref().unwrap().repeat(v.value().unwrap() as usize)
                };

                Some(Value::String(ValueString::with_value(s)))
            },

            Value::I64(v) => {
                let val = v.value().unwrap();
                let s = if val < 0 {
                    String::from("")
                } else {
                    self.value.as_ref().unwrap().repeat(v.value().unwrap() as usize)
                };

                Some(Value::String(ValueString::with_value(s)))
            },

            Value::U8(v) => {
                let s = self.value.as_ref().unwrap().repeat(v.value().unwrap() as usize);
                Some(Value::String(ValueString::with_value(s)))
            },

            Value::U16(v) => {
                let s = self.value.as_ref().unwrap().repeat(v.value().unwrap() as usize);
                Some(Value::String(ValueString::with_value(s)))
            },

            Value::U32(v) => {
                let s = self.value.as_ref().unwrap().repeat(v.value().unwrap() as usize);
                Some(Value::String(ValueString::with_value(s)))
            },

            Value::U64(v) => {
                let s = self.value.as_ref().unwrap().repeat(v.value().unwrap() as usize);
                Some(Value::String(ValueString::with_value(s)))
            },

            _ => None,
        }
    }

    fn unary_not(&self) -> Option<Value> {
        Some(Value::Bool(ValueBool::with_value(
            match &self.value {
                Some(v) => v.is_empty(),
                None => true,
            }
        )))
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

    fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::String))),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(!value.is_empty()))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::String))),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::new())),

                _ => None,
            }
        }
    }

    fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::String => Some(Value::String(ValueString::with_value(value.clone()))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),

                _ => None,
            }
        }
    }

    fn copy(&self) -> Value {
        match &self.value { 
            Some(val) => Value::String(ValueString::with_value(val.to_string())),
            None => Value::String(ValueString::new()),
        }
    }
}