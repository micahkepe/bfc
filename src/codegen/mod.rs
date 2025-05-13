//! # Code generation
//!
//! Defines targets to generate code for using the parsed AST.

pub mod asm;
pub mod targets;

// reexport `CodegenTarget` trait
pub use targets::CodegenTarget;
