use ::ffi;

mod image;

pub struct MagickWand {
    ptr: *mut ffi::MagickWand,
}

impl MagickWand {
    pub fn new() -> MagickWand {
        let ptr = unsafe { ffi::NewMagickWand() };
        MagickWand::from(ptr)
    }

    pub fn as_ptr(&self) -> *mut ffi::MagickWand {
        self.ptr
    }
}

impl Drop for MagickWand {
    fn drop(&mut self) {
        unsafe { ffi::DestroyMagickWand(self.as_ptr()); }
    }
}

impl From<*mut ffi::MagickWand> for MagickWand {
    fn from(ptr: *mut ffi::MagickWand) -> MagickWand {
        MagickWand { ptr: ptr }
    }
}
