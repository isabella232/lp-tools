#include <taglib/mpegfile.h>
#include <taglib/tag.h>

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

TagLib::Tag *taglib_file_tag(TagLib::MPEG::File *file) {
  return file->tag();
}

void taglib_tag_set_title(TagLib::Tag *tag, const char *value) {
  TagLib::String title = c_str_to_string(value);
  return tag->setTitle(title);
}

void taglib_tag_set_artist(TagLib::Tag *tag, const char *value) {
  TagLib::String artist = c_str_to_string(value);
  return tag->setArtist(artist);
}

void taglib_tag_set_album(TagLib::Tag *tag, const char *value) {
  TagLib::String album = c_str_to_string(value);
  return tag->setAlbum(album);
}

void taglib_tag_set_genre(TagLib::Tag *tag, const char *value) {
  TagLib::String genre = c_str_to_string(value);
  return tag->setGenre(genre);
}

void taglib_tag_set_year(TagLib::Tag *tag, unsigned int year) {
  return tag->setYear(year);
}

}
