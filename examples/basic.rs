#![feature(duration_constants)]



use std::time::Instant;

use easy_sdl3_text as sdl3_text;
use ab_glyph::FontRef;
use sdl3::pixels::Color;
pub use sdl3::{render::Canvas, video::Window, event::Event, keyboard::Mod, render::{Texture, TextureCreator}, video::WindowContext};
pub use anyhow::*;



struct AppData {
	should_close: bool,
}



fn main() -> Result<()> {
	
	// init sdl, window, etc
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem
		.window("Text Example", 1400, 600)
		.position_centered()
		.resizable()
		.hidden()
		.build()?;
	let mut canvas = window.into_canvas();
	let mut event_pump = sdl_context.event_pump().unwrap();
	unsafe {
		sdl3::sys::render::SDL_SetRenderVSync(canvas.raw(), 1);
	}
	
	canvas.present();
	canvas.window_mut().show();
	
	let texture_creator = canvas.texture_creator();
	let mut text_cache = sdl3_text::TextCache::new();
	let font = FontRef::try_from_slice(include_bytes!("resources/Inter_24pt-Regular.ttf"))?;
	
	let mut app_data = AppData {
		should_close: false,
	};
	
	while !app_data.should_close {
		
		for event in event_pump.poll_iter() { handle_event(&mut app_data, event)?; }
		
		draw(&mut canvas, &texture_creator, &mut text_cache, &font)?;
		
	}
	
	Ok(())
}



fn handle_event(app_data: &mut AppData, event: Event) -> Result<()> {
	match event {
		
		Event::Quit { timestamp: _ } => app_data.should_close = true,
		
		Event::KeyDown { timestamp: _, window_id: _, keycode, scancode: _, keymod, repeat: _, which: _, raw: _ } => {
			if (keycode == Some(sdl3::keyboard::Keycode::W) || keycode == Some(sdl3::keyboard::Keycode::Q)) && (keymod.contains(Mod::RCTRLMOD) || keymod.contains(Mod::LCTRLMOD)) {
				app_data.should_close = true;
			}
		}
		
		_ => {}
	}
	Ok(())
}



pub fn draw<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, text_cache: &mut sdl3_text::TextCache<'a>, font: &FontRef) -> Result<()> {
	let start = Instant::now();
	canvas.set_draw_color(Color::RGB(255, 255, 255));
	canvas.clear();
	let (width, height) = canvas.output_size()?;
	let (_width, height) = (width as f32, height as f32);
	
	let mut size = height * 0.1;
	let mut y = size;
	while size > 10.0 {
		sdl3_text::render_text_subpixel("Example text 1234567890 !@#$%^&*()_+-=[]{}|;:',.<>/?~", size as u32, (height * 0.1) as i32, y as i32, Color::RGB(30, 30, 30), Color::RGB(255, 255, 255), canvas, texture_creator, text_cache, font)?;
		size *= 0.8;
		y += size * 1.3;
	}
	
	let millis = start.elapsed().as_millis();
	if millis > 0 {println!("Draw time exceeds 0 ms: {millis} ms");}
	canvas.present();
	Ok(())
}
