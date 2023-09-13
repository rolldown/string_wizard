use rustc_hash::FxHashMap;

use crate::{chunk::Chunk, span::Span, ChunkIdx, ChunkVec, CowStr};

#[derive(Debug, Default)]
pub struct MagicStringOptions {
    pub filename: Option<String>,
}

pub struct MagicString<'s> {
    pub filename: Option<String>,

    source: CowStr<'s>,
    source_len: u32,
    chunks: ChunkVec<'s>,
    first_chunk_idx: ChunkIdx,
    last_chunk_idx: ChunkIdx,
    chunk_by_start: FxHashMap<u32, ChunkIdx>,
    chunk_by_end: FxHashMap<u32, ChunkIdx>,
}

impl<'s> MagicString<'s> {
    // --- public

    pub fn new(source: impl Into<CowStr<'s>>) -> Self {
        Self::with_options(source, Default::default())
    }

    pub fn with_options(source: impl Into<CowStr<'s>>, options: MagicStringOptions) -> Self {
        let source = source.into();
        let source_len: u32 = source.len().try_into().expect("source is too big");
        let initial_chunk = Chunk::new(Span(0, source_len));
        let mut chunks = ChunkVec::with_capacity(1);
        let initial_chunk_idx = chunks.push(initial_chunk);
        let mut magic_string = Self {
            source,
            source_len,
            first_chunk_idx: initial_chunk_idx,
            last_chunk_idx: initial_chunk_idx,
            chunks,
            chunk_by_start: Default::default(),
            chunk_by_end: Default::default(),
            // setup options
            filename: options.filename,
        };

        magic_string.chunk_by_start.insert(0, initial_chunk_idx);
        magic_string
            .chunk_by_end
            .insert(source_len, initial_chunk_idx);

        magic_string
    }

    pub fn append(&mut self, source: impl Into<CowStr<'s>>) -> &mut Self {
        self.last_chunk_mut().append_outro(source.into());
        self
    }

    /// # Example
    ///```rust
    /// use string_wizard::MagicString;
    /// let mut s = MagicString::new("01234");
    /// s.append_left(2, "a");
    /// s.append_left(2, "b");
    /// assert_eq!(s.to_string(), "01ab234")
    ///```
    pub fn append_left(&mut self, text_index: u32, source: impl Into<CowStr<'s>>) -> &mut Self {
        self.split_at(text_index);
        let idx = self.chunk_by_end.get(&text_index).unwrap();
        let chunk = &mut self.chunks[*idx];
        chunk.append_outro(source.into());
        self
    }

    /// # Example
    /// ```rust
    /// use string_wizard::MagicString;
    /// let mut s = MagicString::new("01234");
    /// s.append_right(2, "A");
    /// s.append_right(2, "B");
    /// s.append_left(2, "a");
    /// s.append_left(2, "b");
    /// assert_eq!(s.to_string(), "01abAB234")
    ///```
    pub fn append_right(&mut self, text_index: u32, source: impl Into<CowStr<'s>>) -> &mut Self {
        self.split_at(text_index);
        let idx = self.chunk_by_start.get(&text_index).unwrap();
        let chunk = &mut self.chunks[*idx];
        chunk.append_intro(source.into());
        self
    }

    pub fn prepend(&mut self, source: impl Into<CowStr<'s>>) -> &mut Self {
        self.first_chunk_mut().prepend_intro(source.into());
        self
    }

    pub fn prepend_left(&mut self, text_index: u32, content: impl Into<CowStr<'s>>) -> &mut Self {
        self.split_at(text_index);
        let idx = self.chunk_by_end.get(&text_index).unwrap();
        let chunk = &mut self.chunks[*idx];
        chunk.prepend_outro(content.into());
        self
    }

    pub fn prepend_right(&mut self, text_index: u32, content: impl Into<CowStr<'s>>) -> &mut Self {
        self.split_at(text_index);
        let idx = self.chunk_by_start.get(&text_index).unwrap();
        let chunk = &mut self.chunks[*idx];
        chunk.prepend_intro(content.into());
        self
    }

    pub fn len(&self) -> usize {
        self.fragments().map(|f| f.len()).sum()
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
            next: Some(self.first_chunk_idx),
            chunks: &self.chunks,
        }
    }

    pub(crate) fn fragments(&'s self) -> impl Iterator<Item = &'s str> {
        self.iter_chunks().flat_map(|c| c.fragments(&self.source))
    }

    /// For input
    /// "abcdefg"
    ///  0123456
    ///
    /// Chunk{span: (0, 7)} => "abcdefg"
    ///
    /// split_at(3) would create
    ///
    /// Chunk{span: (0, 3)} => "abc"
    /// Chunk{span: (3, 7)} => "defg"
    fn split_at(&mut self, text_index: u32) {
        debug_assert!(text_index <= self.source_len);

        if self.chunk_by_start.contains_key(&text_index)
            || self.chunk_by_end.contains_key(&text_index)
        {
            return;
        }

        let mut target = &self.chunks[self.first_chunk_idx];
        let mut target_idx = self.first_chunk_idx;

        let search_right = text_index > target.end();

        while !target.contains(text_index) {
            let next_idx = if search_right {
                self.chunk_by_start.get(&target.end()).unwrap()
            } else {
                self.chunk_by_end.get(&target.start()).unwrap()
            };
            target = &self.chunks[*next_idx];
            target_idx = *next_idx;
        }

        let chunk_contains_index = &mut self.chunks[target_idx];

        let new_chunk = chunk_contains_index.split(text_index);
        self.chunk_by_end.insert(text_index, target_idx);
        let new_chunk_end = new_chunk.end();
        let new_chunk_idx = self.chunks.push(new_chunk);
        self.chunk_by_start.insert(text_index, new_chunk_idx);
        self.chunk_by_end.insert(new_chunk_end, new_chunk_idx);

        let chunk_contains_index = &mut self.chunks[target_idx];
        if target_idx == self.last_chunk_idx {
            self.last_chunk_idx = new_chunk_idx
        }
        chunk_contains_index.next = Some(new_chunk_idx);
    }

    fn last_chunk_mut(&mut self) -> &mut Chunk<'s> {
        &mut self.chunks[self.last_chunk_idx]
    }

    fn first_chunk_mut(&mut self) -> &mut Chunk<'s> {
        &mut self.chunks[self.first_chunk_idx]
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
