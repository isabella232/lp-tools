# lp-magick

**lp-magick** is a simple binding to `MagickWand-7` for creating thumbnails.

# Usage

There is a single `resize` function that will resize the image, filling the
area to the given dimensions and cropping the excess.

```rust
extern crate lp_magick;

use lp_magick::resize;

resize("in.jpg", "out.jpg", 120, 120);
```

This produces similar results to

    $ magick in.jpg -resize '120x120^' -gravity center -extent 120x120 out.jpg
