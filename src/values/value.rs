use crate::bytecodes::types::ApicaTypeBytecode;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::error::ValueError;
use crate::values::f32::ValueF32;
use crate::values::f64::ValueF64;
use crate::values::i16::ValueI16;
use crate::values::i32::ValueI32;
use crate::values::i64::ValueI64;
use crate::values::i8::ValueI8;
use crate::values::null::ValueNull;
use crate::values::stack_trace::ValueStackTrace;
use crate::values::string::ValueString;
use crate::values::u16::ValueU16;
use crate::values::u32::ValueU32;
use crate::values::u64::ValueU64;
use crate::values::u8::ValueU8;
use crate::values::value_type::ValueType;

pub trait ValueTrait {
    fn is_null(&self) -> bool;
    fn get_type_repr(&self) -> &str;
    fn show(&self, end: char);

    fn add(&self, other: &Value) -> Option<Value>;
    fn increment(&mut self) -> Option<Value>;
    fn left_increment(&mut self) -> Option<Value>;
    fn subtract(&self, other: &Value) -> Option<Value>;
    fn decrement(&mut self) -> Option<Value>;
    fn left_decrement(&mut self) -> Option<Value>;
    fn times(&self, other: &Value) -> Option<Value>;

    fn unary_not(&self) -> Option<Value>;
    fn bitwise_not(&self) -> Option<Value>;

    fn less_than(&self, other: &Value) -> Option<Value>;
    fn less_or_equal(&self, other: &Value) -> Option<Value>;
    fn greater_than(&self, other: &Value) -> Option<Value>;
    fn greater_or_equal(&self, other: &Value) -> Option<Value>;
    fn equals(&self, other: &Value) -> Option<Value>;
    fn not_equals(&self, other: &Value) -> Option<Value>;

    fn assign(&mut self, other: &Value) -> Option<Value>;
    
    fn convert(&self, to: ApicaTypeBytecode) -> Option<Value>;
    fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value>;
    
    fn copy(&self) -> Value;
}

pub enum Value {
    Null(ValueNull),
    I8(ValueI8),
    I16(ValueI16),
    I32(ValueI32),
    I64(ValueI64),
    U8(ValueU8),
    U16(ValueU16),
    U32(ValueU32),
    U64(ValueU64),
    F32(ValueF32),
    F64(ValueF64),
    Bool(ValueBool),
    Char(ValueChar),
    String(ValueString),
    Error(Box<ValueError>),
    StackTrace(Box<ValueStackTrace>),
    Type(ValueType),
}

impl Value {
    pub fn null_operation_error(op: &str, is_unary: bool) -> Value {
        let operation_kind = if is_unary { "unary" } else { "binary" };

        Value::StackTrace(Box::from(ValueStackTrace::with_details(
            String::from("OperationError"),
            format!("Cannot perform {} operation `{}` with a null value", operation_kind, op)
        )))
    }

    pub fn unary_operation_error(op: &str, operand: &str) -> Value {
        Value::StackTrace(Box::from(ValueStackTrace::with_details(
            String::from("OperationError"),
            format!("Unary operator `{}` is not defined for type <{}>", op, operand)
        )))
    }

    pub fn binary_operation_error(op: &str, left: &str, right: &str) -> Value {
        Value::StackTrace(Box::from(ValueStackTrace::with_details(
            String::from("OperationError"),
            format!("Binary operator `{}` is not defined for types <{}> and <{}>", op, left, right)
        )))
    }
    
    pub fn constant_operation_error(op: &str) -> Value {
        Value::StackTrace(Box::from(ValueStackTrace::with_details(
            String::from("ConstantError"),
            format!("Cannot perform binary operation `{}` with a constant", op)
        )))
    }

    pub fn not_nullable_error(op: &str) -> Value {
        Value::StackTrace(Box::from(ValueStackTrace::with_details(
            String::from("NotNullableError"),
            format!("Cannot perform operation `{}` with a not-nullable variable", op)
        )))
    }

    pub fn value_type(&self) -> ApicaTypeBytecode {
        match self {
            Value::Null(_) | Value::StackTrace(_) => ApicaTypeBytecode::Null,
            Value::I8(_) => ApicaTypeBytecode::I8,
            Value::I16(_) => ApicaTypeBytecode::I16,
            Value::I32(_) => ApicaTypeBytecode::I32,
            Value::I64(_) => ApicaTypeBytecode::I64,
            Value::U8(_) => ApicaTypeBytecode::U8,
            Value::U16(_) => ApicaTypeBytecode::U16,
            Value::U32(_) => ApicaTypeBytecode::U32,
            Value::U64(_) => ApicaTypeBytecode::U64,
            Value::F32(_) => ApicaTypeBytecode::F32,
            Value::F64(_) => ApicaTypeBytecode::F64,
            Value::Bool(_) => ApicaTypeBytecode::Bool,
            Value::Char(_) => ApicaTypeBytecode::Char,
            Value::String(_) => ApicaTypeBytecode::String,
            Value::Error(_) => ApicaTypeBytecode::Error,
            Value::Type(_) => ApicaTypeBytecode::Type,
        }
    }
}

impl ValueTrait for Value {
    fn is_null(&self) -> bool {
        match self {
            Value::Null(v) => v.is_null(),
            Value::I8(v) => v.is_null(),
            Value::I16(v) => v.is_null(),
            Value::I32(v) => v.is_null(),
            Value::I64(v) => v.is_null(),
            Value::U8(v) => v.is_null(),
            Value::U16(v) => v.is_null(),
            Value::U32(v) => v.is_null(),
            Value::U64(v) => v.is_null(),
            Value::F32(v) => v.is_null(),
            Value::F64(v) => v.is_null(),
            Value::Bool(v) => v.is_null(),
            Value::Char(v) => v.is_null(),
            Value::String(v) => v.is_null(),
            Value::Error(v) => v.is_null(),
            Value::StackTrace(v) => v.is_null(),
            Value::Type(v) => v.is_null(),
        }
    }

    fn get_type_repr(&self) -> &str {
        match self {
            Value::Null(v) => v.get_type_repr(),
            Value::I8(v) => v.get_type_repr(),
            Value::I16(v) => v.get_type_repr(),
            Value::I32(v) => v.get_type_repr(),
            Value::I64(v) => v.get_type_repr(),
            Value::U8(v) => v.get_type_repr(),
            Value::U16(v) => v.get_type_repr(),
            Value::U32(v) => v.get_type_repr(),
            Value::U64(v) => v.get_type_repr(),
            Value::F32(v) => v.get_type_repr(),
            Value::F64(v) => v.get_type_repr(),
            Value::Bool(v) => v.get_type_repr(),
            Value::Char(v) => v.get_type_repr(),
            Value::String(v) => v.get_type_repr(),
            Value::Error(v) => v.get_type_repr(),
            Value::StackTrace(v) => v.get_type_repr(),
            Value::Type(v) => v.get_type_repr(),
        }
    }

    fn show(&self, end: char) {
        match self {
            Value::Null(v) => v.show(end),
            Value::I8(v) => v.show(end),
            Value::I16(v) => v.show(end),
            Value::I32(v) => v.show(end),
            Value::I64(v) => v.show(end),
            Value::U8(v) => v.show(end),
            Value::U16(v) => v.show(end),
            Value::U32(v) => v.show(end),
            Value::U64(v) => v.show(end),
            Value::F32(v) => v.show(end),
            Value::F64(v) => v.show(end),
            Value::Bool(v) => v.show(end),
            Value::Char(v) => v.show(end),
            Value::String(v) => v.show(end),
            Value::Error(v) => v.show(end),
            Value::StackTrace(v) => v.show(end),
            Value::Type(v) => v.show(end),
        }
    }

    fn add(&self, other: &Value) -> Option<Value> {
        match self {
            Value::Null(v) => v.add(other),
            Value::I8(v) => v.add(other),
            Value::I16(v) => v.add(other),
            Value::I32(v) => v.add(other),
            Value::I64(v) => v.add(other),
            Value::U8(v) => v.add(other),
            Value::U16(v) => v.add(other),
            Value::U32(v) => v.add(other),
            Value::U64(v) => v.add(other),
            Value::F32(v) => v.add(other),
            Value::F64(v) => v.add(other),
            Value::Bool(v) => v.add(other),
            Value::Char(v) => v.add(other),
            Value::String(v) => v.add(other),
            Value::Error(v) => v.add(other),
            Value::StackTrace(v) => v.add(other),
            Value::Type(v) => v.add(other),
        }
    }

    fn increment(&mut self) -> Option<Value> {
        match self {
            Value::Null(v) => v.increment(),
            Value::I8(v) => v.increment(),
            Value::I16(v) => v.increment(),
            Value::I32(v) => v.increment(),
            Value::I64(v) => v.increment(),
            Value::U8(v) => v.increment(),
            Value::U16(v) => v.increment(),
            Value::U32(v) => v.increment(),
            Value::U64(v) => v.increment(),
            Value::F32(v) => v.increment(),
            Value::F64(v) => v.increment(),
            Value::Bool(v) => v.increment(),
            Value::Char(v) => v.increment(),
            Value::String(v) => v.increment(),
            Value::Error(v) => v.increment(),
            Value::StackTrace(v) => v.increment(),
            Value::Type(v) => v.increment(),
        }
    }

    fn left_increment(&mut self) -> Option<Value> {
        match self {
            Value::Null(v) => v.left_increment(),
            Value::I8(v) => v.left_increment(),
            Value::I16(v) => v.left_increment(),
            Value::I32(v) => v.left_increment(),
            Value::I64(v) => v.left_increment(),
            Value::U8(v) => v.left_increment(),
            Value::U16(v) => v.left_increment(),
            Value::U32(v) => v.left_increment(),
            Value::U64(v) => v.left_increment(),
            Value::F32(v) => v.left_increment(),
            Value::F64(v) => v.left_increment(),
            Value::Bool(v) => v.left_increment(),
            Value::Char(v) => v.left_increment(),
            Value::String(v) => v.left_increment(),
            Value::Error(v) => v.left_increment(),
            Value::StackTrace(v) => v.left_increment(),
            Value::Type(v) => v.left_increment(),
        }
    }

    fn subtract(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.subtract(other),
            Value::I8(v) => v.subtract(other),
            Value::I16(v) => v.subtract(other),
            Value::I32(v) => v.subtract(other),
            Value::I64(v) => v.subtract(other),
            Value::U8(v) => v.subtract(other),
            Value::U16(v) => v.subtract(other),
            Value::U32(v) => v.subtract(other),
            Value::U64(v) => v.subtract(other),
            Value::F32(v) => v.subtract(other),
            Value::F64(v) => v.subtract(other),
            Value::Bool(v) => v.subtract(other),
            Value::Char(v) => v.subtract(other),
            Value::String(v) => v.subtract(other),
            Value::Error(v) => v.subtract(other),
            Value::StackTrace(v) => v.subtract(other),
            Value::Type(v) => v.subtract(other),
        }
    }

    fn decrement(&mut self) -> Option<Value> {
        match self { 
            Value::Null(v) => v.decrement(),
            Value::I8(v) => v.decrement(),
            Value::I16(v) => v.decrement(),
            Value::I32(v) => v.decrement(),
            Value::I64(v) => v.decrement(),
            Value::U8(v) => v.decrement(),
            Value::U16(v) => v.decrement(),
            Value::U32(v) => v.decrement(),
            Value::U64(v) => v.decrement(),
            Value::F32(v) => v.decrement(),
            Value::F64(v) => v.decrement(),
            Value::Bool(v) => v.decrement(),
            Value::Char(v) => v.decrement(),
            Value::String(v) => v.decrement(),
            Value::Error(v) => v.decrement(),
            Value::StackTrace(v) => v.decrement(),
            Value::Type(v) => v.decrement(),
        }
    }

    fn left_decrement(&mut self) -> Option<Value> {
        match self { 
            Value::Null(v) => v.left_decrement(),
            Value::I8(v) => v.left_decrement(),
            Value::I16(v) => v.left_decrement(),
            Value::I32(v) => v.left_decrement(),
            Value::I64(v) => v.left_decrement(),
            Value::U8(v) => v.left_decrement(),
            Value::U16(v) => v.left_decrement(),
            Value::U32(v) => v.left_decrement(),
            Value::U64(v) => v.left_decrement(),
            Value::F32(v) => v.left_decrement(),
            Value::F64(v) => v.left_decrement(),
            Value::Bool(v) => v.left_decrement(),
            Value::Char(v) => v.left_decrement(),
            Value::String(v) => v.left_decrement(),
            Value::Error(v) => v.left_decrement(),
            Value::StackTrace(v) => v.left_decrement(),
            Value::Type(v) => v.left_decrement(),
        }
    }

    fn times(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.times(other),
            Value::I8(v) => v.times(other),
            Value::I16(v) => v.times(other),
            Value::I32(v) => v.times(other),
            Value::I64(v) => v.times(other),
            Value::U8(v) => v.times(other),
            Value::U16(v) => v.times(other),
            Value::U32(v) => v.times(other),
            Value::U64(v) => v.times(other),
            Value::F32(v) => v.times(other),
            Value::F64(v) => v.times(other),
            Value::Bool(v) => v.times(other),
            Value::Char(v) => v.times(other),
            Value::String(v) => v.times(other),
            Value::Error(v) => v.times(other),
            Value::StackTrace(v) => v.times(other),
            Value::Type(v) => v.times(other),
        }
    }

    fn unary_not(&self) -> Option<Value> {
        match self { 
            Value::Null(v) => v.unary_not(),
            Value::I8(v) => v.unary_not(),
            Value::I16(v) => v.unary_not(),
            Value::I32(v) => v.unary_not(),
            Value::I64(v) => v.unary_not(),
            Value::U8(v) => v.unary_not(),
            Value::U16(v) => v.unary_not(),
            Value::U32(v) => v.unary_not(),
            Value::U64(v) => v.unary_not(),
            Value::F32(v) => v.unary_not(),
            Value::F64(v) => v.unary_not(),
            Value::Bool(v) => v.unary_not(),
            Value::Char(v) => v.unary_not(),
            Value::String(v) => v.unary_not(),
            Value::Error(v) => v.unary_not(),
            Value::StackTrace(v) => v.unary_not(),
            Value::Type(v) => v.unary_not(),
        }
    }

    fn bitwise_not(&self) -> Option<Value> {
        match self { 
            Value::Null(v) => v.bitwise_not(),
            Value::I8(v) => v.bitwise_not(),
            Value::I16(v) => v.bitwise_not(),
            Value::I32(v) => v.bitwise_not(),
            Value::I64(v) => v.bitwise_not(),
            Value::U8(v) => v.bitwise_not(),
            Value::U16(v) => v.bitwise_not(),
            Value::U32(v) => v.bitwise_not(),
            Value::U64(v) => v.bitwise_not(),
            Value::F32(v) => v.bitwise_not(),
            Value::F64(v) => v.bitwise_not(),
            Value::Bool(v) => v.bitwise_not(),
            Value::Char(v) => v.bitwise_not(),
            Value::String(v) => v.bitwise_not(),
            Value::Error(v) => v.bitwise_not(),
            Value::StackTrace(v) => v.bitwise_not(),
            Value::Type(v) => v.bitwise_not(),
        }
    }

    fn less_than(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.less_than(other),
            Value::I8(v) => v.less_than(other),
            Value::I16(v) => v.less_than(other),
            Value::I32(v) => v.less_than(other),
            Value::I64(v) => v.less_than(other),
            Value::U8(v) => v.less_than(other),
            Value::U16(v) => v.less_than(other),
            Value::U32(v) => v.less_than(other),
            Value::U64(v) => v.less_than(other),
            Value::F32(v) => v.less_than(other),
            Value::F64(v) => v.less_than(other),
            Value::Bool(v) => v.less_than(other),
            Value::Char(v) => v.less_than(other),
            Value::String(v) => v.less_than(other),
            Value::Error(v) => v.less_than(other),
            Value::StackTrace(v) => v.less_than(other),
            Value::Type(v) => v.less_than(other),
        }
    }

    fn less_or_equal(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.less_or_equal(other),
            Value::I8(v) => v.less_or_equal(other),
            Value::I16(v) => v.less_or_equal(other),
            Value::I32(v) => v.less_or_equal(other),
            Value::I64(v) => v.less_or_equal(other),
            Value::U8(v) => v.less_or_equal(other),
            Value::U16(v) => v.less_or_equal(other),
            Value::U32(v) => v.less_or_equal(other),
            Value::U64(v) => v.less_or_equal(other),
            Value::F32(v) => v.less_or_equal(other),
            Value::F64(v) => v.less_or_equal(other),
            Value::Bool(v) => v.less_or_equal(other),
            Value::Char(v) => v.less_or_equal(other),
            Value::String(v) => v.less_or_equal(other),
            Value::Error(v) => v.less_or_equal(other),
            Value::StackTrace(v) => v.less_or_equal(other),
            Value::Type(v) => v.less_or_equal(other),
        }
    }

    fn greater_than(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.greater_than(other),
            Value::I8(v) => v.greater_than(other),
            Value::I16(v) => v.greater_than(other),
            Value::I32(v) => v.greater_than(other),
            Value::I64(v) => v.greater_than(other),
            Value::U8(v) => v.greater_than(other),
            Value::U16(v) => v.greater_than(other),
            Value::U32(v) => v.greater_than(other),
            Value::U64(v) => v.greater_than(other),
            Value::F32(v) => v.greater_than(other),
            Value::F64(v) => v.greater_than(other),
            Value::Bool(v) => v.greater_than(other),
            Value::Char(v) => v.greater_than(other),
            Value::String(v) => v.greater_than(other),
            Value::Error(v) => v.greater_than(other),
            Value::StackTrace(v) => v.greater_than(other),
            Value::Type(v) => v.greater_than(other),
        }
    }

    fn greater_or_equal(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.greater_or_equal(other),
            Value::I8(v) => v.greater_or_equal(other),
            Value::I16(v) => v.greater_or_equal(other),
            Value::I32(v) => v.greater_or_equal(other),
            Value::I64(v) => v.greater_or_equal(other),
            Value::U8(v) => v.greater_or_equal(other),
            Value::U16(v) => v.greater_or_equal(other),
            Value::U32(v) => v.greater_or_equal(other),
            Value::U64(v) => v.greater_or_equal(other),
            Value::F32(v) => v.greater_or_equal(other),
            Value::F64(v) => v.greater_or_equal(other),
            Value::Bool(v) => v.greater_or_equal(other),
            Value::Char(v) => v.greater_or_equal(other),
            Value::String(v) => v.greater_or_equal(other),
            Value::Error(v) => v.greater_or_equal(other),
            Value::StackTrace(v) => v.greater_or_equal(other),
            Value::Type(v) => v.greater_or_equal(other),
        }
    }

    fn equals(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.equals(other),
            Value::I8(v) => v.equals(other),
            Value::I16(v) => v.equals(other),
            Value::I32(v) => v.equals(other),
            Value::I64(v) => v.equals(other),
            Value::U8(v) => v.equals(other),
            Value::U16(v) => v.equals(other),
            Value::U32(v) => v.equals(other),
            Value::U64(v) => v.equals(other),
            Value::F32(v) => v.equals(other),
            Value::F64(v) => v.equals(other),
            Value::Bool(v) => v.equals(other),
            Value::Char(v) => v.equals(other),
            Value::String(v) => v.equals(other),
            Value::Error(v) => v.equals(other),
            Value::StackTrace(v) => v.equals(other),
            Value::Type(v) => v.equals(other),
        }
    }

    fn not_equals(&self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.not_equals(other),
            Value::I8(v) => v.not_equals(other),
            Value::I16(v) => v.not_equals(other),
            Value::I32(v) => v.not_equals(other),
            Value::I64(v) => v.not_equals(other),
            Value::U8(v) => v.not_equals(other),
            Value::U16(v) => v.not_equals(other),
            Value::U32(v) => v.not_equals(other),
            Value::U64(v) => v.not_equals(other),
            Value::F32(v) => v.not_equals(other),
            Value::F64(v) => v.not_equals(other),
            Value::Bool(v) => v.not_equals(other),
            Value::Char(v) => v.not_equals(other),
            Value::String(v) => v.not_equals(other),
            Value::Error(v) => v.not_equals(other),
            Value::StackTrace(v) => v.not_equals(other),
            Value::Type(v) => v.not_equals(other),
        }
    }

    fn assign(&mut self, other: &Value) -> Option<Value> {
        match self { 
            Value::Null(v) => v.assign(other),
            Value::I8(v) => v.assign(other),
            Value::I16(v) => v.assign(other),
            Value::I32(v) => v.assign(other),
            Value::I64(v) => v.assign(other),
            Value::U8(v) => v.assign(other),
            Value::U16(v) => v.assign(other),
            Value::U32(v) => v.assign(other),
            Value::U64(v) => v.assign(other),
            Value::F32(v) => v.assign(other),
            Value::F64(v) => v.assign(other),
            Value::Bool(v) => v.assign(other),
            Value::Char(v) => v.assign(other),
            Value::String(v) => v.assign(other),
            Value::Error(v) => v.assign(other),
            Value::StackTrace(v) => v.assign(other),
            Value::Type(v) => v.assign(other),
        }
    }

    fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        match self { 
            Value::Null(v) => v.convert(to),
            Value::I8(v) => v.convert(to),
            Value::I16(v) => v.convert(to),
            Value::I32(v) => v.convert(to),
            Value::I64(v) => v.convert(to),
            Value::U8(v) => v.convert(to),
            Value::U16(v) => v.convert(to),
            Value::U32(v) => v.convert(to),
            Value::U64(v) => v.convert(to),
            Value::F32(v) => v.convert(to),
            Value::F64(v) => v.convert(to),
            Value::Bool(v) => v.convert(to),
            Value::Char(v) => v.convert(to),
            Value::String(v) => v.convert(to),
            Value::Error(v) => v.convert(to),
            Value::StackTrace(v) => v.convert(to),
            Value::Type(v) => v.convert(to),
        }
    }

    fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        match self { 
            Value::Null(v) => v.auto_convert(to),
            Value::I8(v) => v.auto_convert(to),
            Value::I16(v) => v.auto_convert(to),
            Value::I32(v) => v.auto_convert(to),
            Value::I64(v) => v.auto_convert(to),
            Value::U8(v) => v.auto_convert(to),
            Value::U16(v) => v.auto_convert(to),
            Value::U32(v) => v.auto_convert(to),
            Value::U64(v) => v.auto_convert(to),
            Value::F32(v) => v.auto_convert(to),
            Value::F64(v) => v.auto_convert(to),
            Value::Bool(v) => v.auto_convert(to),
            Value::Char(v) => v.auto_convert(to),
            Value::String(v) => v.auto_convert(to),
            Value::Error(v) => v.auto_convert(to),
            Value::StackTrace(v) => v.auto_convert(to),
            Value::Type(v) => v.auto_convert(to),
        }
    }

    fn copy(&self) -> Value {
        match self { 
            Value::Null(v) => v.copy(),
            Value::I8(v) => v.copy(),
            Value::I16(v) => v.copy(),
            Value::I32(v) => v.copy(),
            Value::I64(v) => v.copy(),
            Value::U8(v) => v.copy(),
            Value::U16(v) => v.copy(),
            Value::U32(v) => v.copy(),
            Value::U64(v) => v.copy(),
            Value::F32(v) => v.copy(),
            Value::F64(v) => v.copy(),
            Value::Bool(v) => v.copy(),
            Value::Char(v) => v.copy(),
            Value::String(v) => v.copy(),
            Value::Error(v) => v.copy(),
            Value::StackTrace(v) => v.copy(),
            Value::Type(v) => v.copy(),
        }
    }
}