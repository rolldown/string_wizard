
mod chunk;
mod span;
mod joiner;
mod magic_string;
use std::borrow::Cow;

type CowStr<'s> = Cow<'s, str>;

use chunk::Chunk;
use index_vec::IndexVec;

pub use crate::{magic_string::{MagicString, MagicStringOptions}, joiner::Joiner};


index_vec::define_index_type! {
    struct ChunkIdx = u32;
}

type ChunkVec<'s> = IndexVec<ChunkIdx, Chunk<'s>>;

index_vec::define_index_type! {
    struct SourceIdx = u32;
}



pub struct Bundle {}
