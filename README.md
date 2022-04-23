<div align="center">

# fugu

A simple cross-platform rendering library for Rust

[![Docs](https://docs.rs/fugu/badge.svg)](https://docs.rs/fugu)
[![Crates.io](https://shields.io/crates/v/fugu)](https://crates.io/crates/fugu)
![License](https://shields.io/crates/l/fugu)

</div>

> :warning: **DISCLAIMER**: fugu is in *very* early stages of development. APIs can and will change, many basic features are missing, and parts of this README may not reflect the current state of the crate. I would recommend against using this in your projects just yet, but for those brave enough, now is the time for suggestions and feature requests!

## Goals

- Simple, modern, and safe API
- Fast compile times with clean builds
- Portability; only depends on `core`, `alloc`, and the rendering backend
- Transparent implementation; internal abstractions are minimal and easy to grok or hack

## Non-Goals

- Windowing or context creation (use [`glutin`](https://crates.io/crates/glutin), [`sdl2`](https://crates.io/crates/sdl2), etc.)
- "Advanced" functionality (use [`wgpu`](https://github.com/gfx-rs/wgpu) or Vulkan/Metal/DirectX/... directly)
- Shader translation (use [`naga`](https://crates.io/crates/naga) or SPIRV-cross)
- GPU-side safety guarantees; the API is safe *Rust* but can still produce crashes or UB

## Platform Support

### Implemented

- Windows (OpenGL via [`glow`](https://crates.io/crates/glow))
- OS X (OpenGL via [`glow`](https://crates.io/crates/glow))
- _Linux (OpenGL via [`glow`](https://crates.io/crates/glow)*)_
- _Android (OpenGL ES via [`glow`](https://crates.io/crates/glow)*)_
- _iOS (OpenGL ES via [`glow`](https://crates.io/crates/glow)*)_

<sub>

\* Untested. These should work in theory, but don't be surprised if they don't. In that case, please open an issue or PR.

</sub>

### Desired

- Web (WebGL); this should be trivial with [`glow`](https://crates.io/crates/glow) but a dependency on [`web-sys`](https://crates.io/crates/web-sys) may not be ideal
- OS X/iOS (Metal); Apple has depricated OpenGL, and although OpenGL probably won't go anywhere any time soon, Metal should be used for futureproofing

## Acknowledgements

- [sokol_gfx](https://github.com/floooh/sokol), a wonderful single-file graphics library for C and a major source of API inspiration ([A Tour of sokol_gfx.h](https://floooh.github.io/2017/07/29/sokol-gfx-tour.html))
- [miniquad](https://github.com/not-fl3/miniquad), a great Rust library also inspired by sokol_gfx
