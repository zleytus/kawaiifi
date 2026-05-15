//! C-compatible FFI bindings for `kawaiifi`.
//!
//! # Safety
//!
//! Unless a function says otherwise, pointer arguments may be null only when
//! represented as nullable parameters in the generated C header. Borrowed
//! pointers returned from this library remain valid only for the lifetime of
//! their owning object. Owned strings and byte buffers returned from this
//! library must be released with the matching `kawaiifi_*_free` function.

#![allow(clippy::missing_safety_doc)]

mod bss;
mod common;
pub mod field;
mod ies;
mod interface;
mod scan;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;
