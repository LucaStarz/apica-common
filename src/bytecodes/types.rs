use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Type tag opcodes supported by the Apica type system.
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum ApicaTypeBytecode {
    /// Null (null).
    Null =      0x00000000,

    /// Dynamic type (any).
    Any =       0x00000001,

    /// 8-bit signed integer (i8).
    I8 =        0x00000002,

    /// 16-bit signed integer (i16).
    I16 =       0x00000003,

    /// 32-bit signed integer (i32).
    I32 =       0x00000004,

    /// 64-bit signed integer (i64).
    I64 =       0x00000005,

    /// 8-bit unsigned integer (u8).
    U8 =        0x00000006,

    /// 16-bit unsigned integer (u16).
    U16 =       0x00000007,

    /// 32-bit unsigned integer (u32).
    U32 =       0x00000008,

    /// 64-bit unsigned integer (u64).
    U64 =       0x00000009,

    /// 32-bit float (f32).
    F32 =       0x0000000A,

    /// 64-bit float (f64).
    F64 =       0x0000000B,

    /// Boolean value (bool, true/false).
    Bool =      0x0000000C,

    /// UTF-32 character (char).
    Char =      0x0000000D,

    /// UTF-8 encoded string (string).
    String =    0x0000000E,

    /// Error (error).
    Error =     0x0000000F,

    /// Type reference (type).
    Type =      0x00000010,
}