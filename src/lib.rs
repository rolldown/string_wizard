mod basic_types;
mod chunk;
mod joiner;
mod locator;
mod magic_string;
#[cfg(feature = "source_map")]
mod source_map;
mod span;

type CowStr<'text> = Cow<'text, str>;

pub(crate) type TextSize = u32;

use std::borrow::Cow;


pub use crate::{
    joiner::{Joiner, JoinerOptions},
    magic_string::{update::UpdateOptions, MagicString, MagicStringOptions, indent::IndentOptions},
};

#[cfg(feature = "source_map")]
pub use crate::magic_string::source_map::SourceMapOptions;
