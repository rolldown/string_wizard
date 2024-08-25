mod chunk;
mod joiner;
mod locator;
mod magic_string;
#[cfg(feature = "source_map")]
mod sourcemap_builder;
mod span;

type CowStr<'text> = Cow<'text, str>;

use std::borrow::Cow;

pub use crate::{
    joiner::{Joiner, JoinerOptions},
    magic_string::{indent::IndentOptions, update::UpdateOptions, MagicString, MagicStringOptions},
};

#[cfg(feature = "source_map")]
pub use crate::magic_string::source_map::SourceMapOptions;
