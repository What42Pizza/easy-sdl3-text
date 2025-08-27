//! # Easy Sdl3 Text
//! 
//! This crate is meant to add easy and useful text rendering for sdl3. It uses [ab_glyph](https://crates.io/crates/ab_glyph) and [rayon](https://crates.io/crates/rayon), has a cache for reusing textures, and implements sub-pixel rendering. I started this because I couldn't get sdl3-rs's ttf feature to compile, plus I probably wouldn't have been happy with it anyway. Right now the two biggest problem with this are 1: you have to choose a single background color, because sub-pixel rendering requires mixing each color channel separately, and 2: rendering characters is somewhat costly, sometimes taking several milliseconds on release mode if the textures aren't already cached. This crate is extremely small, so if it doesn't fit exactly what you want it for, it's probably worth it to just copy this crate's code and change it for your own use.
//! 
//! <br>
//! 
//! **Example Output:** (looks better in the demo, run `cargo run --example subpixel --release` to see the true sub-pixel rendering)
//! 
//! ![Example Image](https://github.com/What42Pizza/easy-sdl3-text/blob/main/images/example.png?raw=true)
//! 
//! <br>
//! 
//! **Example Code:**
//! 
//! ```
//! use easy_sdl3_text as sdl3_text;
//! 
//! pub fn example_draw_function<'a, Font: ThreadSafeFont>(
//! 	app_data: &AppData,
//! 	canvas: &mut Canvas<Window>,
//! 	texture_creator: &'a TextureCreator<WindowContext>,
//! 	text_cache: &mut sdl3_text::TextCache<'a, Font>
//! ) -> anyhow::Result<()> {
//! 	canvas.set_draw_color(Color::RGB(255, 255, 255));
//! 	canvas.clear();
//! 	
//! 	let size = 25;
//! 	let (x, y) = (50, 50);
//! 	let foreground = Color::RGB(30, 30, 30);
//! 	let background = Color::RGB(255, 255, 255);
//! 	sdl3_text::render_text_subpixel(
//! 		"Example text",
//! 		size,
//! 		x, y,
//! 		foreground, background,
//! 		canvas,
//! 		texture_creator,
//! 		text_cache
//! 	)?;
//! 	
//! 	canvas.present();
//! 	Ok(())
//! }
//! ```



#![warn(missing_docs)]

#![feature(array_windows)]



/// Code for non-sub-pixel rendering (a bit faster and easier to use, but looks a bit pixelated)
pub mod regular;
pub use regular::*;
/// Code for sub-pixel rendering (limited and a bit slower but looks really nice)
pub mod subpixel;
pub use subpixel::*;
/// All data types for this crate
pub mod data;
pub use data::*;
