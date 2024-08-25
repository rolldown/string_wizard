use index_vec::IndexVec;

use crate::chunk::{Chunk, ChunkIdx};

pub type IndexChunks<'text> = IndexVec<ChunkIdx, Chunk<'text>>;
