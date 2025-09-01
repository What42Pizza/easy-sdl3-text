//! # Easy Sdl3 Text
//! 
//! To get started, just look at one of the [examples](https://github.com/What42Pizza/easy-sdl3-text/tree/main/examples) (or at the example code below)
//! 
//! <br>
//! 
//! **Quick api overview:**
//! 
//! - `TextCache` - holds all the long-lived data that this crate needs for rendering text, excluding the canvas and texture creator
//! - `TextRenderingSettings` - holds most of the arguments for the text rendering functions
//! - `render_text_regular()` - renders text without subpixel rendering
//! - `render_text_subpixel()` - renders text with subpixel rendering
//! 
//! ### Example Code:
//! 
//! ```rust
//! use easy_sdl3_text::*;
//! 
//! pub fn example_draw_function<'a, F: ThreadSafeFont>(
//! 	app_data: &AppData,
//! 	canvas: &mut Canvas<Window>,
//! 	texture_creator: &'a TextureCreator<WindowContext>,
//! 	text_cache: &mut TextCache<'a, F>,
//! ) -> anyhow::Result<()> {
//! 	canvas.set_draw_color(Color::RGB(255, 255, 255));
//! 	canvas.clear();
//! 	
//! 	let size: u32 = 25;
//! 	let (mut x, mut y): (i32, i32) = (50, 50);
//! 	let foreground = Color::RGB(30, 30, 30);
//! 	let background = Color::RGB(255, 255, 255);
//! 	
//! 	// most arguments to the rendering functions are the same across calls, so they're all put into a reusable struct
//! 	let mut text_rendering_settings = TextRenderingSettings::new_subpixel(
//! 		size,
//! 		HAlign::Left, VAlign::Center,
//! 		foreground, background,
//! 		canvas, texture_creator, text_cache
//! 	);
//! 	
//! 	render_text_subpixel("Example text", x, y, &mut text_rendering_settings)?;
//! 	y += size as i32;
//! 	render_text_subpixel("More example text", x, y, &mut text_rendering_settings)?;
//! 	y += size as i32;
//! 	
//! 	canvas.present();
//! 	Ok(())
//! }
//! ```
//! 
//! <br>
//! 
//! ### Known Limitations:
//! 
//! - The subpixel rendering cannot blend into a background, and must have the background color supplied so that it can pre-blend the text onto a single color. This is because of subpixel rendering's per-channel mixing, which cannot be done after initial rasterization without custom shaders, which sdl3's renderer api doesn't support.
//! - The subpixel rendering takes an integer size, which makes resizing look slightly strange. This is an intentional choice to cut down significantly on the number of textures to rasterize and cache.



#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

#![feature(array_windows)]



/// Functions for non-sub-pixel rendering (a bit faster and easier to use, but looks a bit pixelated)
pub mod regular;
pub use regular::*;
/// Functions for sub-pixel rendering (limited and a bit slower but looks really nice)
pub mod subpixel;
pub use subpixel::*;
/// All data types for this crate
pub mod data;
pub use data::*;



pub(crate) const EXTRA_CHAR_SPACING: f32 = 0.015;
pub(crate) const EXTRA_WHITESPACE_SPACING: f32 = 0.045; // Note: this is added on top of EXTRA_CHAR_SPACING
pub(crate) const REGULAR_VALUE_POW: f32 = 0.7; // affects how dark the edges are
pub(crate) const SUBPIXEL_VALUE_POW: f32 = 0.9;
pub(crate) const TEXT_HEIGHT_MULT: f32 = 0.63; // This is the ratio of actual rendered height to given text size
