use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::error::ValueError;
use crate::values::float::ValueFloat;
use crate::values::int::ValueInt;
use crate::values::null::ValueNull;
use crate::values::reference::ValueReference;
use crate::values::string::ValueString;
use crate::values::uint::ValueUInt;
use crate::values::value_type::ValueType;

pub trait ValueTrait {
    fn is_null(&self) -> bool;
    fn get_type_repr(&self) -> String;
    fn show(&self, end: char);
    fn repr(&self) -> String;

    fn add(&self, other: &Value) -> Option<Value>;
    fn increment(&mut self) -> Option<Value>;
    fn left_increment(&mut self) -> Option<Value>;
    fn subtract(&self, other: &Value) -> Option<Value>;
    fn decrement(&mut self) -> Option<Value>;
    fn left_decrement(&mut self) -> Option<Value>;
    fn times(&self, other: &Value) -> Option<Value>;

    fn unary_not(&self) -> Option<Value>;
    fn bitwise_not(&self) -> Option<Value>;
    fn bitwise_or(&self, other: &Value) -> Option<Value>;
    fn bitwise_xor(&self, other: &Value) -> Option<Value>;
    fn bitwise_and(&self, other: &Value) -> Option<Value>;

    fn less_than(&self, other: &Value) -> Option<Value>;
    fn less_or_equal(&self, other: &Value) -> Option<Value>;
    fn greater_than(&self, other: &Value) -> Option<Value>;
    fn greater_or_equal(&self, other: &Value) -> Option<Value>;
    fn equals(&self, other: &Value) -> Option<Value>;
    fn not_equals(&self, other: &Value) -> Option<Value>;
    
    fn left_shift(&self, other: &Value) -> Option<Value>;
    fn right_shift(&self, other: &Value) -> Option<Value>;

    fn assign(&mut self, other: &Value) -> Option<Value>;
    
    fn convert(&self, to: &ValueType, is_nullable: bool) -> Option<Value>;
    fn auto_convert(&self, to: &ValueType, is_nullable: bool) -> Option<Value>;
}

#[derive(Clone)]
pub enum Value {
    Null(ValueNull),
    Int(ValueInt),
    UInt(ValueUInt),
    Float(ValueFloat),
    Bool(ValueBool),
    Char(ValueChar),
    String(ValueString),
    Error(Box<ValueError>),
    Type(Box<ValueType>),
    Reference(Box<ValueReference>),
}

impl Value {
    pub fn null_operation_error(op: &str, is_unary: bool) -> Value {
        let operation_kind = if is_unary { "unary" } else { "binary" };

        Value::Error(Box::from(ValueError::with_details(
            String::from("OperationError"),
            format!("Cannot perform {} operation `{}` with a null value", operation_kind, op)
        )))
    }

    pub fn unary_operation_error(op: &str, operand: &str) -> Value {
        Value::Error(Box::from(ValueError::with_details(
            String::from("OperationError"),
            format!("Unary operator `{}` is not defined for type <{}>", op, operand)
        )))
    }

    pub fn binary_operation_error(op: &str, left: &str, right: &str) -> Value {
        Value::Error(Box::from(ValueError::with_details(
            String::from("OperationError"),
            format!("Binary operator `{}` is not defined for types <{}> and <{}>", op, left, right)
        )))
    }
    
    pub fn constant_operation_error(op: &str) -> Value {
        Value::Error(Box::from(ValueError::with_details(
            String::from("ConstantError"),
            format!("Cannot perform binary operation `{}` with a constant", op)
        )))
    }

    pub fn not_nullable_error(op: &str) -> Value {
        Value::Error(Box::from(ValueError::with_details(
            String::from("NotNullableError"),
            format!("Cannot perform operation `{}` with a not-nullable variable", op)
        )))
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Null(_) => ValueType::new(ApicaTypeBytecode::Null, true),
            Value::Int(_) => ValueType::new(ApicaTypeBytecode::Int, true),
            Value::UInt(_) => ValueType::new(ApicaTypeBytecode::UnsignedInt, true),
            Value::Float(_) => ValueType::new(ApicaTypeBytecode::Float, true),
            Value::Bool(_) => ValueType::new(ApicaTypeBytecode::Bool, true),
            Value::Char(_) => ValueType::new(ApicaTypeBytecode::Char, true),
            Value::String(_) => ValueType::new(ApicaTypeBytecode::String, true),
            Value::Error(_) => ValueType::new(ApicaTypeBytecode::Error, true),
            Value::Type(_) => ValueType::new(ApicaTypeBytecode::Type, true),
            
            Value::Reference(v) => ValueType::with_contained(
                ApicaTypeBytecode::Reference, 
                true,
                vec![v.contained().clone()],
            ),
        }
    }
}

impl ValueTrait for Value {
    fn is_null(&self) -> bool {
        match self {
            Value::Null(v) => v.is_null(),
            Value::Int(v) => v.is_null(),
            Value::UInt(v) => v.is_null(),
            Value::Float(v) => v.is_null(),
            Value::Bool(v) => v.is_null(),
            Value::Char(v) => v.is_null(),
            Value::String(v) => v.is_null(),
            Value::Error(v) => v.is_null(),
            Value::Type(v) => v.is_null(),
            Value::Reference(v) => v.is_null(),
        }
    }

    fn get_type_repr(&self) -> String {
        match self {
            Value::Null(v) => v.get_type_repr(),
            Value::Int(v) => v.get_type_repr(),
            Value::UInt(v) => v.get_type_repr(),
            Value::Float(v) => v.get_type_repr(),
            Value::Bool(v) => v.get_type_repr(),
            Value::Char(v) => v.get_type_repr(),
            Value::String(v) => v.get_type_repr(),
            Value::Error(v) => v.get_type_repr(),
            Value::Type(v) => v.get_type_repr(),
            Value::Reference(v) => v.get_type_repr(),
        }
    }

    fn show(&self, end: char) {
        match self {
            Value::Null(v) => v.show(end),
            Value::Int(v) => v.show(end),
            Value::UInt(v) => v.show(end),
            Value::Float(v) => v.show(end),
            Value::Bool(v) => v.show(end),
            Value::Char(v) => v.show(end),
            Value::String(v) => v.show(end),
            Value::Error(v) => v.show(end),
            Value::Type(v) => v.show(end),
            Value::Reference(v) => v.show(end),
        }
    }

    fn repr(&self) -> String {
        match self { 
            Value::Null(v) => v.repr(),
            Value::Int(v) => v.repr(),
            Value::UInt(v) => v.repr(),
            Value::Float(v) => v.repr(),
            Value::Bool(v) => v.repr(),
            Value::Char(v) => v.repr(),
            Value::String(v) => v.repr(),
            Value::Error(v) => v.repr(),
            Value::Type(v) => v.repr(),
            Value::Reference(v) => v.repr(),
        }
    }

    fn add(&self, other: &Value) -> Option<Value> {
        match self {
            Value::Null(v) => v.add(other),
            Value::Int(v) => v.add(other),
            Value::UInt(v) => v.add(other),
            Value::Float(v) => v.add(other),
            Value::Bool(v) => v.add(other),
            Value::Char(v) => v.add(other),
            Value::String(v) => v.add(other),
            Value::Error(v) => v.add(other),
            Value::Type(v) => v.add(other),
            Value::Reference(v) => v.add(other),
        }
    }

    fn increment(&mut self) -> Option<Value> {
        match self {
            Value::Null(v) => v.increment(),
            Value::Int(v) => v.increment(),
            Value::UInt(v) => v.increment(),
            Value::Float(v) => v.increment(),
            Value::Bool(v) => v.increment(),
            Value::Char(v) => v.increment(),
            Value::String(v) => v.increment(),
            Value::Error(v) => v.increment(),
            Value::Type(v) => v.increment(),
            Value::Reference(v) => v.increment(),
        }
    }

    fn left_increment(&mut self) -> Option<Value> {
        match self {
            Value::Null(v) => v.left_increment(),
            Value::Int(v) => v.left_increment(),
            Value::UInt(v) => v.left_increment(),
            Value::Float(v) => v.left_increment(),
            Value::Bool(v) => v.left_increment(),
            Value::Char(v) => v.left_increment(),
            Value::String(v) => v.left_increment(),
            Value::Error(v) => v.left_increment(),
            Value::Type(v) => v.left_increment(),
            Value::Reference(v) => v.left_increment(),
        }
    }

    fn subtract(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.subtract(other),
            Value::Int(v) => v.subtract(other),
            Value::UInt(v) => v.subtract(other),
            Value::Float(v) => v.subtract(other),
            Value::Bool(v) => v.subtract(other),
            Value::Char(v) => v.subtract(other),
            Value::String(v) => v.subtract(other),
            Value::Error(v) => v.subtract(other),
            Value::Type(v) => v.subtract(other),
            Value::Reference(v) => v.subtract(other),
        }
    }

    fn decrement(&mut self) -> Option<Value> {
        match self { 
            Value::Null(v) => v.decrement(),
            Value::Int(v) => v.decrement(),
            Value::UInt(v) => v.decrement(),
            Value::Float(v) => v.decrement(),
            Value::Bool(v) => v.decrement(),
            Value::Char(v) => v.decrement(),
            Value::String(v) => v.decrement(),
            Value::Error(v) => v.decrement(),
            Value::Type(v) => v.decrement(),
            Value::Reference(v) => v.decrement(),
        }
    }

    fn left_decrement(&mut self) -> Option<Value> {
        match self { 
            Value::Null(v) => v.left_decrement(),
            Value::Int(v) => v.left_decrement(),
            Value::UInt(v) => v.left_decrement(),
            Value::Float(v) => v.left_decrement(),
            Value::Bool(v) => v.left_decrement(),
            Value::Char(v) => v.left_decrement(),
            Value::String(v) => v.left_decrement(),
            Value::Error(v) => v.left_decrement(),
            Value::Type(v) => v.left_decrement(),
            Value::Reference(v) => v.left_decrement(),
        }
    }

    fn times(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.times(other),
            Value::Int(v) => v.times(other),
            Value::UInt(v) => v.times(other),
            Value::Float(v) => v.times(other),
            Value::Bool(v) => v.times(other),
            Value::Char(v) => v.times(other),
            Value::String(v) => v.times(other),
            Value::Error(v) => v.times(other),
            Value::Type(v) => v.times(other),
            Value::Reference(v) => v.times(other),
        }
    }

    fn unary_not(&self) -> Option<Value> {
        match self { 
            Value::Null(v) => v.unary_not(),
            Value::Int(v) => v.unary_not(),
            Value::UInt(v) => v.unary_not(),
            Value::Float(v) => v.unary_not(),
            Value::Bool(v) => v.unary_not(),
            Value::Char(v) => v.unary_not(),
            Value::String(v) => v.unary_not(),
            Value::Error(v) => v.unary_not(),
            Value::Type(v) => v.unary_not(),
            Value::Reference(v) => v.unary_not(),
        }
    }

    fn bitwise_not(&self) -> Option<Value> {
        match self { 
            Value::Null(v) => v.bitwise_not(),
            Value::Int(v) => v.bitwise_not(),
            Value::UInt(v) => v.bitwise_not(),
            Value::Float(v) => v.bitwise_not(),
            Value::Bool(v) => v.bitwise_not(),
            Value::Char(v) => v.bitwise_not(),
            Value::String(v) => v.bitwise_not(),
            Value::Error(v) => v.bitwise_not(),
            Value::Type(v) => v.bitwise_not(),
            Value::Reference(v) => v.bitwise_not(),
        }
    }

    fn bitwise_or(&self, other: &Value) -> Option<Value> {
        match self {
            Value::Null(v) => v.bitwise_or(other),
            Value::Int(v) => v.bitwise_or(other),
            Value::UInt(v) => v.bitwise_or(other),
            Value::Float(v) => v.bitwise_or(other),
            Value::Bool(v) => v.bitwise_or(other),
            Value::Char(v) => v.bitwise_or(other),
            Value::String(v) => v.bitwise_or(other),
            Value::Error(v) => v.bitwise_or(other),
            Value::Type(v) => v.bitwise_or(other),
            Value::Reference(v) => v.bitwise_or(other),
        }
    }

    fn bitwise_xor(&self, other: &Value) -> Option<Value> {
        match self {
            Value::Null(v) => v.bitwise_xor(other),
            Value::Int(v) => v.bitwise_xor(other),
            Value::UInt(v) => v.bitwise_xor(other),
            Value::Float(v) => v.bitwise_xor(other),
            Value::Bool(v) => v.bitwise_xor(other),
            Value::Char(v) => v.bitwise_xor(other),
            Value::String(v) => v.bitwise_xor(other),
            Value::Error(v) => v.bitwise_xor(other),
            Value::Type(v) => v.bitwise_xor(other),
            Value::Reference(v) => v.bitwise_xor(other),
        }
    }

    fn bitwise_and(&self, other: &Value) -> Option<Value> {
        match self {
            Value::Null(v) => v.bitwise_and(other),
            Value::Int(v) => v.bitwise_and(other),
            Value::UInt(v) => v.bitwise_and(other),
            Value::Float(v) => v.bitwise_and(other),
            Value::Bool(v) => v.bitwise_and(other),
            Value::Char(v) => v.bitwise_and(other),
            Value::String(v) => v.bitwise_and(other),
            Value::Error(v) => v.bitwise_and(other),
            Value::Type(v) => v.bitwise_and(other),
            Value::Reference(v) => v.bitwise_and(other),
        }
    }

    fn less_than(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.less_than(other),
            Value::Int(v) => v.less_than(other),
            Value::UInt(v) => v.less_than(other),
            Value::Float(v) => v.less_than(other),
            Value::Bool(v) => v.less_than(other),
            Value::Char(v) => v.less_than(other),
            Value::String(v) => v.less_than(other),
            Value::Error(v) => v.less_than(other),
            Value::Type(v) => v.less_than(other),
            Value::Reference(v) => v.less_than(other),
        }
    }

    fn less_or_equal(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.less_or_equal(other),
            Value::Int(v) => v.less_or_equal(other),
            Value::UInt(v) => v.less_or_equal(other),
            Value::Float(v) => v.less_or_equal(other),
            Value::Bool(v) => v.less_or_equal(other),
            Value::Char(v) => v.less_or_equal(other),
            Value::String(v) => v.less_or_equal(other),
            Value::Error(v) => v.less_or_equal(other),
            Value::Type(v) => v.less_or_equal(other),
            Value::Reference(v) => v.less_or_equal(other),
        }
    }

    fn greater_than(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.greater_than(other),
            Value::Int(v) => v.greater_than(other),
            Value::UInt(v) => v.greater_than(other),
            Value::Float(v) => v.greater_than(other),
            Value::Bool(v) => v.greater_than(other),
            Value::Char(v) => v.greater_than(other),
            Value::String(v) => v.greater_than(other),
            Value::Error(v) => v.greater_than(other),
            Value::Type(v) => v.greater_than(other),
            Value::Reference(v) => v.greater_than(other),
        }
    }

    fn greater_or_equal(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.greater_or_equal(other),
            Value::Int(v) => v.greater_or_equal(other),
            Value::UInt(v) => v.greater_or_equal(other),
            Value::Float(v) => v.greater_or_equal(other),
            Value::Bool(v) => v.greater_or_equal(other),
            Value::Char(v) => v.greater_or_equal(other),
            Value::String(v) => v.greater_or_equal(other),
            Value::Error(v) => v.greater_or_equal(other),
            Value::Type(v) => v.greater_or_equal(other),
            Value::Reference(v) => v.greater_or_equal(other),
        }
    }

    fn equals(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.equals(other),
            Value::Int(v) => v.equals(other),
            Value::UInt(v) => v.equals(other),
            Value::Float(v) => v.equals(other),
            Value::Bool(v) => v.equals(other),
            Value::Char(v) => v.equals(other),
            Value::String(v) => v.equals(other),
            Value::Error(v) => v.equals(other),
            Value::Type(v) => v.equals(other),
            Value::Reference(v) => v.equals(other),
        }
    }

    fn not_equals(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.not_equals(other),
            Value::Int(v) => v.not_equals(other),
            Value::UInt(v) => v.not_equals(other),
            Value::Float(v) => v.not_equals(other),
            Value::Bool(v) => v.not_equals(other),
            Value::Char(v) => v.not_equals(other),
            Value::String(v) => v.not_equals(other),
            Value::Error(v) => v.not_equals(other),
            Value::Type(v) => v.not_equals(other),
            Value::Reference(v) => v.not_equals(other),
        }
    }

    fn left_shift(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.left_shift(other),
            Value::Int(v) => v.left_shift(other),
            Value::UInt(v) => v.left_shift(other),
            Value::Float(v) => v.left_shift(other),
            Value::Bool(v) => v.left_shift(other),
            Value::Char(v) => v.left_shift(other),
            Value::String(v) => v.left_shift(other),
            Value::Error(v) => v.left_shift(other),
            Value::Type(v) => v.left_shift(other),
            Value::Reference(v) => v.left_shift(other),
        }
    }

    fn right_shift(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.right_shift(other),
            Value::Int(v) => v.right_shift(other),
            Value::UInt(v) => v.right_shift(other),
            Value::Float(v) => v.right_shift(other),
            Value::Bool(v) => v.right_shift(other),
            Value::Char(v) => v.right_shift(other),
            Value::String(v) => v.right_shift(other),
            Value::Error(v) => v.right_shift(other),
            Value::Type(v) => v.right_shift(other),
            Value::Reference(v) => v.right_shift(other),
        }
    }

    fn assign(&mut self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.assign(other),
            Value::Int(v) => v.assign(other),
            Value::UInt(v) => v.assign(other),
            Value::Float(v) => v.assign(other),
            Value::Bool(v) => v.assign(other),
            Value::Char(v) => v.assign(other),
            Value::String(v) => v.assign(other),
            Value::Error(v) => v.assign(other),
            Value::Type(v) => v.assign(other),
            Value::Reference(v) => v.assign(other),
        }
    }

    fn convert(&self, to: &ValueType, is_nullable: bool) -> Option<Value> {
        match self { 
            Value::Null(v) => v.convert(to, is_nullable),
            Value::Int(v) => v.convert(to, is_nullable),
            Value::UInt(v) => v.convert(to, is_nullable),
            Value::Float(v) => v.convert(to, is_nullable),
            Value::Bool(v) => v.convert(to, is_nullable),
            Value::Char(v) => v.convert(to, is_nullable),
            Value::String(v) => v.convert(to, is_nullable),
            Value::Error(v) => v.convert(to, is_nullable),
            Value::Type(v) => v.convert(to, is_nullable),
            Value::Reference(v) => v.convert(to, is_nullable),
        }
    }

    fn auto_convert(&self, to: &ValueType, is_nullable: bool) -> Option<Value> {
        match self { 
            Value::Null(v) => v.auto_convert(to, is_nullable),
            Value::Int(v) => v.auto_convert(to, is_nullable),
            Value::UInt(v) => v.auto_convert(to, is_nullable),
            Value::Float(v) => v.auto_convert(to, is_nullable),
            Value::Bool(v) => v.auto_convert(to, is_nullable),
            Value::Char(v) => v.auto_convert(to, is_nullable),
            Value::String(v) => v.auto_convert(to, is_nullable),
            Value::Error(v) => v.auto_convert(to, is_nullable),
            Value::Type(v) => v.auto_convert(to, is_nullable),
            Value::Reference(v) => v.auto_convert(to, is_nullable),
        }
    }
}