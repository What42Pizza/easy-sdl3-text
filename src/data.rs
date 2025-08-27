use std::collections::{HashMap, HashSet};
use sdl3::{pixels::Color, render::{Texture, TextureValueError, UpdateTextureError}, Error};
use ab_glyph::Font;



pub trait ThreadSafeFont: Font + Send + Sync {}

impl<T: Font + Send + Sync> ThreadSafeFont for T {}

// (char, size, foreground, background) -> (texture, width, height, x_offset, y_offset)
#[derive(Default)]
pub struct TextCache<'a> {
	pub(crate) map: HashMap<(char, u32, Color, Color), (Texture<'a>, u32, u32, f32, f32)>,
	pub(crate) set: HashSet<(char, u32, Color, Color)>,
}

impl<'a> TextCache<'a> {
	#[inline]
	pub fn new() -> Self {
		Self::default()
	}
	pub fn clear(&mut self) {
		self.map.clear();
		self.set.clear();
	}
}



#[derive(Debug)]
pub enum RenderTextError {
	SdlError (Error),
	SdlTextureValueError (TextureValueError),
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
