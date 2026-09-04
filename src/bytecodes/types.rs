use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::bytecodes::apica::ApicaBytecode;

/// Type tag opcodes supported by the Apica type system.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum ApicaTypeBytecode {
    /// Null (null).
    Null =      0x00,

    /// Dynamic type (any).
    Any =       0x01,

    /// 8-bit signed integer (i8).
    I8 =        0x02,

    /// 16-bit signed integer (i16).
    I16 =       0x03,

    /// 32-bit signed integer (i32).
    I32 =       0x04,

    /// 64-bit signed integer (i64).
    I64 =       0x05,

    /// 8-bit unsigned integer (u8).
    U8 =        0x06,

    /// 16-bit unsigned integer (u16).
    U16 =       0x07,

    /// 32-bit unsigned integer (u32).
    U32 =       0x08,

    /// 64-bit unsigned integer (u64).
    U64 =       0x09,

    /// 32-bit float (f32).
    F32 =       0x0A,

    /// 64-bit float (f64).
    F64 =       0x0B,

    /// Boolean value (bool, true/false).
    Bool =      0x0C,

    /// UTF-32 character (char).
    Char =      0x0D,

    /// UTF-8 encoded string (string).
    String =    0x0E,

    /// Error (error).
    Error =     0x0F,

    /// Type reference (type).
    Type =      0x10,
}

impl ApicaTypeBytecode {
    const fn primitive(&self) -> u8 {
        *self as u8
    }

    const fn is_signed_integer(&self) -> bool {
        matches!(self, ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64)
    }

    const fn is_unsigned_integer(&self) -> bool {
        matches!(self, ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32 | ApicaTypeBytecode::U64)
    }

    const fn is_integer(&self) -> bool {
        self.is_signed_integer() || self.is_unsigned_integer()
    }

    const fn is_float(&self) -> bool {
        matches!(self, ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64)
    }

    const fn is_number(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    const fn number_can_convert_to(to: ApicaTypeBytecode, is_auto: bool) -> bool {
        match to {
            ApicaTypeBytecode::Any | ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char => true,
            ApicaTypeBytecode::String | ApicaTypeBytecode::Type => !is_auto,

            _ => to.is_number(),
        }
    }

    const fn decimal_can_convert_to(to: ApicaTypeBytecode, is_auto: bool) -> bool {
        match to {
            ApicaTypeBytecode::Any | ApicaTypeBytecode::Bool => true,
            ApicaTypeBytecode::Char | ApicaTypeBytecode::String | ApicaTypeBytecode::Type => !is_auto,

            _ => to.is_number(),
        }
    }
    
    const fn number_comparison_resolve_to(other: &ApicaTypeBytecode, is_equality: bool) -> Option<ApicaTypeBytecode> {
        match other {
            ApicaTypeBytecode::Null => if is_equality { Some(ApicaTypeBytecode::Bool) } else { None },

            _ if other.is_number() || matches!(other, ApicaTypeBytecode::Any | ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char)
                => Some(ApicaTypeBytecode::Bool),

            _ => None,
        }
    }

    const fn resolve_type_increment_decrement(&self) -> Option<ApicaTypeBytecode> {
        match self {
            ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

            ApicaTypeBytecode::Null | ApicaTypeBytecode::Bool | ApicaTypeBytecode::String | ApicaTypeBytecode::Error | ApicaTypeBytecode::Type
                => None,

            _ => Some(*self),
        }
    }

    const fn resolve_type_unary_not(&self) -> Option<ApicaTypeBytecode> {
        match self {
            ApicaTypeBytecode::Type => None,

            _ => Some(ApicaTypeBytecode::Bool),
        }
    }

    const fn resolve_type_bitwise_not(&self) -> Option<ApicaTypeBytecode> {
        match self {
            ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

            ApicaTypeBytecode::Null | ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64
            | ApicaTypeBytecode::String | ApicaTypeBytecode::Error | ApicaTypeBytecode::Type => None,

            _ => Some(*self),
        }
    }

    const fn resolve_type_compare(&self, other: &ApicaTypeBytecode) -> Option<ApicaTypeBytecode> {
        match self {
            ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Bool),

            _ if self.is_number() || matches!(self, ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char)
                => ApicaTypeBytecode::number_comparison_resolve_to(other, false),

            _ => None,
        }
    }

    const fn resolve_type_equality(&self, other: &ApicaTypeBytecode) -> Option<ApicaTypeBytecode> {
        match self {
            ApicaTypeBytecode::Any | ApicaTypeBytecode::Null => Some(ApicaTypeBytecode::Bool),

            _ if self.is_number() || matches!(self, ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char)
                => ApicaTypeBytecode::number_comparison_resolve_to(other, true),

            _ => if self.primitive() == other.primitive() || matches!(other, ApicaTypeBytecode::Null) {
                Some(ApicaTypeBytecode::Bool)
            } else {
                None
            }
        }
    }

    const fn resolve_type_basic_binary_operations(&self, other: &ApicaTypeBytecode, is_addition: bool) -> Option<ApicaTypeBytecode> {
        match self {
            ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),
            ApicaTypeBytecode::Null => None,

            ApicaTypeBytecode::String => if is_addition && !matches!(other, ApicaTypeBytecode::Null) {
                Some(Self::String)
            } else {
                None
            },
            
            ApicaTypeBytecode::I8 => match other { 
                ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),
                
                ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64
                | ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 => Some(*other),
                
                ApicaTypeBytecode::U8 | ApicaTypeBytecode::Bool => Some(ApicaTypeBytecode::I8),
                ApicaTypeBytecode::U16 => Some(ApicaTypeBytecode::I16),
                ApicaTypeBytecode::U32 | ApicaTypeBytecode::Char => Some(ApicaTypeBytecode::I32),
                ApicaTypeBytecode::U64 => Some(ApicaTypeBytecode::I64),
                
                _ => None,
            },
            
            ApicaTypeBytecode::I16 => match other {
                ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

                ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64
                | ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 => Some(*other),

                ApicaTypeBytecode::I8 | ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::Bool 
                    => Some(ApicaTypeBytecode::I16),
                
                ApicaTypeBytecode::U32 | ApicaTypeBytecode::Char => Some(ApicaTypeBytecode::I32),
                ApicaTypeBytecode::U64 => Some(ApicaTypeBytecode::I64),

                _ => None,
            },
            
            ApicaTypeBytecode::I32 => match other {
                ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

                ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64
                | ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 => Some(*other),

                ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 
                | ApicaTypeBytecode::U32 | ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char
                => Some(ApicaTypeBytecode::I32),

                ApicaTypeBytecode::U64 => Some(ApicaTypeBytecode::I64),

                _ => None,
            },
            
            ApicaTypeBytecode::I64 => match other {
                ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),
                    
                ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 => Some(ApicaTypeBytecode::F64),

                ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64 
                | ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32 | ApicaTypeBytecode::U64
                | ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char
                => Some(ApicaTypeBytecode::I64),
                
                _ => None,
            },
            
            ApicaTypeBytecode::U8 | ApicaTypeBytecode::Bool => match other {
                ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

                ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64
                | ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32 | ApicaTypeBytecode::U64
                | ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 => Some(*other),
                
                ApicaTypeBytecode::Bool => Some(ApicaTypeBytecode::U8),
                ApicaTypeBytecode::Char => Some(ApicaTypeBytecode::U32),

                _ => None,
            },
            
            ApicaTypeBytecode::U16 => match other { 
                ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

                ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64
                | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32 | ApicaTypeBytecode::U64
                | ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 => Some(*other),

                ApicaTypeBytecode::I8 => Some(ApicaTypeBytecode::I16),
                ApicaTypeBytecode::U8 | ApicaTypeBytecode::Bool => Some(ApicaTypeBytecode::U16),
                ApicaTypeBytecode::Char => Some(ApicaTypeBytecode::U32),

                _ => None,
            },
            
            ApicaTypeBytecode::U32 | ApicaTypeBytecode::Char => match other { 
                ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

                ApicaTypeBytecode::I64 | ApicaTypeBytecode::U64 | ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 => Some(*other),
                
                ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 => Some(ApicaTypeBytecode::I32),
                
                ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32
                | ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char => Some(ApicaTypeBytecode::U32),
                
                _ => None,
            },
            
            ApicaTypeBytecode::U64 => match other {
                ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

                ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 => Some(ApicaTypeBytecode::F64),

                ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64 
                    => Some(ApicaTypeBytecode::I64),

                ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32 | ApicaTypeBytecode::U64
                | ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char => Some(ApicaTypeBytecode::U64),

                _ => None,
            },
            
            ApicaTypeBytecode::F32 => match other { 
                ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

                ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32
                | ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32
                | ApicaTypeBytecode::F32 | ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char
                    => Some(ApicaTypeBytecode::F32),
                
                ApicaTypeBytecode::I64 | ApicaTypeBytecode::U64 | ApicaTypeBytecode::F64 => Some(ApicaTypeBytecode::F64),

                _ => None,
            },
            
            ApicaTypeBytecode::F64 => match other {
                ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

                _ if other.is_number() || matches!(other, ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char)
                    => Some(ApicaTypeBytecode::F64),
                
                _ => None,
            },
            
            _ => None,
        }
    }
    
    const fn resolve_type_shift(&self, other: &ApicaTypeBytecode) -> Option<ApicaTypeBytecode> {
        match self {
            ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),

            _ if self.is_number() && other.is_number() => Some(*self),

            _ => None,
        }
    }
    
    const fn resolve_type_assign(&self, other: &ApicaTypeBytecode) -> Option<ApicaTypeBytecode> {
        match self {
            ApicaTypeBytecode::Any => Some(ApicaTypeBytecode::Any),
            ApicaTypeBytecode::Null => None,

            _ if self.is_number() && other.is_number() => Some(*self),

            _ => if self.primitive() == other.primitive() {
                Some(*self)
            } else {
                None
            },
        }
    }
    
    const fn resolve_type_convert(&self, other: ApicaTypeBytecode) -> Option<ApicaTypeBytecode> {
        if self.can_be_converted_to(other, false) {
            Some(other)
        } else {
            None
        }
    }

    /// Obtain the representation of a [`ApicaTypeBytecode`].
    pub const fn repr(&self) -> &'static str {
        match self {
            ApicaTypeBytecode::Null => "null",
            ApicaTypeBytecode::Any => "???",

            ApicaTypeBytecode::I8 => "i8",
            ApicaTypeBytecode::I16 => "i16",
            ApicaTypeBytecode::I32 => "i32",
            ApicaTypeBytecode::I64 => "i64",
            ApicaTypeBytecode::U8 => "u8",
            ApicaTypeBytecode::U16 => "u16",
            ApicaTypeBytecode::U32 => "u32",
            ApicaTypeBytecode::U64 => "u64",
            ApicaTypeBytecode::F32 => "f32",
            ApicaTypeBytecode::F64 => "f64",
            ApicaTypeBytecode::Bool => "bool",

            ApicaTypeBytecode::Char => "char",
            ApicaTypeBytecode::String => "string",

            ApicaTypeBytecode::Error => "error",
            ApicaTypeBytecode::Type => "type",
        }
    }

    /// Check if a [`ApicaTypeBytecode`] can be converted to another [`ApicaTypeBytecode`], automatically or not.
    ///
    /// # Returns
    ///
    /// [`true`] if it is convertable, [`false`] otherwise.
    pub const fn can_be_converted_to(&self, to: ApicaTypeBytecode, is_auto: bool) -> bool {
        match self {
            ApicaTypeBytecode::Null | ApicaTypeBytecode::Any => true,

            ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64 |
            ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32 | ApicaTypeBytecode::U64
                => ApicaTypeBytecode::number_can_convert_to(to, is_auto),

            ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 | ApicaTypeBytecode::Bool
                => ApicaTypeBytecode::decimal_can_convert_to(to, is_auto),

            ApicaTypeBytecode::Char => match to {
                ApicaTypeBytecode::Any |
                ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64 |
                ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32 | ApicaTypeBytecode::U64 |
                ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 |
                ApicaTypeBytecode::Char => true,

                ApicaTypeBytecode::Bool | ApicaTypeBytecode::String | ApicaTypeBytecode::Type => !is_auto,

                _ => false,
            },

            ApicaTypeBytecode::String => match to {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::String => true,
                ApicaTypeBytecode::Bool | ApicaTypeBytecode::Type => !is_auto,
                _ => false,
            },

            ApicaTypeBytecode::Type => match to {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Type => true,
                ApicaTypeBytecode::Bool | ApicaTypeBytecode::String => !is_auto,
                _ => false,
            },

            ApicaTypeBytecode::Error => match to {
                ApicaTypeBytecode::Any | ApicaTypeBytecode::Error => true,
                ApicaTypeBytecode::Bool | ApicaTypeBytecode::String | ApicaTypeBytecode::Type => !is_auto,
                _ => false,
            },
        }
    }

    /// Get the [`ApicaTypeBytecode`] resulting of an operator applied to values of one or two specific(s) [`ApicaTypeBytecode`].
    /// To handle assignment operators (i.e. `+=`, `|=`, ...), it should be used twice (`+` then `=`, `|` then `=`, ...).
    ///
    /// # Returns
    ///
    /// [`Some(ApicaTypeBytecode)`] if the operator is applicable, [`None`] otherwise.
    pub const fn resolve_type_operators(&self, other: ApicaTypeBytecode, operator: ApicaBytecode) -> Option<ApicaTypeBytecode> {
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

            ApicaBytecode::SpecialOp => Some(ApicaTypeBytecode::Any),
            _ => None,
        }
    }
}