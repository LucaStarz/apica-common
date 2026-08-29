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