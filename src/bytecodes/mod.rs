//! # Apica Bytecode Definition
//!
//! This module provides the core bytecode definitions for the **Apica** system.
//!
//! All opcodes are encoded as 64-bit unsigned integers (`u64`) and organized into distinct enums:
//! - [`ApicaBytecode`]: General instructions, arithmetic/logical operators, control flow, ...
//! - [`ApicaBuiltinFunctionBytecode`]: Built-in system functions.
//! - [`ApicaEntrypointBytecode`]: Application lifecycle entry points.
//! - [`ApicaSpecificationBytecode`]: Application metadata and configuration directives.
//! - [`ApicaTypeBytecode`]: Primitive and composite data type identifiers.

pub mod apica;
pub mod entrypoint;
pub mod builtin_function;
pub mod specification;
pub mod types;