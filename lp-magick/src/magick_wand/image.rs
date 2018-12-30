use std::ffi::CString;
use std::path::Path;

use crate::ffi::{self, FilterType, GravityType};
use crate::MagickWand;

impl MagickWand {
    pub fn read_image<P>(&self, path: P) where P: AsRef<Path> {
        let p = path.as_ref().to_str().unwrap();
        let pathname = CString::new(p).unwrap();
        unsafe { ffi::MagickReadImage(self.as_ptr(), pathname.as_ptr()); }
    }

    pub fn write_image<P>(&self, path: P) where P: AsRef<Path> {
        let p = path.as_ref().to_str().unwrap();
        let pathname = CString::new(p).unwrap();
        unsafe { ffi::MagickWriteImage(self.as_ptr(), pathname.as_ptr()); }
    }

    pub fn crop_image(&self, width: usize, height: usize, x: isize, y: isize) {
        unsafe { ffi::MagickCropImage(self.as_ptr(), width, height, x, y); }
    }

    pub fn image_width(&self) -> usize {
        unsafe { ffi::MagickGetImageWidth(self.as_ptr()) }
    }

    pub fn image_height(&self) -> usize {
        unsafe { ffi::MagickGetImageHeight(self.as_ptr()) }
    }

    pub fn resize_image(&self, width: usize, height: usize) {
        let filter = FilterType::Lanczos2Filter;
        unsafe { ffi::MagickResizeImage(self.as_ptr(), width, height, filter); }
    }

    pub fn set_image_extent(&self, width: usize, height: usize) {
        unsafe { ffi::MagickSetImageExtent(self.as_ptr(), width, height); }
    }

    pub fn set_image_gravity(&self, gravity: GravityType) {
        unsafe { ffi::MagickSetImageGravity(self.as_ptr(), gravity); }
    }
}
