# Easy Sdl3 Text

This crate adds easy text rendering function for sdl3 using [ab_glyph](https://crates.io/crates/ab_glyph). Current features:
- Cache for reusing textures
- Both regular and sub-pixel rendering
- Multithreaded rasterization
- Pure rust, no compilation headaches

<br>

This might work best as a starting point for you to make your own text rendering library, but it is already very usable on its own. Also, rendering uncached text usually takes over a millisecond (sometimes over 5 ms in the examples), but it's mostly a one-time cost, and frame-time spikes from text rasterizing should very quickly disappear as the program continues running.

**NOTE:** This currently depends on sdl3 version "0.14", ab_glyph version "0.2", and rayon version "1", if any of these crates update and you need this crate to update too, please let me know!

<br>

**Example Output:** (looks better in the demo, run `cargo run --example subpixel --release` with the downloaded crate to see the true sub-pixel rendering)

![Example Image](https://github.com/What42Pizza/easy-sdl3-text/blob/main/images/example.png?raw=true)

<br>

**Example Code:**

```rust
use easy_sdl3_text as sdl3_text;

pub fn example_draw_function<'a, F: ThreadSafeFont>(
	app_data: &AppData,
	canvas: &mut Canvas<Window>,
	texture_creator: &'a TextureCreator<WindowContext>,
	text_cache: &mut sdl3_text::TextCache<'a, F>,
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
