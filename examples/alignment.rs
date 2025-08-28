#![feature(duration_constants)]



use std::time::Instant;
use easy_sdl3_text::{self as sdl3_text, ThreadSafeFont};
use ab_glyph::FontRef;
use sdl3::{mouse::MouseState, pixels::Color, render::FRect};
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
	
	// the window starts hidden to avoid weird startup visuals, so make it show here
	canvas.present();
	canvas.window_mut().show();
	
	// Note: it is strongly suggested that the canvas, texture creator, and text cache are not lumped in with the rest of the program data because of lifetime issues
	let texture_creator = canvas.texture_creator();
	let font = FontRef::try_from_slice(include_bytes!("resources/Inter_24pt-Regular.ttf"))?;
	let mut text_cache = sdl3_text::TextCache::new(font);
	
	let mut app_data = AppData {
		should_close: false,
	};
	
	// main loop
	while !app_data.should_close {
		
		// handle events
		for event in event_pump.poll_iter() { handle_event(&mut app_data, event)?; }
		
		// other program logic can go here
		
		// render
		let mouse_state = event_pump.mouse_state();
		draw(&mut canvas, &texture_creator, &mut text_cache, mouse_state)?;
		
	}
	
	Ok(())
}



fn handle_event(app_data: &mut AppData, event: Event) -> Result<()> {
	match event {
		
		Event::Quit { timestamp: _ } => app_data.should_close = true,
		
		// close the example if ctrl+w or ctrl+q is pressed
		Event::KeyDown { timestamp: _, window_id: _, keycode, scancode: _, keymod, repeat: _, which: _, raw: _ } => {
			if (keycode == Some(sdl3::keyboard::Keycode::W) || keycode == Some(sdl3::keyboard::Keycode::Q)) && (keymod.contains(Mod::RCTRLMOD) || keymod.contains(Mod::LCTRLMOD)) {
				app_data.should_close = true;
			}
		}
		
		_ => {}
	}
	Ok(())
}



pub fn draw<'a, F: ThreadSafeFont>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, text_cache: &mut sdl3_text::TextCache<'a, F>, mouse_state: MouseState) -> Result<()> {
	let start = Instant::now();
	canvas.set_draw_color(Color::RGB(255, 255, 255));
	canvas.clear();
	let canvas_size = canvas.output_size()?;
	let (width, height) = (canvas_size.0 as f32, canvas_size.1 as f32);
	
	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.fill_rect(FRect::new(0.0, height * 0.5, width, 1.0))?;
	canvas.fill_rect(FRect::new(width * 0.5, 0.0, 1.0, height))?;
	
	let (mouse_x, mouse_y) = (mouse_state.x(), mouse_state.y());
	let indicator_width = (mouse_x - width * 0.5).abs() * 2.0;
	let indicator_height = (mouse_y - height * 0.5).abs() * 2.0;
	canvas.draw_rect(FRect::new(width * 0.5 - indicator_width * 0.5, height * 0.5 - indicator_height * 0.5, indicator_width, indicator_height))?;
	
	sdl3_text::render_text_regular("Example text 1234567890 !@#$%^&*()_+-=[]{}|;:',.<>/?~", height * 0.07, width as i32 / 2, height as i32 / 2, sdl3_text::HAlign::Center, sdl3_text::VAlign::Center, Color::RGB(30, 30, 30), canvas, texture_creator, text_cache)?;
	
	let millis = start.elapsed().as_millis();
	if millis > 0 {println!("Draw time exceeds 0 ms: {millis} ms");}
	canvas.present();
	Ok(())
}
