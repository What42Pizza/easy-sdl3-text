use std::collections::{HashMap, HashSet};
use sdl3::{pixels::Color, render::{Texture, TextureValueError, UpdateTextureError}, Error};
use ab_glyph::Font;



/// Basically a wrapper for `ab_glyph::Font` that also implements `Send` and `Sync`
pub trait ThreadSafeFont: Font + Send + Sync {}

impl<T: Font + Send + Sync> ThreadSafeFont for T {}



/// A cache for character textures, this must be passed to `render_text_regular()` and `render_text_subpixel()`
pub struct TextCache<'a, F: ThreadSafeFont> {
	// (char, foreground) -> (texture, width, height, x_offset, y_offset)
	pub(crate) map_regular: HashMap<(char, Color), (Texture<'a>, u32, u32, f32, f32)>,
	pub(crate) set_regular: HashSet<(char, Color)>,
	// NOTE: this can kinda look a bit nicer if `size` here is replaced with usize and `size` as input for `render_text_*()` is replaced with f32 (which allows for better text scaling), but that significantly increases the number of textures to rasterize and store
	// (char, size, foreground, background) -> (texture, width, height, x_offset, y_offset)
	pub(crate) map_subpixel: HashMap<(char, u32, Color, Color), (Texture<'a>, u32, u32, f32, f32)>,
	pub(crate) set_subpixel: HashSet<(char, u32, Color, Color)>,
	pub(crate) font: F,
}

impl<'a, F: ThreadSafeFont> TextCache<'a, F> {
	/// Creates a new TextCache
	#[inline]
	pub fn new(font: F) -> Self {
		Self {
			map_regular: HashMap::new(),
			set_regular: HashSet::new(),
			map_subpixel: HashMap::new(),
			set_subpixel: HashSet::new(),
			font,
		}
	}
	/// Switches this cache to a different font (and clears the cache so the characters can be re-rendered)
	pub fn switch_font(&mut self, new_font: F) {
		self.font = new_font;
		self.clear();
	}
	/// Clears the cache, probably should only be done if the program is actually low on ram or vram
	pub fn clear(&mut self) {
		self.map_regular.clear();
		self.set_regular.clear();
		self.map_subpixel.clear();
		self.set_subpixel.clear();
	}
}



/// A wrapper for all errors that can occur while rendering text
#[derive(Debug)]
pub enum RenderTextError {
	/// Wrapper for sdl3::Error
	SdlError (Error),
	/// Wrapper for sdl3::texture::TextureValueError
	SdlTextureValueError (TextureValueError),
	/// Wrapper for sdl3::texture::UpdateTextureError
	SdlUpdateTextureError (UpdateTextureError),
}

impl From<Error> for RenderTextError {
	fn from(value: Error) -> Self {
		Self::SdlError (value)
	}
}

impl From<TextureValueError> for RenderTextError {
	fn from(value: TextureValueError) -> Self {
		Self::SdlTextureValueError (value)
	}
}

impl From<UpdateTextureError> for RenderTextError {
	fn from(value: UpdateTextureError) -> Self {
		Self::SdlUpdateTextureError (value)
	}
}

impl std::fmt::Display for RenderTextError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::SdlError (err) => write!(f, "Render Text Error: {err}"),
			Self::SdlTextureValueError (err) => write!(f, "Render Text Error: {err}"),
			Self::SdlUpdateTextureError (err) => write!(f, "Render Text Error: {err}"),
		}
	}
}

impl std::error::Error for RenderTextError {}
