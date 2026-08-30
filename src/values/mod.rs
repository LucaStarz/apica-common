//! # Values Module
//! 
//! This module provides structures representing the Apica values
//! 
//! - [`value::ValueTrait`]: The operations handled by all Apica values.
//! - [`value::Value`]: An enumeration wrapping all Apica values.
//! - [`null::ValueNull`]: An Apica value representing an always null value.
//! - [`i8::ValueI8`]: An Apica value representing a nullable signed 8-bit integer.
//! - [`i16::ValueI16`]: An Apica value representing a nullable signed 16-bit integer.
//! - [`i32::ValueI32`]: An Apica value representing a nullable signed 32-bit integer.
//! - [`i64::ValueI64`]: An Apica value representing a nullable signed 64-bit integer.
//! - [`u8::ValueU8`]: An Apica value representing a nullable unsigned 8-bit integer.
//! - [`u16::ValueU16`]: An Apica value representing a nullable unsigned 16-bit integer.
//! - [`u32::ValueU32`]: An Apica value representing a nullable unsigned 32-bit integer.
//! - [`u64::ValueU64`]: An Apica value representing a nullable unsigned 64-bit integer.
//! - [`f32::ValueF32`]: An Apica value representing a nullable 32-bit floating number.
//! - [`f64::ValueF64`]: An Apica value representing a nullable 64-bit floating number.
//! - [`bool::ValueBool`]: An Apica value representing a nullable boolean.
//! - [`char::ValueChar`]: An Apica value representing a nullable 32-bit character.
//! - [`string::ValueString`]: An Apica value representing a nullable UTF-8 encoded array of bytes.
//! - [`error::ValueError`]: An Apica value representing a nullable error.
//! - [`stack_trace::ValueStackTrace`]: An Apica value (system-only) representing a not-nullable error stack trace.
//! - [`value_type::ValueType`]: An Apica value representing a not-nullable type.

pub mod value;
pub mod null;
pub mod i8;
pub mod i16;
pub mod i32;
pub mod i64;
pub mod u8;
pub mod u16;
pub mod u32;
pub mod u64;
pub mod f32;
pub mod f64;
pub mod bool;
pub mod char;
pub mod string;
pub mod error;
pub mod stack_trace;
pub mod value_type;