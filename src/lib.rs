//! # Easy Sdl3 Text
//! 
//! To get started, just look at one of the [examples](https://github.com/What42Pizza/easy-sdl3-text/tree/main/examples)
//! 
//! The three main functions from this crate you need to know are:
//! 
//! - `TextCache::new()` - for creating the cache (this single cache can and should be used for ALL text rendering)
//! - `render_text_regular()` - for rendering text with a specific position, size, and foreground color to a canvas
//! - `render_text_subpixel()` - for rendering text with a specific position, size, foreground color, and background color to a canvas
//! 
//! ### Known Limitations:
//! 
//! - The subpixel rendering cannot blend into a background, and must have the background color supplied so that it can pre-blend the text onto a single color. This is because of subpixel rendering's per-channel mixing, which cannot be done after initial rasterization without custom shaders, which sdl3's renderer api doesn't support.
//! - The subpixel rendering takes an integer size, which makes resizing look slightly strange. This is an intentional choice to cut down significantly on the number of textures to rasterize and cache.
//! 
//! <br>
//! 
//! ### Example Code:
//! 
//! ```
//! use easy_sdl3_text as sdl3_text;
//! 
//! pub fn example_draw_function<'a, Font: ThreadSafeFont>(
//! 	app_data: &AppData,
//! 	canvas: &mut Canvas<Window>,
//! 	texture_creator: &'a TextureCreator<WindowContext>,
//! 	text_cache: &mut sdl3_text::TextCache<'a, Font>,
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
//! 		text_cache,
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
