# artscii-img

Load an image and convert it to ASCII art.

> This is the main library entry point to ArtSCII. It re-exports the
> configuration types from [`artscii-core`], so depending on this crate alone is
> enough for most use cases.

[`artscii-core`]: ../artscii-core

## Contents

- [`convert_image`] — turn a [`DynamicImage`] into an [`AsciiResult`].
- [`AsciiResult`] — the grid of characters, optional per-cell color, and
  ready-made formatters: `to_plain_text()`, `to_ansi()`, `to_html()`.
- Re-exports of [`ConvertConfig`], [`DitheringStrategy`], [`ArtsciiError`] from
  `artscii-core`.

[`DynamicImage`]: https://docs.rs/image/latest/image/enum.DynamicImage.html

## Example

```rust
use artscii_img::{convert_image, ConvertConfig, DitheringStrategy};

let img = image::open("photo.png")?;
let config = ConvertConfig {
    resolution: 0.4,
    contrast: 1.0,
    brightness: 1.0,
    inverted: false,
    colored: true,
    dithering: DitheringStrategy::Atkinson,
};

let result = convert_image(&img, &config)?;

// Plain text, ANSI-colored terminal text, or a styled HTML document:
let _text = result.to_plain_text();
let _ansi = result.to_ansi();     // requires the `cli` feature
let _html = result.to_html();
# Ok::<(), artscii_img::ArtsciiImgError>(())
```

## Features

| Flag  | Effect                                                             |
| ----- | ------------------------------------------------------------------ |
| `cli` | Enables `to_ansi()` (depends on `colored`) and `artscii-core/cli`. |

The `cli` feature pulls in `colored` for ANSI terminal coloring. Leave it
disabled for server, WASM, or any other non-terminal environment — the rest of
the crate compiles and works without it.

## Using from another Rust project

```toml
[dependencies]
artscii-img = { git = "https://github.com/4ster-light/artscii" }
# or via cargo add:
# cargo add --git https://github.com/4ster-light/artscii artscii-img
```

Refer to the [workspace README](../README.md) for more details on using the
other crates.

## License

MIT
