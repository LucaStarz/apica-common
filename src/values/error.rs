use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::string::ValueString;
use crate::values::value::{Value, ValueTrait};
use crate::values::value_type::ValueType;

pub struct ValueError {
    name: Option<String>,
    details: Option<String>,
}

impl ValueError {
    pub fn new() -> ValueError {
        ValueError { name: None, details: None }
    }

    pub fn with_name(name: String) -> ValueError {
        ValueError { name: Some(name), details: None }
    }

    pub fn with_details(name: String, details: String) -> ValueError {
        ValueError { name: Some(name), details: Some(details) }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }
}

impl ValueTrait for ValueError {
    fn is_null(&self) -> bool {
        self.name.is_none()
    }

    fn get_type_repr(&self) -> &str {
        "error"
    }

    fn show(&self, end: char) {
        match self.name.as_deref() {
            Some(n) => {
                match self.details.as_deref() {
                    Some(d) => print!("error<{}: {}>{}", n, d, end),
                    None => print!("error<{}>{}", n, end),
                }
            },
            None => print!("error<>{}", end),
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
        Some(Value::Bool(ValueBool::with_value(
            match &self.name { 
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

    fn assign(&mut self, other: &Value) -> Option<Value> {
        match other { 
            Value::Error(v) => {
                self.name = match v.name() { 
                    Some(n) => Some(n.to_string()),
                    None => None,
                };
                self.details = match v.details() {
                    Some(d) => Some(d.to_string()),
                    None => None,
                };
                
                Some(self.copy())
            },
            
            _ => None,
        }
    }

    fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(name) = &self.name {
            match to { 
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(true))),
                ApicaTypeBytecode::String => match &self.details {
                    Some(details) => Some(Value::String(ValueString::with_value(format!("{}: {}", name, details)))),
                    None => Some(Value::String(ValueString::with_value(name.to_string()))),
                },
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::Error))),
                
                _ => None,
            }
        } else {
            match to { 
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::new())),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::with_type(ApicaTypeBytecode::Error))),
                
                _ => None,
            }
        }
    }

    fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(name) = &self.name {
            match to { 
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Error => match &self.details { 
                    Some(details) => Some(Value::Error(Box::new(ValueError::with_details(name.to_string(), details.to_string())))),
                    None => Some(Value::Error(Box::new(ValueError::with_name(name.to_string())))),
                },
                
                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Error => Some(Value::Error(Box::new(ValueError::new()))),

                _ => None,   
            }
        }
    }

    fn copy(&self) -> Value {
        match &self.name { 
            Some(name) => match &self.details { 
                Some(details) => Value::Error(Box::new(ValueError::with_details(name.to_string(), details.to_string()))),
                None => Value::Error(Box::new(ValueError::with_name(name.to_string()))),
            },
            
            None => Value::Error(Box::new(ValueError::new())),
        }
    }
}