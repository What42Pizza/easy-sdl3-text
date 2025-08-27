# Easy Sdl3 Text

This crate adds easy text rendering function for sdl3 using [ab_glyph](https://crates.io/crates/ab_glyph). Current features:
- Cache for reusing textures
- Both regular and sub-pixel rendering
- Multithreaded rasterization
- Pure rust, no compilation headaches

<br>

This might work best as a starting point for you to make your own text rendering library, but it is already very usable on its own. Also, render uncached text can often take over a millisecond (sometimes taking over 5 ms in the examples), but it's mostly a one-time cost, and frame-time spikes from text rasterizing should very quickly disappear as the program continues running.

<br>

**Example Output:** (looks better in the demo, run `cargo run --example subpixel --release` to see the true sub-pixel rendering)

![Example Image](https://github.com/What42Pizza/easy-sdl3-text/blob/main/images/example.png?raw=true)

<br>

**Example Code:**

```
use easy_sdl3_text as sdl3_text;

pub fn example_draw_function<'a, Font: ThreadSafeFont>(
	app_data: &AppData,
	canvas: &mut Canvas<Window>,
	texture_creator: &'a TextureCreator<WindowContext>,
	text_cache: &mut sdl3_text::TextCache<'a, Font>,
) -> anyhow::Result<()> {
	canvas.set_draw_color(Color::RGB(255, 255, 255));
	canvas.clear();
	
	let size = 25;
	let (x, y) = (50, 50);
	let foreground = Color::RGB(30, 30, 30);
	let background = Color::RGB(255, 255, 255);
	sdl3_text::render_text_subpixel(
		"Example text",
		size,
		x, y,
		foreground, background,
		canvas,
		texture_creator,
		text_cache,
	)?;
	
	canvas.present();
	Ok(())
}
```

<br>

This crate and all its code is dedicated to the public domain, being licensed under [CC0 1.0 Universal](LICENSE)
