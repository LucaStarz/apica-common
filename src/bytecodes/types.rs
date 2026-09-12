use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Type tag opcodes supported by the Apica type system.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum ApicaTypeBytecode {
    /// Null (null).
    Null =          0x00,

    /// Dynamic type (any).
    Any =           0x01,
    
    /// 64-bit signed integer value (int).
    Int =           0x02,
    
    /// 64-bit unsigned integer value (uint).
    UnsignedInt =   0x03,
    
    /// 64-bit signed float value (float).
    Float =         0x04,

    /// Boolean value (bool, true/false).
    Bool =      0x05,

    /// UTF-32 character (char).
    Char =      0x06,

    /// UTF-8 encoded string (string).
    String =    0x07,

    /// Error (error).
    Error =     0x08,

    /// Type reference (type).
    Type =      0x09,

    /// Reference to other value (ref).
    Reference = 0x0A,
}

impl ApicaTypeBytecode {
    /// Obtain the representation of a [`ApicaTypeBytecode`].
    pub const fn repr(&self) -> &'static str {
        match self {
            ApicaTypeBytecode::Null => "null",
            ApicaTypeBytecode::Any => "any",

            ApicaTypeBytecode::Int => "int",
            ApicaTypeBytecode::UnsignedInt => "uint",
            ApicaTypeBytecode::Float => "float",
            ApicaTypeBytecode::Bool => "bool",

            ApicaTypeBytecode::Char => "char",
            ApicaTypeBytecode::String => "string",

            ApicaTypeBytecode::Error => "error",
            ApicaTypeBytecode::Type => "type",
            ApicaTypeBytecode::Reference => "reference",
        }
    }
}