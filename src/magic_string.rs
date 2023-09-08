use rustc_hash::FxHashMap;

use crate::{chunk::Chunk, span::Span, ChunkIdx, ChunkVec, CowStr};

#[derive(Debug, Default)]
pub struct MagicStringOptions {
    pub filename: Option<String>,
}

#[derive(Default)]
pub struct MagicString<'s> {
    source: CowStr<'s>,
    source_len: u32,
    chunks: ChunkVec<'s>,
    first_chunk: Option<ChunkIdx>,
    chunk_by_start: FxHashMap<u32, ChunkIdx>,
    chunk_by_end: FxHashMap<u32, ChunkIdx>,
    pub filename: Option<String>,
}

impl<'s> MagicString<'s> {
    // --- public

    pub fn new(source: impl Into<CowStr<'s>>) -> Self {
        Self::with_options(source, Default::default())
    }

    pub fn with_options(source: impl Into<CowStr<'s>>, options: MagicStringOptions) -> Self {
        let source = source.into();
        let source_len: u32 = source.len().try_into().expect("source is too big");
        let mut magic_string = Self {
            source,
            source_len,
            ..Default::default()
        };
        magic_string.split_at(source_len);
        magic_string.first_chunk = Some(ChunkIdx::from_raw(0));

        magic_string.filename = options.filename;

        magic_string
    }

    pub fn append(&mut self, source: impl Into<CowStr<'s>>) -> &mut Self {
        self.last_chunk_mut().append_outro(source.into());
        self
    }

    pub fn len(&self) -> usize {
        self.chunks.iter().map(|c| c.len()).sum()
    }

    pub fn to_string(&self) -> String {
        let size_hint = self.len();
        let mut ret = String::with_capacity(size_hint);
        self.fragments().for_each(|f| ret.push_str(f));
        ret
    }

    // --- private

    fn iter_chunks(&self) -> impl Iterator<Item = &Chunk> {
        ChunkIter {
            next: self.first_chunk,
            chunks: &self.chunks,
        }
    }

    pub(crate) fn fragments(&'s self) -> impl Iterator<Item = &'s str> {
        self.iter_chunks().flat_map(|c| c.fragments(&self.source))
    }

    /// a b c d e f g
    /// 0 1 2 3 4 5 6
    /// split_at(3)
    ///  a b c  d e f g
    /// [0 1 2][3 4 5 6]
    fn split_at(&mut self, idx: u32) {
        let source_len = self.source_len;
        debug_assert!(idx <= source_len);
        if self.chunks.is_empty() {
            let prev_span = Span(0, idx);
            self.create_chunk(prev_span);
            if idx < source_len {
                self.create_chunk(Span(idx, source_len));
            }
        } else {
        }
    }

    fn create_chunk(&mut self, span: Span) -> ChunkIdx {
        let idx = self.chunks.push(Chunk::new(span));
        self.chunk_by_start.insert(span.start(), idx);
        self.chunk_by_end.insert(span.end(), idx);
        idx
    }

    fn last_chunk_mut(&mut self) -> &mut Chunk<'s> {
        let idx = self.chunk_by_end.get(&(self.source_len)).unwrap();
        &mut self.chunks[*idx]
    }
}

struct ChunkIter<'a> {
    next: Option<ChunkIdx>,
    chunks: &'a ChunkVec<'a>,
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = &'a Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|next| {
            let chunk = &self.chunks[next];
            self.next = chunk.next;

            chunk
        })
    }
}
