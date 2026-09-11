use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::string::ValueString;
use crate::values::value::{Value, ValueTrait};
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct ValueReference {
    address: Option<u32>,
    contained: ValueType,
}

impl ValueReference {
    pub fn new(contained: ValueType) -> ValueReference {
        ValueReference { address: None, contained }
    }

    pub fn with_address(contained: ValueType, address: u32) -> ValueReference {
        ValueReference { address: Some(address), contained }
    }
    
    pub fn address(&self) -> Option<u32> {
        self.address
    }
    
    pub fn contained(&self) -> &ValueType {
        &self.contained
    }
}

impl ValueTrait for ValueReference {
    fn is_null(&self) -> bool {
        self.address.is_none()
    }

    fn get_type_repr(&self) -> String {
        let inner = self.contained.inner_repr();
        format!("reference<{}>", inner)
    }

    fn show(&self, end: char) {
        if let Some(address) = self.address {
            print!("reference<{}>{}", address, end);
        } else {
            print!("reference<>{}", end);
        }
    }

    fn repr(&self) -> String {
        if let Some(address) = self.address {
            format!("reference<{}>", address)
        } else {
            String::from("reference<>")
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
        Some(Value::Bool(ValueBool::with_value(self.is_null())))
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
            Value::Null(_) => Some(Value::Bool(ValueBool::with_value(self.is_null()))),
            
            Value::Reference(v) => if self.contained.type_equals(v.contained()) {
                Some(Value::Bool(ValueBool::with_value(self.address == v.address)))
            } else { None },
            
            _ => None,
        }
    }

    fn not_equals(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Null(_) => Some(Value::Bool(ValueBool::with_value(!self.is_null()))),

            Value::Reference(v) => if self.contained.type_equals(v.contained()) {
                Some(Value::Bool(ValueBool::with_value(self.address != v.address)))
            } else { None },

            _ => None,
        }
    }

    fn assign(&mut self, other: &Value) -> Option<Value> {
        match other { 
            Value::Null(_) => {
                self.address = None;
                Some(Value::Reference(Box::new(self.clone())))
            },
            
            Value::Reference(v) => if self.contained.type_equals(v.contained()) {
                self.address = v.address.clone();
                Some(Value::Reference(v.clone()))
            } else { None },
            
            _ => None,
        }
    }

    fn convert(&self, to: &ValueType, is_nullable: bool) -> Option<Value> {
        if let Some(address) = &self.address {
            match to.value() {
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(true))),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::with_value(format!("@{}", address)))),
                ApicaTypeBytecode::Type => Some(Value::Type(Box::new(ValueType::with_contained(ApicaTypeBytecode::Reference, is_nullable, vec![self.contained.clone()])))),

                _ => None,
            }
        } else {
            match to.value() { 
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::new())),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),
                ApicaTypeBytecode::Type => Some(Value::Type(Box::new(ValueType::with_contained(ApicaTypeBytecode::Reference, is_nullable, vec![self.contained.clone()])))),
                
                _ => None,
            }
        }
    }

    fn auto_convert(&self, to: &ValueType, _is_nullable: bool) -> Option<Value> {
        match to.value() { 
            ApicaTypeBytecode::Any => Some(Value::Reference(Box::new(self.clone()))),
            ApicaTypeBytecode::Reference => if self.contained.type_equals(&to.contained()[0]) {
                Some(Value::Reference(Box::new(self.clone())))
            } else { None },
            
            _ => None,
        }
    }
}