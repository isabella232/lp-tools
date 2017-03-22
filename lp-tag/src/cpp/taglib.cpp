#include <taglib/attachedpictureframe.h>
#include <taglib/id3v2tag.h>
#include <taglib/mpegfile.h>
#include <taglib/tag.h>
#include <taglib/textidentificationframe.h>

static const int ID3V2_VERSION = 3;

TagLib::String c_str_to_string(const char *s) {
  return TagLib::String(s, TagLib::String::UTF8);
}

extern "C" {

TagLib::MPEG::File *taglib_file_new(char const *pathname) {
  return new TagLib::MPEG::File(pathname);
}

void taglib_file_free(TagLib::MPEG::File *file) {
  delete file;
}

bool taglib_file_save(TagLib::MPEG::File *file) {
  return file->save(TagLib::MPEG::File::AllTags, true, ID3V2_VERSION);
}

bool taglib_file_strip(TagLib::MPEG::File *file) {
  return file->strip();
}

TagLib::ID3v2::Tag *taglib_file_id3v2_tag(TagLib::MPEG::File *file) {
  return file->ID3v2Tag(true);
}

void taglib_tag_add_frame(TagLib::ID3v2::Tag *tag, TagLib::ID3v2::Frame *frame) {
  tag->addFrame(frame);
}

void taglib_tag_set_title(TagLib::ID3v2::Tag *tag, const char *value) {
  TagLib::String title = c_str_to_string(value);
  return tag->setTitle(title);
}

void taglib_tag_set_artist(TagLib::ID3v2::Tag *tag, const char *value) {
  TagLib::String artist = c_str_to_string(value);
  return tag->setArtist(artist);
}

void taglib_tag_set_album(TagLib::ID3v2::Tag *tag, const char *value) {
  TagLib::String album = c_str_to_string(value);
  return tag->setAlbum(album);
}

void taglib_tag_set_genre(TagLib::ID3v2::Tag *tag, const char *value) {
  TagLib::String genre = c_str_to_string(value);
  return tag->setGenre(genre);
}

void taglib_tag_set_year(TagLib::ID3v2::Tag *tag, unsigned int year) {
  return tag->setYear(year);
}

TagLib::ID3v2::FrameFactory *taglib_id3v2_frame_factory_instance() {
  return TagLib::ID3v2::FrameFactory::instance();
}

void taglib_id3v2_frame_factory_set_default_text_encoding(TagLib::ID3v2::FrameFactory *factory, TagLib::String::Type encoding) {
  factory->setDefaultTextEncoding(encoding);
}

TagLib::ID3v2::AttachedPictureFrame *taglib_id3v2_attached_picture_frame_new() {
  return new TagLib::ID3v2::AttachedPictureFrame();
}

void taglib_id3v2_attached_picture_frame_set_mime_type(TagLib::ID3v2::AttachedPictureFrame *frame, const char *value) {
  TagLib::String mime_type = c_str_to_string(value);
  frame->setMimeType(mime_type);
}

void taglib_id3v2_attached_picture_frame_set_picture(TagLib::ID3v2::AttachedPictureFrame *frame, const char *data, unsigned int len) {
  TagLib::ByteVector picture(data, len);
  frame->setPicture(picture);
}

void taglib_id3v2_attached_picture_frame_set_type(TagLib::ID3v2::AttachedPictureFrame *frame, TagLib::ID3v2::AttachedPictureFrame::Type t) {
  frame->setType(t);
}

TagLib::ID3v2::TextIdentificationFrame *taglib_id3v2_text_identification_frame_new(const char *id, TagLib::String::Type encoding) {
  TagLib::ByteVector frame_id(id);
  return new TagLib::ID3v2::TextIdentificationFrame(frame_id, encoding);
}

void taglib_id3v2_text_identification_frame_set_text(TagLib::ID3v2::TextIdentificationFrame *frame, const char *value) {
  TagLib::String text = c_str_to_string(value);
  frame->setText(text);
}

}
