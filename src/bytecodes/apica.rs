use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Opcodes representing low-level instructions for the Apica system.
///
/// Covers execution flow control, declarations, arithmetic, logical operations, ...
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum ApicaBytecode {
    /// End of bytecode file
    EndOfFile =       0x00000000,

    /// End-of-block (compound, list, ...).
    EndOfBlock =        0x00000001,

    /// Entrypoint definition.
    Entrypoint =        0x00000002,

    /// Built-in Apica function call.
    BuiltinFuncCall =   0x00000003,

    /// Raw literal value.
    Literal =           0x00000004,

    /// Compound structure ({ ... }).
    Compound =          0x00000005,

    /// Variable or constant call.
    VarConstCall =      0x00000006,

    /// Variable definition (var).
    VarDecl =           0x00000007,

    /// Constant definition (const).
    ConstDecl =         0x00000008,

    /// Addition (elt + elt).
    Add =               0x00000009,

    /// Subtraction (elt - elt).
    Subtract =          0x0000000A,

    /// Assignment (elt = elt).
    Assign =            0x0000000B,

    /// Postfix increment (elt++).
    Increment =         0x0000000C,

    /// Postfix decrement (elt--).
    Decrement =         0x0000000D,

    /// Less-than operation (elt < elt).
    LessThan =          0x0000000E,

    /// Equality (elt == elt).
    Equals =            0x0000000F,

    /// Logical NOT (!elt).
    Not =               0x00000010,

    /// Type casting (elt as type).
    As =                0x00000011,

    /// Prefix increment (++elt).
    LeftIncrement =     0x00000012,

    /// Prefix decrement (--elt).
    LeftDecrement =     0x00000013,

    /// Addition assignment (elt += elt).
    AddAssign =         0x00000014,

    /// Subtraction assignment (elt -= elt).
    SubtractAssign =    0x00000015,

    /// Multiplication (elt * elt).
    Multiply =          0x00000016,

    /// Multiplication assignment (elt *= elt).
    MultiplyAssign =    0x00000017,

    /// Division (elt / elt).
    Divide =            0x00000018,

    /// Division assignment (elt /= elt).
    DivideAssign =      0x00000019,

    /// Inequality (elt != elt).
    NotEquals =         0x0000001A,

    /// Custom operation (elt @ elt).
    SpecialOp =         0x0000001B,

    /// Custom operation assignment (elt @= elt).
    SpecialOpAssign =   0x0000001C,

    /// Bitwise AND (elt & elt).
    BitwiseAnd =        0x0000001D,

    /// Logical AND (elt && elt).
    LogicalAnd =        0x0000001E,

    /// Bitwise AND assignment (elt &= elt).
    BitwiseAndAssign =  0x0000001F,

    /// Bitwise OR (elt | elt).
    BitwiseOr =         0x00000020,

    /// Logical OR (elt || elt).
    LogicalOr =         0x00000021,

    /// Bitwise OR assignment (elt |= elt).
    BitwiseOrAssign =   0x00000022,

    /// Bitwise NOT (~elt).
    BitwiseNot =        0x00000023,

    /// Bitwise XOR (elt ^ elt).
    BitwiseXor =        0x00000024,

    /// Bitwise XOR assignment (elt ^= elt).
    BitwiseXorAssign =  0x00000025,

    /// Less-than or equal operation (elt <= elt).
    LessOrEquals =      0x00000026,

    /// Greater-than operation (elt > elt).
    GreaterThan =       0x00000027,

    /// Greater-than or equal operation (elt >= elt).
    GreaterOrEquals =   0x00000028,

    /// Modulo (elt % elt).
    Modulo =            0x00000029,

    /// Modulo assignment (elt %= elt).
    ModuloEquals =      0x0000002A,

    /// Member access (elt.member).
    Access =            0x0000002B,

    /// Conditional member access (elt?.member).
    ConditionalAccess = 0x0000002C,

    /// Break loop statement.
    Break =             0x0000002D,

    /// Continue loop iteration statement.
    Continue =          0x0000002E,

    /// Return statement without payload (return).
    BlankReturn =       0x0000002F,

    /// Return statement with payload (return expr).
    FilledReturn =      0x00000030,

    /// Ternary operation (?elt : valid : invalid).
    QuestionOperation = 0x00000031,

    /// IF statement.
    If =                0x00000032,

    /// IF-ELSE statement.
    IfElse =            0x00000033,

    /// WHILE statement.
    While =             0x00000034,

    /// FOR statement.
    For =               0x00000035,

    /// No-operation (empty compound, ...)
    NoOperation =       0x00000036,

    /// Left shift (elt << elt)
    LeftShift =         0x00000037,

    /// Left shift assignment (elt <<= elt)
    LeftShiftAssign =   0x00000038,

    /// Right shift (elt >> elt)
    RightShift =        0x00000039,
    
    /// Right shift assignment (elt >>= elt)
    RightShiftAssign =  0x0000003A,
}