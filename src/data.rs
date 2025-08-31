use crate::*;
use std::collections::{HashMap, HashSet};
use sdl3::{pixels::Color, render::{Canvas, Texture, TextureCreator, TextureValueError, UpdateTextureError}, video::{Window, WindowContext}, Error};
use ab_glyph::Font;



/// This is a holder for most of the arguments to `render_text_regular()` and `render_text_subpixel()`
pub struct TextRenderingSettings<'a, 'b, F: ThreadSafeFont> {
	/// NOTE: for `render_text_subpixel()`, this is converted to u32 (this is don to significantly cut down on the number of character textures to rasterize and cache)
	pub size: f32,
	#[allow(missing_docs)]
	pub h_align: HAlign,
	#[allow(missing_docs)]
	pub v_align: VAlign,
	#[allow(missing_docs)]
	pub foreground: Color,
	/// This only exists for `render_text_subpixel()`, with `render_text_regular()` you can set this to whatever you want and it won't affect anything
	pub background: Color,
	#[allow(missing_docs)]
	pub canvas: &'a mut Canvas<Window>,
	#[allow(missing_docs)]
	pub texture_creator: &'b TextureCreator<WindowContext>,
	#[allow(missing_docs)]
	pub text_cache: &'a mut TextCache<'b, F>,
}

impl<'a, 'b, F:ThreadSafeFont> TextRenderingSettings<'a, 'b, F> {
	/// Creates a new `TextRenderingSettings` that is meant to be used with `render_text_regular()`, but can also be used for subpixel rendering
	#[allow(clippy::too_many_arguments)]
	pub fn new_regular(size: f32, h_align: impl Into<HAlign>, v_align: impl Into<VAlign>, foreground: Color, canvas: &'a mut Canvas<Window>, texture_creator: &'b TextureCreator<WindowContext>, text_cache: &'a mut TextCache<'b, F>) -> Self {
		Self {
			size,
			h_align: h_align.into(),
			v_align: v_align.into(),
			foreground,
			background: Color::RGB(127, 127, 127),
			canvas,
			texture_creator,
			text_cache,
		}
	}
	/// Creates a new `TextRenderingSettings` that is meant to be used with `render_text_subpixel()`, but can also be used for regular rendering
	#[allow(clippy::too_many_arguments)]
	pub fn new_subpixel(size: u32, h_align: impl Into<HAlign>, v_align: impl Into<VAlign>, foreground: Color, background: Color, canvas: &'a mut Canvas<Window>, texture_creator: &'b TextureCreator<WindowContext>, text_cache: &'a mut TextCache<'b, F>) -> Self {
		Self {
			size: size as f32,
			h_align: h_align.into(),
			v_align: v_align.into(),
			foreground,
			background,
			canvas,
			texture_creator,
			text_cache,
		}
	}
}



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



/// Horizontal alignment
#[derive(Copy, Clone)]
pub enum HAlign {
	/// Treats the 'x' value as the left edge
	Left,
	/// Treats the 'x' value as the text middle
	Center,
	/// Treats the 'x' value as the right edge
	Right,
}

impl HAlign {
	pub(crate) fn get_offset(&self, width: f32) -> f32 {
		match self {
			Self::Left => 0.0,
			Self::Center => width * -0.5,
			Self::Right => -width,
		}
	}
}

/// Vertical alignment
#[derive(Copy, Clone)]
pub enum VAlign {
	/// Treats the 'x' value as the top edge
	Top,
	/// Treats the 'x' value as the text middle
	Center,
	/// Treats the 'x' value as the bottom edge
	Bottom,
}

impl VAlign {
	pub(crate) fn get_offset(&self, height: f32) -> f32 {
		match self {
			Self::Top => height * TEXT_HEIGHT_MULT,
			Self::Center => height * TEXT_HEIGHT_MULT * 0.5,
			Self::Bottom => 0.0,
		}
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
