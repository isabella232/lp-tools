use ::ffi;
use std::ffi::CString;
use std::path::Path;

pub struct File {
    ptr: *mut ffi::File,
}

impl File {
    pub fn new<P>(path: P) -> File where P: AsRef<Path> {
        let path = path.as_ref();
        let c_path = CString::new(path.to_str().unwrap()).unwrap();
        let ptr = unsafe { ffi::taglib_file_new(c_path.as_ptr()) };
        File { ptr: ptr }
    }

    pub fn save(&self) -> bool {
         unsafe { ffi::taglib_file_save(self.ptr) }
    }

    pub fn strip(&self) -> bool {
        unsafe { ffi::taglib_file_strip(self.ptr) }
    }

    pub fn tag(&self) -> Tag {
        let ptr = unsafe { ffi::taglib_file_tag(self.ptr) };
        Tag { ptr: ptr }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { ffi::taglib_file_free(self.ptr); }
    }
}

pub struct Tag {
    ptr: *mut ffi::Tag,
}

impl Tag {
    pub fn set_title(&self, value: &str) {
        let title = CString::new(value).unwrap();
        unsafe { ffi::taglib_tag_set_title(self.ptr, title.as_ptr()); }
    }

    pub fn set_artist(&self, value: &str) {
        let artist = CString::new(value).unwrap();
        unsafe { ffi::taglib_tag_set_artist(self.ptr, artist.as_ptr()); }
    }

    pub fn set_album(&self, value: &str) {
        let album = CString::new(value).unwrap();
        unsafe { ffi::taglib_tag_set_album(self.ptr, album.as_ptr()); }
    }

    pub fn set_genre(&self, value: &str) {
        let genre = CString::new(value).unwrap();
        unsafe { ffi::taglib_tag_set_genre(self.ptr, genre.as_ptr()); }
    }

    pub fn set_year(&self, year: u32) {
        unsafe { ffi::taglib_tag_set_year(self.ptr, year); }
    }
}
