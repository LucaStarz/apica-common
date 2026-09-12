use crate::bytecodes::apica::ApicaBytecode;
use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::string::ValueString;
use crate::values::value::{Value, ValueTrait};

#[derive(Clone)]
pub struct ValueType {
    value: ApicaTypeBytecode,
    is_nullable: bool,
    contained: Vec<ValueType>,
}

impl ValueType {
    pub const fn new(value: ApicaTypeBytecode, is_nullable: bool) -> ValueType {
        ValueType { value, is_nullable, contained: vec![] }
    }
    
    pub const fn with_contained(value: ApicaTypeBytecode, is_nullable: bool, contained: Vec<ValueType>) -> ValueType {
        ValueType { value, is_nullable, contained }
    }

    pub fn value(&self) -> ApicaTypeBytecode {
        self.value
    }
    
    pub fn is_nullable(&self) -> bool {
        self.is_nullable
    }
    
    pub fn contained(&self) -> &Vec<ValueType> {
        &self.contained
    }
    
    pub fn inner_repr(&self) -> String {
        let mut inner = String::new();
        if !self.contained.is_empty() {
            inner.push('<');
            for i in 0..self.contained.len() {
                inner.push_str(&self.contained[i].inner_repr());

                if i < self.contained.len() - 1 {
                    inner.push_str(", ");
                }
            }
            
            inner.push('>');
        }

        if self.is_nullable {
            inner.push('!');
        }
        
        format!("{}<{}>", self.value.repr(), inner)
    }
    
    pub fn type_equals(&self, other: &ValueType) -> bool {
        let mut result = (self.value == other.value || self.value == ApicaTypeBytecode::Any) && self.contained.len() == other.contained.len();
        if result {
            for i in 0..self.contained.len() {
                result = self.contained[i].type_equals(&other.contained[i]);
                if !result { break; }
            }
        }
        
        result
    }
    
    pub const TYPE_STRING: ValueType = ValueType::new(ApicaTypeBytecode::String, false);
    pub const TYPE_BOOLEAN: ValueType = ValueType::new(ApicaTypeBytecode::Bool, false);
    pub const TYPE_TYPE: ValueType = ValueType::new(ApicaTypeBytecode::Type, false);
    
    fn primitive(&self) -> u8 {
        self.value as u8
    }

    fn is_number(&self) -> bool {
        matches!(self.value, ApicaTypeBytecode::Int | ApicaTypeBytecode::UnsignedInt | ApicaTypeBytecode::Float)
    }

    fn number_can_convert_to(to: &ValueType, is_auto: bool) -> bool {
        match to.value {
            ApicaTypeBytecode::Any | ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char => true,
            ApicaTypeBytecode::String | ApicaTypeBytecode::Type => !is_auto,

            _ => to.is_number(),
        }
    }

    fn decimal_can_convert_to(to: &ValueType, is_auto: bool) -> bool {
        match to.value {
            ApicaTypeBytecode::Any | ApicaTypeBytecode::Bool => true,
            ApicaTypeBytecode::Char | ApicaTypeBytecode::String | ApicaTypeBytecode::Type => !is_auto,

            _ => to.is_number(),
        }
    }

    fn number_comparison_resolve_to(other: &ValueType, is_equality: bool) -> Option<ValueType> {
        match other.value {
            ApicaTypeBytecode::Null => if is_equality { 
                Some(ValueType::new(ApicaTypeBytecode::Bool, true)) 
            } else {
                None 
            },

            _ if other.is_number() || matches!(other.value, ApicaTypeBytecode::Any | ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char)
            => Some(ValueType::new(ApicaTypeBytecode::Bool, true)),

            _ => None,
        }
    }

    fn resolve_type_increment_decrement(&self) -> Option<ValueType> {
        match self.value {
            ApicaTypeBytecode::Any => Some(ValueType::new(ApicaTypeBytecode::Any, true)),

            ApicaTypeBytecode::Null | ApicaTypeBytecode::Bool | ApicaTypeBytecode::String
            | ApicaTypeBytecode::Error | ApicaTypeBytecode::Type | ApicaTypeBytecode::Reference
                => None,

            _ => Some(self.clone()),
        }
    }

    fn resolve_type_unary_not(&self) -> Option<ValueType> {
        match self.value {
            ApicaTypeBytecode::Type => None,

            _ => Some(ValueType::new(ApicaTypeBytecode::Bool, true)),
        }
    }

    fn resolve_type_bitwise_not(&self) -> Option<ValueType> {
        match self.value {
            ApicaTypeBytecode::Any => Some(ValueType::new(ApicaTypeBytecode::Any, true)),

            ApicaTypeBytecode::Null | ApicaTypeBytecode::Float
            | ApicaTypeBytecode::String | ApicaTypeBytecode::Error | ApicaTypeBytecode::Type
            | ApicaTypeBytecode::Reference => None,

            _ => Some(self.clone()),
        }
    }

    fn resolve_type_compare(&self, other: &ValueType) -> Option<ValueType> {
        match self.value {
            ApicaTypeBytecode::Any => Some(ValueType::new(ApicaTypeBytecode::Bool, false)),

            _ if self.is_number() || matches!(self.value, ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char)
                => ValueType::number_comparison_resolve_to(other, false),

            _ => None,
        }
    }

    fn resolve_type_equality(&self, other: &ValueType) -> Option<ValueType> {
        match self.value {
            ApicaTypeBytecode::Any | ApicaTypeBytecode::Null => Some(ValueType::new(ApicaTypeBytecode::Bool, false)),

            ApicaTypeBytecode::Reference => if self.contained[0].type_equals(&other.contained()[0]) || other.value == ApicaTypeBytecode::Null {
                Some(ValueType::new(ApicaTypeBytecode::Bool, false))
            } else { None },

            _ if self.is_number() || matches!(self.value, ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char)
                => ValueType::number_comparison_resolve_to(other, false),

            _ => if self.primitive() == other.primitive() || matches!(other.value, ApicaTypeBytecode::Null) {
                Some(ValueType::new(ApicaTypeBytecode::Bool, false))
            } else {
                None
            }
        }
    }

    fn resolve_type_basic_binary_operations(&self, other: &ValueType, is_addition: bool) -> Option<ValueType> {
        match self.value {
            ApicaTypeBytecode::Any => Some(ValueType::new(ApicaTypeBytecode::Any, true)),
            ApicaTypeBytecode::Null => None,

            ApicaTypeBytecode::String => if is_addition && !matches!(other.value, ApicaTypeBytecode::Null) {
                Some(ValueType::new(ApicaTypeBytecode::String, true))
            } else {
                None
            },

            ApicaTypeBytecode::Int => match other.value {
                ApicaTypeBytecode::Any => Some(ValueType::new(ApicaTypeBytecode::Any, true)),

                ApicaTypeBytecode::Int | ApicaTypeBytecode::UnsignedInt | ApicaTypeBytecode::Bool  | ApicaTypeBytecode::Char
                    => Some(ValueType::new(ApicaTypeBytecode::Int, true)),

                ApicaTypeBytecode::Float => Some(ValueType::new(ApicaTypeBytecode::Float, true)),

                _ => None,
            },

            ApicaTypeBytecode::UnsignedInt => match other.value {
                ApicaTypeBytecode::Any => Some(ValueType::new(ApicaTypeBytecode::Any, true)),

                ApicaTypeBytecode::Int => Some(ValueType::new(ApicaTypeBytecode::Int, true)),
                ApicaTypeBytecode::UnsignedInt | ApicaTypeBytecode::Bool  | ApicaTypeBytecode::Char
                    => Some(ValueType::new(ApicaTypeBytecode::UnsignedInt, true)),

                ApicaTypeBytecode::Float => Some(ValueType::new(ApicaTypeBytecode::Float, true)),

                _ => None,
            },

            ApicaTypeBytecode::Float => match other.value {
                ApicaTypeBytecode::Any => Some(ValueType::new(ApicaTypeBytecode::Any, true)),

                ApicaTypeBytecode::Int | ApicaTypeBytecode::UnsignedInt | ApicaTypeBytecode::Float
                | ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char
                    => Some(ValueType::new(ApicaTypeBytecode::Float, true)),

                _ => None,
            },

            _ => None,
        }
    }

    fn resolve_type_shift(&self, other: &ValueType) -> Option<ValueType> {
        match self.value {
            ApicaTypeBytecode::Any => Some(ValueType::new(ApicaTypeBytecode::Any, true)),

            _ if self.is_number() && other.is_number() => Some(self.clone()),

            _ => None,
        }
    }

    fn resolve_type_assign(&self, other: &ValueType) -> Option<ValueType> {
        match self.value {
            ApicaTypeBytecode::Any => Some(ValueType::new(ApicaTypeBytecode::Any, true)),
            ApicaTypeBytecode::Null => None,

            ApicaTypeBytecode::Reference => if self.contained[0].type_equals(&other.contained()[0]) {
                Some(self.clone())
            } else { None },

            _ if self.is_number() && (other.is_number() || other.value == ApicaTypeBytecode::Null)
                => Some(self.clone()),

            _ => if self.primitive() == other.primitive() || other.value == ApicaTypeBytecode::Null {
                Some(self.clone())
            } else {
                None
            },
        }
    }

    fn resolve_type_convert(&self, other: &ValueType) -> Option<ValueType> {
        if self.can_be_converted_to(other, false) {
            Some(other.clone())
        } else {
            None
        }
    }
    
    pub fn can_be_converted_to(&self, to: &ValueType, is_auto: bool) -> bool {
        match self.value {
            ApicaTypeBytecode::Null | ApicaTypeBytecode::Any => true,
            ApicaTypeBytecode::Int | ApicaTypeBytecode::UnsignedInt => ValueType::number_can_convert_to(to, is_auto),
            ApicaTypeBytecode::Float | ApicaTypeBytecode::Bool => ValueType::decimal_can_convert_to(to, is_auto),

            ApicaTypeBytecode::Char => match to.value {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Int | ApicaTypeBytecode::UnsignedInt
                | ApicaTypeBytecode::Float | ApicaTypeBytecode::Char => true,

                ApicaTypeBytecode::Bool | ApicaTypeBytecode::String | ApicaTypeBytecode::Type => !is_auto,

                _ => false,
            },

            ApicaTypeBytecode::String => match to.value {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::String => true,
                ApicaTypeBytecode::Bool | ApicaTypeBytecode::Type => !is_auto,
                _ => false,
            },

            ApicaTypeBytecode::Type => match to.value {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Type => true,
                ApicaTypeBytecode::Bool | ApicaTypeBytecode::String => !is_auto,
                _ => false,
            },

            ApicaTypeBytecode::Error => match to.value {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Error => true,
                ApicaTypeBytecode::Bool | ApicaTypeBytecode::String | ApicaTypeBytecode::Type => !is_auto,
                _ => false,
            },
            
            ApicaTypeBytecode::Reference => match to.value { 
                ApicaTypeBytecode::Any => true,
                ApicaTypeBytecode::Reference => self.contained[0].type_equals(&to.contained()[0]),
                _ => false,
            },
        }
    }

    pub fn resolve_type_operators(&self, other: &ValueType, operator: ApicaBytecode) -> Option<ValueType> {
        match operator {
            ApicaBytecode::Increment | ApicaBytecode::LeftIncrement | ApicaBytecode::Decrement | ApicaBytecode::LeftDecrement
            => self.resolve_type_increment_decrement(),

            ApicaBytecode::BitwiseNot => self.resolve_type_bitwise_not(),

            ApicaBytecode::Not => self.resolve_type_unary_not(),

            ApicaBytecode::LessThan | ApicaBytecode::LessOrEquals | ApicaBytecode::GreaterThan | ApicaBytecode::GreaterOrEquals
            => self.resolve_type_compare(&other),

            ApicaBytecode::Equals | ApicaBytecode::NotEquals => self.resolve_type_equality(&other),

            ApicaBytecode::Add => self.resolve_type_basic_binary_operations(&other, true),
            ApicaBytecode::Subtract | ApicaBytecode::Multiply | ApicaBytecode::Divide | ApicaBytecode::Modulo
            | ApicaBytecode::BitwiseOr | ApicaBytecode::BitwiseAnd | ApicaBytecode::BitwiseXor
            => self.resolve_type_basic_binary_operations(&other, false),

            ApicaBytecode::LeftShift | ApicaBytecode::RightShift => self.resolve_type_shift(&other),

            ApicaBytecode::Assign => self.resolve_type_assign(&other),

            ApicaBytecode::As => self.resolve_type_convert(other),

            ApicaBytecode::SpecialOp => Some(ValueType::new(ApicaTypeBytecode::Any, true)),
            _ => None,
        }
    }
}

impl ValueTrait for ValueType {
    fn is_null(&self) -> bool {
        false
    }

    fn get_type_repr(&self) -> String {
        String::from("type")
    }

    fn show(&self, end: char) {
        if self.is_nullable {
            print!("type<{}!>{}", self.value.repr(), end);
        } else {
            print!("type<{}>{}", self.value.repr(), end);
        }
    }

    fn repr(&self) -> String {
        if self.is_nullable {
            format!("type<{}!>", self.value.repr())
        } else {
            format!("type<{}>", self.value.repr())
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
                Some(Value::Type(Box::new(self.clone())))
            },
            
            _ => None,
        }
    }
    
    fn convert(&self, to: &ValueType, _is_nullable: bool) -> Option<Value> {
        match to.value {
            ApicaTypeBytecode::String => Some(Value::String(ValueString::with_value(self.repr()))),
            ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::with_value(self.value != ApicaTypeBytecode::Null))),

            _ => None,
        }
    }

    fn auto_convert(&self, to: &ValueType, _is_nullable: bool) -> Option<Value> {
        match to.value {
            ApicaTypeBytecode::Any => Some(Value::Type(Box::new(self.clone()))),
            ApicaTypeBytecode::Type => Some(Value::Type(Box::new(self.clone()))),

            _ => None,
        }
    }
}