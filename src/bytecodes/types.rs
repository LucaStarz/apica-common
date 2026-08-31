use num_enum::{IntoPrimitive, TryFromPrimitive};

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
    /// Obtain the representation of a [`ApicaTypeBytecode`].
    pub fn repr(&self) -> &'static str {
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

    fn number_can_convert_to(to: ApicaTypeBytecode, is_auto: bool) -> bool {
        match to {
            ApicaTypeBytecode::Any |
            ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64 |
            ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32 | ApicaTypeBytecode::U64 |
            ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 |
            ApicaTypeBytecode::Bool | ApicaTypeBytecode::Char => true,

            ApicaTypeBytecode::String | ApicaTypeBytecode::Type => !is_auto,

            _ => false,
        }
    }

    fn decimal_can_convert_to(to: ApicaTypeBytecode, is_auto: bool) -> bool {
        match to {
            ApicaTypeBytecode::Any |
            ApicaTypeBytecode::I8 | ApicaTypeBytecode::I16 | ApicaTypeBytecode::I32 | ApicaTypeBytecode::I64 |
            ApicaTypeBytecode::U8 | ApicaTypeBytecode::U16 | ApicaTypeBytecode::U32 | ApicaTypeBytecode::U64 |
            ApicaTypeBytecode::F32 | ApicaTypeBytecode::F64 |
            ApicaTypeBytecode::Bool => true,

            ApicaTypeBytecode::Char | ApicaTypeBytecode::String | ApicaTypeBytecode::Type => !is_auto,

            _ => false,
        }
    }

    /// Check if a [`ApicaTypeBytecode`] can be converted to another [`ApicaTypeBytecode`], automatically or not.
    ///
    /// # Returns
    ///
    /// [`true`] if it is convertable, [`false`] otherwise.
    pub fn can_be_converted_to(&self, to: ApicaTypeBytecode, is_auto: bool) -> bool {
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
}