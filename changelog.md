### Note: this crate uses [Romantic Versioning](https://github.com/romversioning/romver)

<br>

- v0.2.1
  - Api changes:
    - `render_text_regular()` and `render_text_subpixel()` now take `Into<HAlign>` and `Into<VAlign>` instead of `HAlign` and `VAlign`
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
