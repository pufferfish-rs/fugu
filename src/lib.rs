//! A simple cross-platform rendering library for Rust.
//!
//! [![Docs](https://docs.rs/fugu/badge.svg)](https://docs.rs/fugu)
//! [![Crates.io](https://shields.io/crates/v/fugu)](https://crates.io/crates/fugu)
//! ![License](https://shields.io/crates/l/fugu)
//!
//! ## Goals
//!
//! - Simple, modern, and safe API
//! - Fast compile times with clean builds
//! - Portability; only depends on `core`, `alloc`, and the rendering backend
//! - Transparent implementation; internal abstractions are minimal and easy to
//!   grok or hack
//!
//! ## Non-Goals
//!
//! - Windowing or context creation (use [`glutin`](https://crates.io/crates/glutin),
//!   [`sdl2`](https://crates.io/crates/sdl2), etc.)
//! - "Advanced" functionality (use [`wgpu`](https://github.com/gfx-rs/wgpu) or
//!   Vulkan/Metal/DirectX/... directly)
//! - Shader translation (use [`naga`](https://crates.io/crates/naga) or
//!   SPIRV-cross)
//! - GPU-side safety guarantees; the API is safe *Rust* but can still produce
//!   crashes or UB

#![warn(missing_docs)]
#![no_std]

extern crate alloc;

mod blend;
mod buffer;
mod context;
mod image;
mod pass;
mod pipeline;
mod shader;

pub use crate::blend::*;
pub use crate::buffer::*;
pub use crate::context::*;
pub use crate::image::*;
pub use crate::pass::*;
pub use crate::pipeline::*;
pub use crate::shader::*;
