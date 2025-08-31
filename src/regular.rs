use crate::*;
use std::{f32, sync::Mutex};
use ab_glyph::{Glyph, PxScale, PxScaleFont, ScaleFont};
use sdl3::{pixels::{Color, PixelFormat}, rect::Rect, sys::pixels::SDL_PixelFormat};



/// Renders text without sub-pixel rendering (a bit faster and easier to use, but looks a bit pixelated)
pub fn render_text_regular<'a, 'b, F: ThreadSafeFont>(text: impl AsRef<str>, x: i32, y: i32, settings: &mut TextRenderingSettings<'a, 'b, F>) -> Result<(), RenderTextError> {
	let (text, size, h_align, v_align, foreground, texture_creator) = (text.as_ref(), settings.size, settings.h_align, settings.v_align, settings.foreground, settings.texture_creator);
	if text.is_empty() {return Ok(());}
	let font = settings.text_cache.font.as_scaled(PxScale::from(100.0));
	
	// convert chars to glyphs & rasterize uncached glyphs
	let mut glyphs = Vec::with_capacity(text.len());
	let new_textures = Mutex::new(vec!());
	let set_regular = &mut settings.text_cache.set_regular;
	rayon::scope(|s| {
		for c in text.chars() {
			let glyph = font.scaled_glyph(c);
			let is_new = set_regular.insert((c, foreground));
			if is_new {
				let new_textures = &new_textures;
				let glyph = glyph.clone();
				s.spawn(move |_s| {
					let result = rasterize_glyph_regular(glyph.clone(), c, foreground, &font);
					new_textures.lock().unwrap().push(result);
				});
			}
			glyphs.push((c, glyph));
		}
	});
	
	// upload new glyph textures to gpu
	for texture_data in new_textures.into_inner().unwrap() {
		let Some((c, pixels, width, height, x_offset, y_offset)) = texture_data else {continue;};
		let mut texture = texture_creator.create_texture(
			Some(unsafe {PixelFormat::from_ll(SDL_PixelFormat::ABGR8888)}),
			sdl3::render::TextureAccess::Static,
			width,
			height,
		)?;
		texture.update(None, &pixels, width as usize * 4)?;
		settings.text_cache.map_regular.insert((c, foreground), (texture, width, height, x_offset, y_offset));
	}
	let font = settings.text_cache.font.as_scaled(PxScale::from(size));
	
	// get text width & align properly
	let mut width = 0.0;
	let (c, first_glyph) = &glyphs[0];
	width += font.h_advance(first_glyph.id);
	width += size * EXTRA_CHAR_SPACING;
	if c.is_whitespace() {width += size * EXTRA_WHITESPACE_SPACING;}
	for [(_prev_c, prev_glyph), (c, glyph)] in glyphs.array_windows() {
		width += font.kern(prev_glyph.id, glyph.id);
		width += font.h_advance(glyph.id);
		width += size * EXTRA_CHAR_SPACING;
		if c.is_whitespace() {width += size * EXTRA_WHITESPACE_SPACING;}
	}
	width -= size * EXTRA_CHAR_SPACING;
	let mut x = x as f32 + h_align.get_offset(width);
	let y = y as f32 + v_align.get_offset(font.height());
	
	// render first char
	if let Some((c, first_glyph)) = glyphs.first() {
		let texture_data = settings.text_cache.map_regular.get(&(*c, foreground));
		if let Some((texture, width, height, x_offset, y_offset)) = texture_data {
			let dst = Rect::new((x - *x_offset * size / 100.0) as i32, (y - *y_offset * size / 100.0) as i32, (size * (*width as f32 / 100.0)) as u32, (size * (*height as f32 / 100.0)) as u32);
			settings.canvas.copy(texture, None, dst)?;
		}
		x += font.h_advance(first_glyph.id);
		x += size * EXTRA_CHAR_SPACING;
		if c.is_whitespace() {x += size * EXTRA_WHITESPACE_SPACING;}
	}
	
	// render remaining chars (with kerning)
	for [(_prev_c, prev_glyph), (c, glyph)] in glyphs.array_windows() {
		x += font.kern(prev_glyph.id, glyph.id);
		let texture_data = settings.text_cache.map_regular.get(&(*c, foreground));
		if let Some((texture, width, height, x_offset, y_offset)) = texture_data {
			let dst = Rect::new((x - *x_offset * size / 100.0) as i32, (y - *y_offset * size / 100.0) as i32, (size * (*width as f32 / 100.0)) as u32, (size * (*height as f32 / 100.0)) as u32);
			settings.canvas.copy(texture, None, dst)?;
		}
		x += font.h_advance(glyph.id);
		x += size * EXTRA_CHAR_SPACING;
		if c.is_whitespace() {x += size * EXTRA_WHITESPACE_SPACING;}
	}
	
	Ok(())
}



fn rasterize_glyph_regular(glyph: Glyph, c: char, foreground: Color, font: &PxScaleFont<&impl ThreadSafeFont>) -> Option<(char, Vec<u8>, u32, u32, f32, f32)> {
	
	let glyph = font.outline_glyph(glyph)?;
	let bounds = glyph.px_bounds();
	
	let alpha = foreground.a as f32;
	let foreground = [foreground.r, foreground.g, foreground.b, 0];
	let width = bounds.width().ceil() as u32;
	let height = bounds.height().ceil() as u32;
	let mut pixels = foreground.repeat((width * height) as usize);
	glyph.draw(|x, y, v| {
		pixels[((x + y * width) * 4 + 3) as usize] = (alpha * v.powf(REGULAR_VALUE_POW)) as u8;
	});
	
	Some((c, pixels, width, height, -bounds.min.x, -bounds.min.y))
}
