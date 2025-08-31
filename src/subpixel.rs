use crate::*;
use std::{f32, sync::Mutex};
use ab_glyph::{Glyph, PxScale, PxScaleFont, ScaleFont};
use sdl3::{pixels::{Color, PixelFormat}, rect::Rect, sys::pixels::SDL_PixelFormat};



/// Renders text with sub-pixel rendering (limited and a bit slower but looks really nice)
pub fn render_text_subpixel<'a, 'b, F: ThreadSafeFont>(text: impl AsRef<str>, x: i32, y: i32, settings: &mut TextRenderingSettings<'a, 'b, F>) -> Result<(), RenderTextError> {
	let (text, size, h_align, v_align, foreground, background, texture_creator) = (text.as_ref(), settings.size as u32, settings.h_align, settings.v_align, settings.foreground, settings.background, settings.texture_creator);
	if text.is_empty() {return Ok(());}
	let mut font = settings.text_cache.font.as_scaled(PxScale::from(size as f32));
	
	// convert chars to glyphs & rasterize uncached glyphs
	font.scale.x *= 3.0; // for sub-pixel rendering
	let mut glyphs = Vec::with_capacity(text.len());
	let new_textures = Mutex::new(vec!());
	let set_subpixel = &mut settings.text_cache.set_subpixel;
	rayon::scope(|s| {
		for c in text.chars() {
			let glyph = font.scaled_glyph(c);
			let is_new = set_subpixel.insert((c, size, foreground, background));
			if is_new {
				let new_textures = &new_textures;
				let glyph = glyph.clone();
				s.spawn(move |_s| {
					let result = rasterize_glyph_subpixel(glyph.clone(), c, foreground, background, &font);
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
		settings.text_cache.map_subpixel.insert((c, size, foreground, background), (texture, width, height, x_offset, y_offset));
	}
	
	// get text width & align properly
	font.scale.x /= 3.0; // undo sub-pixel rendering scaling to do spacing
	let mut width = 0.0;
	let (c, first_glyph) = &glyphs[0];
	width += font.h_advance(first_glyph.id);
	width += size as f32 * EXTRA_CHAR_SPACING;
	if c.is_whitespace() {width += size as f32 * EXTRA_WHITESPACE_SPACING;}
	for [(_prev_c, prev_glyph), (c, glyph)] in glyphs.array_windows() {
		width += font.kern(prev_glyph.id, glyph.id);
		width += font.h_advance(glyph.id);
		width += size as f32 * EXTRA_CHAR_SPACING;
		if c.is_whitespace() {width += size as f32 * EXTRA_WHITESPACE_SPACING;}
	}
	width -= size as f32 * EXTRA_CHAR_SPACING;
	let mut x = x as f32 + h_align.get_offset(width);
	let y = y as f32 + v_align.get_offset(font.height());
	
	// render first char
	if let Some((c, first_glyph)) = glyphs.first() {
		let texture_data = settings.text_cache.map_subpixel.get(&(*c, size, foreground, background));
		if let Some((texture, width, height, x_offset, y_offset)) = texture_data {
			let dst = Rect::new((x - *x_offset) as i32, (y - *y_offset) as i32, *width, *height);
			settings.canvas.copy(texture, None, dst)?;
		}
		x += font.h_advance(first_glyph.id);
		x += size as f32 * EXTRA_CHAR_SPACING;
		if c.is_whitespace() {x += size as f32 * EXTRA_WHITESPACE_SPACING;}
	}
	
	// render remaining chars (with kerning)
	for [(_prev_c, prev_glyph), (c, glyph)] in glyphs.array_windows() {
		x += font.kern(prev_glyph.id, glyph.id);
		let texture_data = settings.text_cache.map_subpixel.get(&(*c, size, foreground, background));
		if let Some((texture, width, height, x_offset, y_offset)) = texture_data {
			let dst = Rect::new((x - *x_offset) as i32, (y - *y_offset) as i32, *width, *height);
			settings.canvas.copy(texture, None, dst)?;
		}
		x += font.h_advance(glyph.id);
		x += size as f32 * EXTRA_CHAR_SPACING;
		if c.is_whitespace() {x += size as f32 * EXTRA_WHITESPACE_SPACING;}
	}
	
	Ok(())
}



fn rasterize_glyph_subpixel(glyph: Glyph, c: char, foreground: Color, background: Color, font: &PxScaleFont<&impl ThreadSafeFont>) -> Option<(char, Vec<u8>, u32, u32, f32, f32)> {
	
	let glyph = font.outline_glyph(glyph)?;
	let bounds = glyph.px_bounds();
	
	let foreground = [foreground.r, foreground.g, foreground.b, foreground.a];
	let background = [background.r, background.g, background.b, background.a];
	let width = bounds.width().ceil() as u32 / 3 + 3; // Note: this is the width of the final image, not the `channel_datas`
	let height = bounds.height().ceil() as u32 + 2;
	let mut channel_datas = vec![0.0; (width * 3 * height) as usize];
	glyph.draw(|x, y, v| {
		let (x, y) = (x + 1, y + 1);
		channel_datas[(x + y * width * 3) as usize] = v.clamp(0.0, 1.0);
	});
	let mut channel_datas_alt = vec![0.0; (width * 3 * height) as usize];
	const HORIZONTAL_WEIGHTS: [f32; 5] = [0.09526326, 0.55556049, 1.0, 0.55556049, 0.09526326];
	for x in 0.. (width * 3) as usize {
		for y in 0.. height as usize {
			let left_margin = x.min(2);
			let right_margin = ((width * 3) as usize - 1 - x).min(2);
			let i = x + y * (width * 3) as usize;
			let pixels = &channel_datas[i - left_margin ..= i + right_margin];
			let weights = &HORIZONTAL_WEIGHTS[2 - left_margin ..= 2 + right_margin];
			let mut total = 0.0;
			let mut total_weight = 0.0;
			for (pixel, weight) in pixels.iter().zip(weights) {
				total += pixel * weight;
				total_weight += weight;
			}
			channel_datas_alt[i] = total / total_weight;
		}
	}
	for x in 0.. (width * 3) as usize {
		for y in 0.. height as usize {
			let mut total = 0.0;
			let mut total_weight = 0.0;
			let i = x + y * (width * 3) as usize;
			if y > 0 {
				total += channel_datas_alt[i - (width * 3) as usize] * 0.00504176;
				total_weight += 0.00504176;
			}
			total += channel_datas_alt[i];
			total_weight += 1.0;
			if y < height as usize - 1 {
				total += channel_datas_alt[i + (width * 3) as usize] * 0.00504176;
				total_weight += 0.00504176;
			}
			channel_datas[i] = (total / total_weight).powf(SUBPIXEL_VALUE_POW);
		}
	}
	let mut pixels = vec![0; (width * 4 * height) as usize];
	for x in 0..width as usize {
		for y in 0..height as usize {
			let red_value   = (channel_datas[x * 3     + y * (width * 3) as usize] * 255.0) as u16;
			let green_value = (channel_datas[x * 3 + 1 + y * (width * 3) as usize] * 255.0) as u16;
			let blue_value  = (channel_datas[x * 3 + 2 + y * (width * 3) as usize] * 255.0) as u16;
			let alpha_value = (red_value + green_value + blue_value) / 3;
			let red   = background[0] as u16 * (255 - red_value  ) / 255 + foreground[0] as u16 * red_value   / 255;
			let green = background[1] as u16 * (255 - green_value) / 255 + foreground[1] as u16 * green_value / 255;
			let blue  = background[2] as u16 * (255 - blue_value ) / 255 + foreground[2] as u16 * blue_value  / 255;
			let alpha = background[3] as u16 * (255 - alpha_value) / 255 + foreground[3] as u16 * alpha_value / 255;
			pixels[(x + y * width as usize) * 4    ] = red as u8;
			pixels[(x + y * width as usize) * 4 + 1] = green as u8;
			pixels[(x + y * width as usize) * 4 + 2] = blue as u8;
			pixels[(x + y * width as usize) * 4 + 3] = alpha as u8;
		}
	}
	
	Some((c, pixels, width, height, -bounds.min.x / 3.0, -bounds.min.y))
}
