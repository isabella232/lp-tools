use libc::{c_char, c_uint};

pub enum File {}
pub enum Tag {}

#[link(name = "taglib")]
extern {
    pub fn taglib_file_new(pathname: *const c_char) -> *mut File;
    pub fn taglib_file_free(file: *mut File);
    pub fn taglib_file_save(file: *mut File) -> bool;
    pub fn taglib_file_strip(file: *mut File) -> bool;
    pub fn taglib_file_tag(file: *mut File) -> *mut Tag;

    pub fn taglib_tag_set_title(tag: *mut Tag, value: *const c_char);
    pub fn taglib_tag_set_artist(tag: *mut Tag, value: *const c_char);
    pub fn taglib_tag_set_album(tag: *mut Tag, value: *const c_char);
    pub fn taglib_tag_set_genre(tag: *mut Tag, value: *const c_char);
    pub fn taglib_tag_set_year(tag: *mut Tag, value: c_uint);
}
