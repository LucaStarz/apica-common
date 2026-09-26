use crate::bytecodes::builtin_method::ApicaBuiltinMethodBytecode;
use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::common;
use crate::values::list::ValueList;
use crate::values::string::ValueString;
use crate::values::uint::ValueUInt;
use crate::values::value::{Value, ValueTrait};
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct ValueArray {
    values: Option<Vec<Value>>,
    contained: ValueType,
}

impl ValueArray {
    pub fn new(contained: ValueType) -> ValueArray {
        ValueArray { values: None, contained }
    }
    
    pub fn with_length(contained: ValueType, length: usize) -> ValueArray {
        ValueArray {
            values: Some(Vec::with_capacity(length)),
            contained
        }
    }
    
    pub fn with_values(contained: ValueType, values: Vec<Value>) -> ValueArray {
        ValueArray { values: Some(values), contained }
    }
    
    pub fn values(&self) -> Option<&Vec<Value>> {
        self.values.as_ref()
    }
    
    pub fn contained(&self) -> &ValueType {
        &self.contained
    }
    
    fn values_repr(&self) -> String {
        let mut result = String::from("array<");
        if let Some(values) = &self.values {
            for i in 0..values.len() {
                result.push_str(&values[i].repr());
                if i < values.len() - 1 {
                    result.push_str(", ");
                }
            }
        }
        
        result.push('>');
        result
    }

    fn repeat(&self, n: usize) -> Value {
        let values = self.values().unwrap();
        let mut repeated = Vec::with_capacity(values.len() * n);
        for _ in 0..n {
            repeated.extend(values.iter().cloned());
        }

        Value::Array(Box::new(ValueArray::with_values(self.contained.clone(), repeated)))
    }
}

impl ValueTrait for ValueArray {
    fn is_null(&self) -> bool {
        self.values.is_none()
    }

    fn get_type_repr(&self) -> String {
        let inner = self.contained.inner_repr();
        format!("array{}", inner)
    }

    fn show(&self, end: char) {
        print!("{}{}", self.values_repr(), end);
    }

    fn repr(&self) -> String {
        self.values_repr()
    }

    fn add(&self, other: &Value) -> Option<Value> {
        match other { 
            Value::Array(v) => {
                let mut array = Vec::with_capacity(self.values().unwrap().len() + v.values().unwrap().iter().len());
                array.extend(self.values().unwrap().iter().cloned());
                array.extend(v.values().unwrap().iter().cloned());

                Some(Value::Array(Box::new(ValueArray::with_values(
                    if v.contained.type_equals(&self.contained) {
                        self.contained.clone()
                    } else {
                        ValueType::new(ApicaTypeBytecode::Any, true)
                    },
                    array,
                ))))
            },

            Value::List(v) => {
                let mut array = Vec::with_capacity(self.values().unwrap().len() + v.values().unwrap().iter().len());
                array.extend(self.values().unwrap().iter().cloned());
                array.extend(v.values().unwrap().iter().cloned());

                Some(Value::List(Box::new(ValueList::with_values(
                    if v.contained().type_equals(&self.contained) {
                        self.contained.clone()
                    } else {
                        ValueType::new(ApicaTypeBytecode::Any, true)
                    },
                    array,
                ))))
            },

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

    fn times(&self, other: &Value) -> Result<Option<Value>, ()> {
        match other {
            Value::Int(v) => {
                let val = v.value().unwrap();

                if val < 0 {
                    Err(())
                } else {
                    Ok(Some(self.repeat(val as usize)))
                }
            },

            Value::UInt(v) => {
                let val = v.value().unwrap();
                Ok(Some(self.repeat(val as usize)))
            },

            Value::Char(v) => {
                let val = v.value().unwrap();
                Ok(Some(self.repeat(val as usize)))
            },

            _ => Ok(None),
        }
    }

    fn divide(&self, _other: &Value) -> Result<Option<Value>, ()> {
        Ok(None)
    }

    fn modulo(&self, _other: &Value) -> Result<Option<Value>, ()> {
        Ok(None)
    }

    fn unary_not(&self) -> Option<Value> {
        Some(Value::Bool(ValueBool::with_value(
            match &self.values {
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

            Value::Array(v) => Some(Value::Bool(ValueBool::with_value(
                if self.contained.type_equals(v.contained()) {
                    common::vec_equals(self.values().unwrap(), v.values().unwrap())
                } else {
                    false
                }
            ))),
            
            Value::List(v) => Some(Value::Bool(ValueBool::with_value(
                if self.contained.type_equals(v.contained()) {
                    common::vec_equals(self.values().unwrap(), v.values().unwrap())
                } else {
                    false
                }
            ))),

            _ => None,
        }
    }

    fn not_equals(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Null(_) => Some(Value::Bool(ValueBool::with_value(
                !self.is_null()
            ))),

            Value::Array(v) => Some(Value::Bool(ValueBool::with_value(
                if self.contained.type_equals(v.contained()) {
                    !common::vec_equals(self.values().unwrap(), v.values().unwrap())
                } else {
                    false
                }
            ))),

            Value::List(v) => Some(Value::Bool(ValueBool::with_value(
                if self.contained.type_equals(v.contained()) {
                    !common::vec_equals(self.values().unwrap(), v.values().unwrap())
                } else {
                    false
                }
            ))),

            _ => None,
        }
    }

    fn logical_or(&self, other: &Value) -> Value {
        if let Some(values) = self.values.as_ref() && !values.is_empty() {
            Value::Bool(ValueBool::with_value(true))
        } else {
            common::boolean_state(other)
        }
    }

    fn logical_and(&self, other: &Value) -> Value {
        if self.values.is_none() || self.values.as_ref().unwrap().is_empty() {
            Value::Bool(ValueBool::with_value(false))
        } else {
            common::boolean_state(other)
        }
    }

    fn left_shift(&self, _other: &Value) -> Result<Option<Value>, ()> {
        Ok(None)
    }

    fn right_shift(&self, _other: &Value) -> Result<Option<Value>, ()> {
        Ok(None)
    }

    fn assign(&mut self, other: &Value) -> Option<Value> {
        match other {
            Value::Null(_) => {
                self.values = None;
                Some(Value::Array(Box::new(self.clone())))
            },
            
            Value::Array(v) => if self.contained.type_equals(v.contained()) {
                self.values = Some(v.values().unwrap().clone());
                Some(Value::Array(Box::new(self.clone())))
            } else { None },
            
            Value::List(v) => if self.contained.type_equals(v.contained()) {
                self.values = Some(v.values().unwrap().clone());
                Some(Value::Array(Box::new(self.clone())))
            } else { None },
            
            _ => None,
        }
    }

    fn access(&mut self, method: ApicaBuiltinMethodBytecode) -> Option<Value> {
        match method { 
            ApicaBuiltinMethodBytecode::Length => Some(Value::UInt(ValueUInt::with_value(self.values().unwrap().len() as u64))),
            ApicaBuiltinMethodBytecode::Empty => Some(Value::Bool(ValueBool::with_value(self.values().unwrap().is_empty()))),
            
            _ => None,
        }
    }

    fn convert(&self, to: &ValueType, is_nullable: bool) -> Option<Value> {
        if let Some(values) = self.values.as_ref() {
            match to.value() { 
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(!values.is_empty()))),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::with_value(self.repr()))),
                ApicaTypeBytecode::Type => Some(Value::Type(Box::new(ValueType::with_contained(ApicaTypeBytecode::Array, is_nullable, vec![self.contained.clone()])))),
                
                _ => None,
            }
        } else {
            match to.value() { 
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::new())),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::new())),
                ApicaTypeBytecode::Type => Some(Value::Type(Box::new(ValueType::with_contained(ApicaTypeBytecode::Array, is_nullable, vec![self.contained.clone()])))),
                
                _ => None,
            }
        }
    }

    fn auto_convert(&self, to: &ValueType, _is_nullable: bool) -> Option<Value> {
        match to.value() { 
            ApicaTypeBytecode::Any => Some(Value::Array(Box::new(self.clone()))),
            ApicaTypeBytecode::Array => if self.contained.type_equals(&to.contained()[0]) {
                Some(Value::Array(Box::new(self.clone())))
            } else { None },
            
            _ => None,
        }
    }
}