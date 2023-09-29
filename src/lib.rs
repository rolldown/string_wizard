mod basic_types;
mod chunk;
mod joiner;
mod locator;
mod magic_string;
#[cfg(feature = "source_map")]
mod source_map;
mod span;

type CowStr<'text> = BasicCowStr<'text>;

pub(crate) type TextSize = u32;

use basic_types::BasicCowStr;

pub use crate::{
    joiner::{Joiner, JoinerOptions},
    magic_string::{mutation::UpdateOptions, MagicString, MagicStringOptions},
};

#[cfg(feature = "source_map")]
pub use crate::magic_string::source_map::SourceMapOptions;
