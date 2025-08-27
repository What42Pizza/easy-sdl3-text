use crate::*;
use std::{f32, sync::Mutex};
use ab_glyph::{Glyph, PxScale, PxScaleFont, ScaleFont};
use sdl3::{pixels::{Color, PixelFormat}, rect::Rect, render::{Canvas, TextureCreator}, sys::pixels::SDL_PixelFormat, video::{Window, WindowContext}};



/// Renders text without sub-pixel rendering (fast and easy to use, but looks a bit pixelated)
pub fn render_text_regular<'a>(text: impl AsRef<str>, size: u32, x: i32, mut y: i32, foreground: Color, background: Color, canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, text_cache: &mut TextCache<'a>, font: &impl ThreadSafeFont) -> Result<(), RenderTextError> {
	let text = text.as_ref();
	if text.is_empty() {return Ok(());}
	let mut font = font.as_scaled(PxScale::from(size as f32));
	font.scale.x *= 3.0; // for sub-pixel rendering
	let mut glyphs = Vec::with_capacity(text.len());
	let new_textures = Mutex::new(vec!());
	// convert chars to glyphs & rasterize uncached glyphs
	rayon::scope(|s| {
		for c in text.chars() {
			let glyph = font.scaled_glyph(c);
			let is_new = text_cache.set_subpixel.insert((c, size, foreground, background));
			if is_new {
				let new_textures = &new_textures;
				let glyph = glyph.clone();
				s.spawn(move |_s| {
					let result = rasterize_glyph_regular(glyph.clone(), c, foreground, background, &font);
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
		text_cache.map_subpixel.insert((c, size, foreground, background), (texture, width, height, x_offset, y_offset));
	}
	// render first char
	font.scale.x /= 3.0; // undo sub-pixel rendering for spacing
	let mut x = x as f32;
	y += font.height() as i32 * 75 / 100;
	if let Some((c, glyph)) = glyphs.first() {
		let texture_data = text_cache.map_subpixel.get(&(*c, size, foreground, background));
		if let Some((texture, width, height, x_offset, y_offset)) = texture_data {
			let dst = Rect::new((x - *x_offset) as i32, y - *y_offset as i32, *width, *height);
			canvas.copy(texture, None, dst)?;
		}
		x += font.h_advance(glyph.id);
	}
	// render remaining chars (with kerning)
	for [(_prev_c, prev_glyph), (c, glyph)] in glyphs.array_windows() {
		x += font.kern(prev_glyph.id, glyph.id);
		let texture_data = text_cache.map_subpixel.get(&(*c, size, foreground, background));
		if let Some((texture, width, height, x_offset, y_offset)) = texture_data {
			let dst = Rect::new((x - *x_offset) as i32, y - *y_offset as i32, *width, *height);
			canvas.copy(texture, None, dst)?;
		}
		x += font.h_advance(glyph.id);
		x += size as f32 * 0.03;
		if c.is_whitespace() {x += size as f32 * 0.04;}
	}
	Ok(())
}



fn rasterize_glyph_regular(glyph: Glyph, c: char, foreground: Color, background: Color, font: &PxScaleFont<&impl ThreadSafeFont>) -> Option<(char, Vec<u8>, u32, u32, f32, f32)> {
	
	let Some(glyph) = font.outline_glyph(glyph) else {return None;};
	let bounds = glyph.px_bounds();
	
	let foreground = [foreground.r, foreground.g, foreground.b, foreground.a];
	let background = [background.r, background.g, background.b, background.a];
	let width = bounds.width().ceil() as u32 / 3 + 3; // Note: this is the width of the final image
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
			channel_datas[i] = (total / total_weight).powf(0.9);
		}
	}
	let mut pixels = vec![0; (width * 4 * height) as usize];
	for x in 0..width as usize {
		for y in 0..height as usize {
			let red_value = (channel_datas[x * 3 + y * (width * 3) as usize] * 255.0) as u16;
			let green_value = (channel_datas[x * 3 + 1 + y * (width * 3) as usize] * 255.0) as u16;
			let blue_value = (channel_datas[x * 3 + 2 + y * (width * 3) as usize] * 255.0) as u16;
			let alpha_value = (red_value + green_value + blue_value) / 3;
			let red = background[0] as u16 * (255 - red_value) / 255 + foreground[0] as u16 * red_value / 255;
			let green = background[1] as u16 * (255 - green_value) / 255 + foreground[1] as u16 * green_value / 255;
			let blue = background[2] as u16 * (255 - blue_value) / 255 + foreground[2] as u16 * blue_value / 255;
			let alpha = background[3] as u16 * (255 - alpha_value) / 255 + foreground[3] as u16 * alpha_value / 255;
			pixels[(x + y * width as usize) * 4] = red as u8;
			pixels[(x + y * width as usize) * 4 + 1] = green as u8;
			pixels[(x + y * width as usize) * 4 + 2] = blue as u8;
			pixels[(x + y * width as usize) * 4 + 3] = alpha as u8;
		}
	}
	
	Some((c, pixels, width, height, -bounds.min.x / 3.0, -bounds.min.y))
}
