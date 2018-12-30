pub use crate::context::Context;

pub mod context;
pub mod readers;

use lazy_static::lazy_static;
use regex::Regex;
use unidecode::unidecode;

// See <https://github.com/rails/rails/blob/5-0-stable/activesupport/lib/active_support/inflector/transliterate.rb#L66-L111>.
pub fn parameterize(s: &str) -> String {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"(?i)[^a-z0-9-_]+").unwrap();
        static ref RE2: Regex = Regex::new(r"-{2,}").unwrap();
    }

    let s = unidecode(s);
    let s = RE1.replace_all(&s, "-");
    let s = RE2.replace_all(&s, "-");
    s.trim_matches('-').to_lowercase()
}
