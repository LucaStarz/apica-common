//! # Values Module
//! 
//! This module provides structures representing the Apica values
//! 
//! - [`value::ValueTrait`]: The operations handled by all Apica values.
//! - [`value::Value`]: An enumeration wrapping all Apica values.
//! - [`null::ValueNull`]: An Apica value representing an always null value.
//! - [`int::ValueInt`]: An Apica value representing a nullable signed 64-bit integer.
//! - [`uint::ValueUInt`]: An Apica value representing a nullable unsigned 64-bit integer.
//! - [`float::ValueFloat`]: An Apica value representing a nullable 64-bit floating number.
//! - [`bool::ValueBool`]: An Apica value representing a nullable boolean.
//! - [`char::ValueChar`]: An Apica value representing a nullable 32-bit character.
//! - [`string::ValueString`]: An Apica value representing a nullable UTF-8 encoded array of bytes.
//! - [`error::ValueError`]: An Apica value representing a nullable error with stack trace.
//! - [`value_type::ValueType`]: An Apica value representing a not-nullable type.
//! - [`reference::ValueReference`]: An Apica value representing a reference to a variable.

pub mod value;
pub mod null;
pub mod int;
pub mod uint;
pub mod float;
pub mod bool;
pub mod char;
pub mod string;
pub mod error;
pub mod value_type;
pub mod reference;