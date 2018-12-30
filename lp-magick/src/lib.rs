pub use self::magick_wand::MagickWand;

pub mod ffi;
pub mod magick_wand;

use std::path::Path;

pub fn resize<P>(src: P, dst: P, width: usize, height: usize) where P: AsRef<Path> {
    let wand = MagickWand::new();
    wand.read_image(src);

    let w = wand.image_width();
    let h = wand.image_height();

    let (constrained_width, constrained_height) = if w >= h {
        (w * height / h, height)
    } else {
        (width, h * width / w)
    };

    let x = (constrained_width - width) / 2;
    let y = (constrained_height - height) / 2;

    wand.resize_image(constrained_width, constrained_height);
    wand.crop_image(width, height, x as isize, y as isize);

    wand.write_image(dst);

}
