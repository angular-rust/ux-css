//! Sass reimplemented in rust with nom.
//!
//! Sass reimplemented in rust with nom (early stage).
//! The "r" in the name might stand for the Rust programming language,
//! for "re-implemented", or possibly for my name Rasmus.
//!
//! # Example
//!
//! ```
//! use ::css::{compile_scss_path, output};
//!
//! let path = "tests/basic/14_imports/a.scss".as_ref();
//! let format = output::Format {
//!     style: output::Style::Compressed,
//!     precision: 5,
//! };
//! let css = compile_scss_path(path, format).unwrap();
//!
//! assert_eq!(css, b"div span{moo:goo}\n")
//! ```
//!
//! # Sass language and implemetation status
//!
//! The sass language [is defined in its reference
//! doc](http://sass-lang.com/documentation/file.SASS_REFERENCE.html).
//! This implementation is incomplete but getting there, if slowly.
//!
//! Progress: 3456 of 5962 tests passed in dart-sass compatibility mode.
//!
//! If you want a working rust library for sass right now, you may
//! be better of with [sass-rs](https://crates.io/crates/sass-rs)
//! or [sass-alt](https://crates.io/crates/sass-alt),
//! which are rust wrappers around libsass.
//! Another alternative is [grass](https://crates.io/crates/grass)
//! which is another early stage pure rust implementation.
//! That said, this implementation has reached a version where I find it
//! usable for my personal projects, and the number of working tests are
//! improving.
#![forbid(unsafe_code)]
// TODO: #![forbid(missing_docs)]
use std::path::Path;

#[forbid(missing_docs)]
pub mod css;
mod error;
#[forbid(missing_docs)]
mod file_context;
#[forbid(missing_docs)]
mod ordermap;
#[forbid(missing_docs)]
pub mod output;
#[forbid(missing_docs)]
mod parser;
#[forbid(missing_docs)]
pub mod sass;
#[forbid(missing_docs)]
pub mod selectors;
#[forbid(missing_docs)]
pub mod value;
#[forbid(missing_docs)]
mod variablescope;

pub use error::SassError;
pub use file_context::{FileContext, FsFileContext};
use output::Format;
pub use parser::{
    parse_scss_data, parse_scss_file, parse_scss_path, parse_value_data,
    ParseError, SourceName, SourcePos,
};
pub use variablescope::{Scope, ScopeRef};

/// Parse a scss value from a buffer and write its css representation
/// in the given format.
///
/// # Example
///
/// ```
/// # use ::css::{compile_value, SassError};
/// # fn main() -> Result<(), SassError> {
/// assert_eq!(compile_value(b"10px + 4px", Default::default())?, b"14px");
/// assert_eq!(compile_value(b"10px 4px", Default::default())?, b"10px 4px");
/// # Ok(())
/// # }
/// ```
pub fn compile_value(input: &[u8], format: Format) -> Result<Vec<u8>, SassError> {
    let scope = ScopeRef::new_global(format);
    let value = parse_value_data(input)?.evaluate(scope)?;
    Ok(value.format(format).to_string().into_bytes())
}

/// Parse scss data from a buffer and write css in the given style.
///
/// # Example
///
/// ```
/// use ::css::compile_scss;
///
/// assert_eq!(
///     compile_scss(
///         b"foo {\
///         \n    bar {\
///         \n        baz:value;\
///         \n    }\
///         \n}",
///         Default::default(),
///     ).unwrap(),
///     b"foo bar {\
///     \n  baz: value;\
///     \n}\n"
/// )
/// ```
pub fn compile_scss(input: &[u8], format: Format) -> Result<Vec<u8>, SassError> {
    let file_context = FsFileContext::new();
    let items = parse_scss_data(input)?;
    format.write_root(&items, ScopeRef::new_global(format), &file_context)
}

/// Parse a file of scss data and write css in the given style.
///
/// Any `@import` directives will be handled relative to the directory
/// part of `file`.
///
/// # Example
///
/// ```
/// use ::css::{compile_scss_path, output::{Format, Style}};
///
/// assert_eq!(
///     compile_scss_path(
///         "tests/basic/14_imports/a.scss".as_ref(),
///         Format { style: Style::Compressed, precision: 5 },
///     ).unwrap(),
///     b"div span{moo:goo}\n"
/// )
/// ```
///
/// **Attention**: Previously, this function was named `compile_scss_file()`.
pub fn compile_scss_path(
    path: &Path,
    format: Format,
) -> Result<Vec<u8>, SassError> {
    let file_context = FsFileContext::new();
    let (sub_context, path) = file_context.file(path);
    let items = parse_scss_path(&path)?;
    format.write_root(&items, ScopeRef::new_global(format), &sub_context)
}
