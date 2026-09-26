//! # Apica Bytecode Definition
//!
//! This module provides the core bytecode definitions for the **Apica** system.
//!
//! All opcodes are encoded as 64-bit unsigned integers (`u64`) and organized into distinct enums:
//! - [`apica::ApicaBytecode`]: General instructions, arithmetic/logical operators, control flow, ...
//! - [`builtin_function::ApicaBuiltinFunctionBytecode`]: Built-in system functions.
//! - [`entrypoint::ApicaEntrypointBytecode`]: Application lifecycle entry points.
//! - [`specification::ApicaSpecificationBytecode`]: Application metadata and configuration directives.
//! - [`types::ApicaTypeBytecode`]: Primitive and composite data type identifiers.
//! - [`builtin_method::ApicaBuiltinMethodBytecode`]: Built-in methods defined for Apica values.

pub mod apica;
pub mod entrypoint;
pub mod builtin_function;
pub mod specification;
pub mod types;
pub mod builtin_method;