use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::error::ValueError;
use crate::values::float::ValueFloat;
use crate::values::int::ValueInt;
use crate::values::reference::ValueReference;
use crate::values::string::ValueString;
use crate::values::uint::ValueUInt;
use crate::values::value::{Value, ValueTrait};
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct ValueNull {

}

impl ValueNull {
    pub fn new() -> ValueNull {
        ValueNull {}
    }
}

impl ValueTrait for ValueNull {
    fn is_null(&self) -> bool {
        true
    }

    fn get_type_repr(&self) -> String {
        String::from("null")
    }

    fn show(&self, end: char) {
        print!("null<>{}", end);
    }

    fn repr(&self) -> String {
        String::from("null<>")
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
        Some(Value::Bool(ValueBool::with_value(true)))
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
        Some(Value::Bool(ValueBool::with_value(other.is_null())))
    }

    fn not_equals(&self, other: &Value) -> Option<Value> {
        Some(Value::Bool(ValueBool::with_value(!other.is_null())))
    }

    fn left_shift(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn right_shift(&self, _other: &Value) -> Option<Value> {
        None
    }

    fn assign(&mut self, _other: &Value) -> Option<Value> {
        None
    }
    
    fn convert(&self, _to: &ValueType, _is_nullable: bool) -> Option<Value> {
        None // null is AUTOMATICALLY converted
    }

    fn auto_convert(&self, to: &ValueType, is_nullable: bool) -> Option<Value> {
        Some(match to.value() {
            ApicaTypeBytecode::Any | ApicaTypeBytecode::Null => Value::Null(ValueNull::new()),
            ApicaTypeBytecode::Int => Value::Int(ValueInt::new()),
            ApicaTypeBytecode::UnsignedInt => Value::UInt(ValueUInt::new()),
            ApicaTypeBytecode::Float => Value::Float(ValueFloat::new()),
            ApicaTypeBytecode::Bool => Value::Bool(ValueBool::new()),
            ApicaTypeBytecode::Char => Value::Char(ValueChar::new()),
            ApicaTypeBytecode::String => Value::String(ValueString::new()),
            ApicaTypeBytecode::Error => Value::Error(Box::new(ValueError::new())),
            ApicaTypeBytecode::Type => Value::Type(Box::new(ValueType::new(ApicaTypeBytecode::Null, is_nullable))),

            ApicaTypeBytecode::Reference => Value::Reference(Box::new(ValueReference::new(
                to.contained()[0].clone()
            ))),
        })
    }
}