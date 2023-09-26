mod chunk;
mod decoded_map;
mod joiner;
mod locator;
mod magic_string;
mod mappings;
mod source_map;
mod span;

type CowStr<'text> = std::borrow::Cow<'text, str>;

pub(crate) type TextSize = usize;

pub use crate::{
    joiner::{Joiner, JoinerOptions},
    magic_string::{
        mutation::UpdateOptions, source_map::SourceMapOptions, MagicString, MagicStringOptions,
    },
};
