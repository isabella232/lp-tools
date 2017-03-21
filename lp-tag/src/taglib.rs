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
        let ptr = unsafe { ffi::taglib_file_id3v2_tag(self.ptr) };
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
    pub fn add_frame(&self, frame: &Frame) {
        unsafe { ffi::taglib_tag_add_frame(self.ptr, frame.as_frame_ptr()); }
    }

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

pub trait Frame {
    fn as_frame_ptr(&self) -> *mut ffi::Frame;
}

pub struct TextIdentificationFrame {
    ptr: *mut ffi::TextIdentificationFrame,
}

impl TextIdentificationFrame {
    pub fn new(id: &str, encoding: ffi::StringType) -> TextIdentificationFrame {
        let id = CString::new(id).unwrap();

        let ptr = unsafe {
            ffi::taglib_id3v2_text_identification_frame_new(
                id.as_ptr(),
                encoding,
            )
        };

        TextIdentificationFrame { ptr: ptr }
    }

    pub fn set_text(&self, value: &str) {
        let text = CString::new(value).unwrap();

        unsafe {
            ffi::taglib_id3v2_text_identification_frame_set_text(
                self.ptr,
                text.as_ptr(),
            );
        }
    }
}

impl Frame for TextIdentificationFrame {
    fn as_frame_ptr(&self) -> *mut ffi::Frame {
        self.ptr as *mut ffi::Frame
    }
}
