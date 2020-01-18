use crate::ffi;

mod image;

use std::sync::Once;

static GENESIS: Once = Once::new();

fn magick_wand_genesis() {
    unsafe {
        ffi::MagickWandGenesis();
    }
}

pub struct MagickWand {
    ptr: *mut ffi::MagickWand,
}

impl MagickWand {
    pub fn new() -> MagickWand {
        GENESIS.call_once(|| magick_wand_genesis());
        MagickWand::default()
    }

    pub fn as_ptr(&self) -> *mut ffi::MagickWand {
        self.ptr
    }
}

impl Default for MagickWand {
    fn default() -> MagickWand {
        let ptr = unsafe { ffi::NewMagickWand() };
        MagickWand::from(ptr)
    }
}

impl Drop for MagickWand {
    fn drop(&mut self) {
        unsafe {
            ffi::DestroyMagickWand(self.as_ptr());
        }
    }
}

impl From<*mut ffi::MagickWand> for MagickWand {
    fn from(ptr: *mut ffi::MagickWand) -> MagickWand {
        MagickWand { ptr }
    }
}
