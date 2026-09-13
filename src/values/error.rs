use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::string::ValueString;
use crate::values::value::{Value, ValueTrait};
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct ValueError {
    name: Option<String>,
    details: Option<String>,
    stack_trace: Vec<String>,
}

impl ValueError {
    pub fn new() -> ValueError {
        ValueError { name: None, details: None, stack_trace: vec![] }
    }

    pub fn with_name(name: String) -> ValueError {
        ValueError { name: Some(name), details: None, stack_trace: vec![] }
    }

    pub fn with_details(name: String, details: String) -> ValueError {
        ValueError { name: Some(name), details: Some(details), stack_trace: vec![] }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }
    
    pub fn stack_trace(&self) -> &Vec<String> {
        &self.stack_trace
    }
    
    pub fn message(&self) -> String {
        let mut message = self.name.as_ref().unwrap_or(&String::from("error")).clone();
        if let Some(d) = self.details.as_deref() {
            message.push_str(": ");
            message.push_str(d);
        }

        message.push_str("\nStack trace:");
        for trace in self.stack_trace.iter() {
            message.push('\n');
            message.push_str(trace);
        }

        message
    }
    
    pub fn add_trace(&mut self, trace: String) {
        self.stack_trace.push(trace);
    }
}

impl ValueTrait for ValueError {
    fn is_null(&self) -> bool {
        self.name.is_none()
    }

    fn get_type_repr(&self) -> String {
        String::from("error")
    }

    fn show(&self, end: char) {
        match self.name.as_deref() {
            Some(n) => match self.details.as_deref() {
                Some(d) => print!("error<{}: {}>{}", n, d, end),
                None => print!("error<{}>{}", n, end),
            },
            None => print!("error<>{}", end),
        }
    }

    fn repr(&self) -> String {
        match self.name.as_deref() { 
            Some(n) => match self.details.as_deref() { 
                Some(d) => format!("error<{}: {}>", n, d),
                None => format!("error<{}>", n),
            },
            None => String::from("error<>"),
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

    fn bitwise_or(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn bitwise_xor(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn bitwise_and(&self, _other: &Value) -> Option<Value> {
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
                self.is_null()
            ))),
            
            Value::Error(v) => Some(Value::Bool(ValueBool::with_value(
                self.name == v.name && self.details == v.details,
            ))),
            
            _ => None,
        }
    }

    fn not_equals(&self, other: &Value) -> Option<Value> {
        match other { 
            Value::Null(_) => Some(Value::Bool(ValueBool::with_value(
                !self.is_null()
            ))),
            
            Value::Error(v) => Some(Value::Bool(ValueBool::with_value(
                self.name != v.name || self.details != v.details,
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
                self.name = None;
                self.details = None;
                Some(Value::Error(Box::new(self.clone())))
            },
            
            Value::Error(v) => {
                self.name = match v.name() { 
                    Some(n) => Some(n.to_string()),
                    None => None,
                };
                self.details = match v.details() {
                    Some(d) => Some(d.to_string()),
                    None => None,
                };
                
                Some(Value::Error(Box::new(self.clone())))
            },
            
            _ => None,
        }
    }

    fn convert(&self, to: &ValueType, is_nullable: bool) -> Option<Value> {
        if let Some(name) = &self.name {
            match to.value() { 
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(true))),
                ApicaTypeBytecode::String => match &self.details {
                    Some(details) => Some(Value::String(ValueString::with_value(format!("{}: {}", name, details)))),
                    None => Some(Value::String(ValueString::with_value(name.to_string()))),
                },
                ApicaTypeBytecode::Type => Some(Value::Type(Box::new(ValueType::new(ApicaTypeBytecode::Error, is_nullable)))),
                
                _ => None,
            }
        } else {
            match to.value() { 
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::new())),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),
                ApicaTypeBytecode::Type => Some(Value::Type(Box::new(ValueType::new(ApicaTypeBytecode::Error, is_nullable)))),
                
                _ => None,
            }
        }
    }

    fn auto_convert(&self, to: &ValueType, _is_nullable: bool) -> Option<Value> {
        if self.name.is_some() {
            match to.value() { 
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Error => Some(Value::Error(Box::new(self.clone()))),
                
                _ => None,
            }
        } else {
            match to.value() {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Error => Some(Value::Error(Box::new(ValueError::new()))),

                _ => None,   
            }
        }
    }
}