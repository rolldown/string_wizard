mod chunk;
mod joiner;
mod locator;
mod magic_string;
mod mappings;
mod source_map;
mod span;
mod decoded_map;

use std::borrow::Cow;

type CowStr<'s> = Cow<'s, str>;

use chunk::Chunk;
use index_vec::IndexVec;

pub use crate::{
    joiner::{Joiner, JoinerOptions},
    magic_string::{
        mutation::UpdateOptions, source_map::SourceMapOptions, MagicString, MagicStringOptions,
    },
};

index_vec::define_index_type! {
    struct ChunkIdx = u32;
}

type ChunkVec<'s> = IndexVec<ChunkIdx, Chunk<'s>>;

index_vec::define_index_type! {
    struct SourceIdx = u32;
}

pub(crate) type TextSize = usize;
