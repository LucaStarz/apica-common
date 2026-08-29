use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::string::ValueString;
use crate::values::value::{Value, ValueTrait};

pub struct ValueType {
    value: Option<ApicaTypeBytecode>,
}

impl ApicaTypeBytecode {
    pub fn repr(&self) -> &'static str {
        match self {
            ApicaTypeBytecode::Null => "null",

            ApicaTypeBytecode::I8 => "i8",
            ApicaTypeBytecode::I16 => "i16",
            ApicaTypeBytecode::I32 => "i32",
            ApicaTypeBytecode::I64 => "i64",
            ApicaTypeBytecode::U8 => "u8",
            ApicaTypeBytecode::U16 => "u16",
            ApicaTypeBytecode::U32 => "u32",
            ApicaTypeBytecode::U64 => "u64",
            ApicaTypeBytecode::F32 => "f32",
            ApicaTypeBytecode::F64 => "f64",
            ApicaTypeBytecode::Bool => "bool",

            ApicaTypeBytecode::Char => "char",
            ApicaTypeBytecode::String => "string",

            ApicaTypeBytecode::Error => "error",
            ApicaTypeBytecode::Type => "type",

            _ => "???",
        }
    }
}

impl ValueType {
    pub fn new() -> ValueType {
        ValueType { value: None }
    }

    pub fn with_type(value: ApicaTypeBytecode) -> ValueType {
        ValueType { value: Some(value) }
    }

    pub fn value(&self) -> Option<ApicaTypeBytecode> {
        self.value
    }
}

impl ValueTrait for ValueType {
    fn is_null(&self) -> bool {
        self.value.is_none()
    }

    fn get_type_repr(&self) -> &str {
        "type"
    }

    fn show(&self, end: char) {
        match self.value {
            Some(v) => print!("type<{}>{}", v.repr(), end),
            None => print!("type<>{}", end),
        }
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

    fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = self.value {
            match to {
                ApicaTypeBytecode::String => Some(Value::String(ValueString::with_value(format!("<{}>", value.repr())))),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(value != ApicaTypeBytecode::Null))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::new())),

                _ => None,
            }
        }
    }

    fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = self.value {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Type(ValueType::with_type(value))),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::Type))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Type(ValueType::new())),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::Type))),

                _ => None,
            }
        }
    }

    fn copy(&self) -> Value {
        match self.value { 
            Some(vt) => Value::Type(ValueType::with_type(vt)),
            None => Value::Type(ValueType::new()),
        }
    }
}