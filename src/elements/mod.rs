//! # Elements Module
//!
//! This module provides structures and modifiers for representing evaluated values
//! and runtime control metadata within the Apica execution stack.
//!
//! - [`element::Element`]: A wrapper holding an evaluated [`Value`] alongside metadata flags.
//! - [`modifier::ElementModifier`]: A bitflag set representing properties such as mutability, errors, or control flow triggers.

pub mod element;
pub mod modifier;