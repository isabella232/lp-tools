#![allow(proc_macro_derive_resolution_fallback)]

// increase limit for `infer_schema!`
#![recursion_limit="128"]

#[macro_use] extern crate diesel;

pub use self::duration::Duration;
pub use self::partial_date::PartialDate;

pub mod db;
pub mod duration;
pub mod graphql;
pub mod models;
pub mod partial_date;
pub mod repositories;
pub mod schema;
