use bitflags::bitflags;

bitflags! {
    /// Bitflags representing modifiers and control attributes attached to an [`Element`](crate::elements::element::Element).
    ///
    /// These flags determine variable mutability, runtime states,
    /// and control flow signals (such as `break`, `continue`, `return`, or program termination).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ElementModifier: u8 {
        /// Default flag indicating no special modifiers or state.
        const NONE =            0b0000_0000;

        /// Immutable variable (constant) value flag.
        const CONST =           0b0000_0001;

        /// Dynamic type handling flag.
        const ANY =             0b0000_0010;

        /// Error state indicator.
        const ERROR =           0b0000_0100;

        /// Control flow signal to interrupt current loop execution.
        const BREAK =           0b0000_1000;

        /// Control flow signal to skip the next loop iteration.
        const CONTINUE =        0b0001_0000;

        /// Control flow signal to return from the current function.
        const RETURN =          0b0010_0000;

        /// Control flow signal to immediately stop execution.
        const TERMINATE =       0b0100_0000;
        
        /// Not nullable variable value flag.
        const NOT_NULLABLE =    0b1000_0000;
    }
}