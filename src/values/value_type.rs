use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::string::ValueString;
use crate::values::value::{Value, ValueTrait};

pub struct ValueType {
    value: ApicaTypeBytecode,
}

impl ValueType {
    pub fn with_type(value: ApicaTypeBytecode) -> ValueType {
        ValueType { value }
    }

    pub fn value(&self) -> ApicaTypeBytecode {
        self.value
    }
}

impl ValueTrait for ValueType {
    fn is_null(&self) -> bool {
        false
    }

    fn get_type_repr(&self) -> &str {
        "type"
    }

    fn show(&self, end: char) {
        print!("type<{}>{}", self.value.repr(), end);
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

    fn equals(&self, other: &Value) -> Option<Value> {
        match other { 
            Value::Null(_) => Some(Value::Bool(ValueBool::with_value(
                self.value == ApicaTypeBytecode::Null
            ))),
            
            Value::Type(v) => Some(Value::Bool(ValueBool::with_value(
                self.value == v.value
            ))),
            
            _ => None,
        }
    }

    fn not_equals(&self, other: &Value) -> Option<Value> {
        match other { 
            Value::Null(_) => Some(Value::Bool(ValueBool::with_value(
                self.value != ApicaTypeBytecode::Null
            ))),
            
            Value::Type(v) => Some(Value::Bool(ValueBool::with_value(
                self.value != v.value
            ))),
            
            _ => None,
        }
    }

    fn assign(&mut self, other: &Value) -> Option<Value> {
        match other { 
            Value::Type(v) => {
                self.value = v.value();
                Some(self.copy())
            },
            
            _ => None,
        }
    }
    
    fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        match to {
            ApicaTypeBytecode::String => Some(Value::String(ValueString::with_value(format!("<{}>", self.value.repr())))),
            ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(self.value != ApicaTypeBytecode::Null))),

            _ => None,
        }
    }

    fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        match to {
            ApicaTypeBytecode::Any => Some(Value::Type(ValueType::with_type(self.value))),
            ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::Type))),

            _ => None,
        }
    }

    fn copy(&self) -> Value {
        Value::Type(ValueType::with_type(self.value.clone()))
    }
}