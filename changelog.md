### Note: this crate uses [Romantic Versioning](https://github.com/romversioning/romver)

<br>

- v0.3.1 (25/09/01)
  - Api changes:
    - `TextRenderingSettings::new_regular()` and `TextRenderingSettings::new_subpixel()` now take `Into<sdl3::pixels::Color>` instead of `sdl3::pixels::Color`
  - Updated and polished documentation

<br>

- **v0.3.0:** (25/08/31)
  - Api changes:
    - `render_text_regular()` and `render_text_subpixel()` now take `TextRenderingSettings` in place of multiple other arguments
  - Tweaked character spacing

<br>

- **v0.2.0:** (25/08/28)
  - Added text alignment and `TextCache::switch_font()`
  - Api changes:
    - `TextCache::new()` now takes `ThreadSafeFont` instead of `&ThreadSafeFont`
	- `render_text_regular()` and `render_text_subpixel()` now take two more arguments: `h_align: sdl3_text::HAlign` and `v_align: sdl3_text::VAlign`
  - Still depends on sdl3 version "0.14", ab_glyph version "0.2", and rayon version "1"

<br>

- **v0.1.0:** (25/08/27)
  - Initial release
  - Includes regular and sub-pixel rendering
  - Depends on sdl3 version "0.14", ab_glyph version "0.2", and rayon version "1"
