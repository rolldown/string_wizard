mod chunk;
mod decoded_map;
mod joiner;
mod locator;
mod magic_string;
mod mappings;
mod source_map;
mod span;
mod basic_types;

type CowStr<'text> = BasicCowStr<'text>;

pub(crate) type TextSize = u32;

use basic_types::BasicCowStr;

pub use crate::{
    joiner::{Joiner, JoinerOptions},
    magic_string::{
        mutation::UpdateOptions, source_map::SourceMapOptions, MagicString, MagicStringOptions,
    },
};
