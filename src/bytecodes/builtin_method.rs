use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Opcodes for built-in methods provided for different Apica values.
/// 
/// Includes methods such as `length`, `empty`, `hex_repr` and more.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum ApicaBuiltinMethodBytecode {
    /// Get the length of a value (length() -> uint).
    Length =                    0x00000000,
    
    /// Check if a value is empty (empty() -> bool).
    Empty =                     0x00000001,
    
    /// Get the binary representation of a value (bin_repr() -> string).
    BinaryRepresentation =      0x00000002,
    
    /// Get the octal representation of a value (oct_repr() -> string).
    OctalRepresentation =       0x00000003,
    
    /// Get the hexadecimal representation of a value (hex_repr() -> string).
    HexadecimalRepresentation = 0x00000004,
    
    /// Get the inner name of a value (name() -> string).
    Name =                      0x00000005,
    
    /// Get inner details of a value (details() -> string).
    Details =                   0x00000006,
    
    /// Get the stack trace of a value (stack_trace() -> string).
    StackTrace =                0x00000007,
}

impl ApicaBuiltinMethodBytecode {
    pub const fn repr(&self) -> &'static str {
        match self { 
            ApicaBuiltinMethodBytecode::Length => "length",
            ApicaBuiltinMethodBytecode::Empty => "empty",
            ApicaBuiltinMethodBytecode::BinaryRepresentation => "bin_repr",
            ApicaBuiltinMethodBytecode::OctalRepresentation => "octal_repr",
            ApicaBuiltinMethodBytecode::HexadecimalRepresentation => "hex_repr",
            ApicaBuiltinMethodBytecode::Name => "name",
            ApicaBuiltinMethodBytecode::Details => "details",
            ApicaBuiltinMethodBytecode::StackTrace => "stack_trace",
        }
    }
    
    pub fn from(name: &str) -> Option<ApicaBuiltinMethodBytecode> {
        match name { 
            "length" => Some(ApicaBuiltinMethodBytecode::Length),
            "empty" => Some(ApicaBuiltinMethodBytecode::Empty),
            "bin_repr" => Some(ApicaBuiltinMethodBytecode::BinaryRepresentation),
            "octal_repr" => Some(ApicaBuiltinMethodBytecode::OctalRepresentation),
            "hex_repr" => Some(ApicaBuiltinMethodBytecode::HexadecimalRepresentation),
            "name" => Some(ApicaBuiltinMethodBytecode::Name),
            "details" => Some(ApicaBuiltinMethodBytecode::Details),
            "stack_trace" => Some(ApicaBuiltinMethodBytecode::StackTrace),
            
            _ => None,
        }
    }
}