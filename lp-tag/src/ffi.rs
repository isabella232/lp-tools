use libc::{c_char, c_uint};

pub enum File {}
pub enum Frame {}
pub enum Tag {}
pub enum TextIdentificationFrame {}

#[repr(C)]
pub enum StringType {
    Latin1,
    UTF16,
    UTF16BE,
    UTF8,
    UTF16LE,
}

#[link(name = "taglib")]
extern {
    pub fn taglib_file_new(pathname: *const c_char) -> *mut File;
    pub fn taglib_file_free(file: *mut File);
    pub fn taglib_file_save(file: *mut File) -> bool;
    pub fn taglib_file_strip(file: *mut File) -> bool;
    pub fn taglib_file_id3v2_tag(file: *mut File) -> *mut Tag;

    pub fn taglib_tag_add_frame(tag: *mut Tag, frame: *const Frame);
    pub fn taglib_tag_set_title(tag: *mut Tag, value: *const c_char);
    pub fn taglib_tag_set_artist(tag: *mut Tag, value: *const c_char);
    pub fn taglib_tag_set_album(tag: *mut Tag, value: *const c_char);
    pub fn taglib_tag_set_genre(tag: *mut Tag, value: *const c_char);
    pub fn taglib_tag_set_year(tag: *mut Tag, value: c_uint);

    pub fn taglib_id3v2_text_identification_frame_new(id: *const c_char, encoding: StringType) -> *mut TextIdentificationFrame;
    pub fn taglib_id3v2_text_identification_frame_set_text(frame: *mut TextIdentificationFrame, value: *const c_char);
}
