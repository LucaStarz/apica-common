//! # Apica library
//!
//! - [`bytecodes`]: Defines the bytecode opcodes, types, entry points, specifications used by the Apica system.
//! - [`elements`]: Provides runtime value containers ([`Element`](elements::element::Element)) and execution state modifiers ([`ElementModifier`](elements::modifier::ElementModifier)).
//! - [`values`]: Implements primitive and composite value types processed during execution.

pub mod bytecodes;
pub mod elements;
pub mod values;

#[cfg(test)]
mod tests {
    use crate::elements::element::Element;

    /// Verifies that the memory footprint of [`Element`] remains within acceptable bounds (<= 32 bytes)
    /// to ensure optimal cache locality and stack efficiency during evaluation loops.
    #[test]
    fn test_element_size() {
        println!("size of element : {}", size_of::<Element>());
        assert!(size_of::<Element>() <= 32);
    }
}
