//! # Easy Sdl3 Text
//! 
//! This crate is meant to add easy and useful text rendering for sdl3. It uses [ab_glyph](https://crates.io/crates/ab_glyph) and [rayon](https://crates.io/crates/rayon), has a cache for reusing textures, and implements sub-pixel rendering. I started this because I couldn't get sdl3-rs's ttf feature to compile, plus I probably wouldn't have been happy with it anyway. Right now the two biggest problem with this are 1: you have to choose a single background color, because sub-pixel rendering requires mixing each color channel separately, and 2: rendering characters is somewhat costly, sometimes taking several milliseconds on release mode if the textures aren't already cached. This crate is extremely small, so if it doesn't fit exactly what you want it for, it's probably worth it to just copy this crate's code and change it for your own use.
//! 
//! ### Example: (looks better in the demo, run `cargo run --example basic` to see the true sub-pixel rendering)
//! 
//! ![Example Image](https://github.com/What42Pizza/easy-sdl3-text/blob/main/images/example.png?raw=true)



#![feature(array_windows)]



pub mod data;
pub use data::*;
pub mod subpixel;
pub use subpixel::*;
