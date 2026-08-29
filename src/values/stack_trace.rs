use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::value::{Value, ValueTrait};

pub struct ValueStackTrace {
    name: String,
    details: Option<String>,
    trace: Vec<String>,
}

impl ValueStackTrace {
    pub fn new(name: String) -> ValueStackTrace {
        ValueStackTrace { name, details: None, trace: Vec::new() }
    }

    pub fn with_details(name: String, details: String) -> ValueStackTrace {
        ValueStackTrace { name, details: Some(details), trace: Vec::new() }
    }

    pub fn add_trace(&mut self, trace: String) {
        self.trace.push(trace);
    }

    pub fn message(&self) -> String {
        let mut message = self.name.clone();
        if let Some(d) = self.details.as_deref() {
            message.push_str(": ");
            message.push_str(d);
        }

        message.push_str("\nStack trace:");
        for trace in &self.trace {
            message.push('\n');
            message.push_str(trace);
        }

        message
    }
}

impl ValueTrait for ValueStackTrace {
    fn is_null(&self) -> bool {
        false
    }

    fn get_type_repr(&self) -> &str {
        "error"
    }
    
    fn show(&self, end: char) {
        print!("stack-trace<>{}", end);
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
        None
    }
    
    fn convert(&self, _to: ApicaTypeBytecode) -> Option<Value> {
        None
    }

    fn auto_convert(&self, _to: ApicaTypeBytecode) -> Option<Value> {
        None
    }

    fn can_convert_to(&self, _to: ApicaTypeBytecode, _is_auto: bool) -> bool {
        false
    }

    fn copy(&self) -> Value {
        match &self.details { 
            Some(details) => Value::StackTrace(Box::new(ValueStackTrace::with_details(self.name.clone(), details.to_string()))),
            None => Value::StackTrace(Box::new(ValueStackTrace::new(self.name.clone()))),
        }
    }
}