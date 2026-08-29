use crate::bytecodes::types::ApicaTypeBytecode;

pub fn number_can_convert_to(to: ApicaTypeBytecode, is_auto: bool) -> bool {
    match to {
        ApicaTypeBytecode::Any
        | ApicaTypeBytecode::I8
        | ApicaTypeBytecode::I16
        | ApicaTypeBytecode::I32
        | ApicaTypeBytecode::I64
        | ApicaTypeBytecode::U8
        | ApicaTypeBytecode::U16
        | ApicaTypeBytecode::U32
        | ApicaTypeBytecode::U64
        | ApicaTypeBytecode::F32
        | ApicaTypeBytecode::F64
        | ApicaTypeBytecode::Bool => true,

        ApicaTypeBytecode::Char
        | ApicaTypeBytecode::String
        | ApicaTypeBytecode::Type => !is_auto,

        _ => false,
    }
}