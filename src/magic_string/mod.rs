pub mod indent;
pub mod mutation;
#[cfg(feature = "source_map")]
pub mod source_map;

use std::collections::VecDeque;

use rustc_hash::FxHashMap;

use crate::{
    basic_types::AssertIntoU32,
    chunk::{Chunk, ChunkIdx, ChunkVec},
    span::Span,
    CowStr, TextSize,
};

#[derive(Debug, Default)]
pub struct MagicStringOptions {
    pub filename: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MagicString<'s> {
    pub filename: Option<String>,
    intro: VecDeque<CowStr<'s>>,
    outro: VecDeque<CowStr<'s>>,
    indent_str: Option<String>,
    source: CowStr<'s>,
    source_len: TextSize,
    chunks: ChunkVec<'s>,
    first_chunk_idx: ChunkIdx,
    last_chunk_idx: ChunkIdx,
    chunk_by_start: FxHashMap<TextSize, ChunkIdx>,
    chunk_by_end: FxHashMap<TextSize, ChunkIdx>,
}

impl<'text> MagicString<'text> {
    // --- public

    pub fn new(source: impl Into<CowStr<'text>>) -> Self {
        Self::with_options(source, Default::default())
    }

    pub fn with_options(source: impl Into<CowStr<'text>>, options: MagicStringOptions) -> Self {
        let source = source.into();
        let source_len = u32::try_from(source.len()).unwrap();
        let initial_chunk = Chunk::new(Span(0, source_len));
        let mut chunks = ChunkVec::with_capacity(1);
        let initial_chunk_idx = chunks.push(initial_chunk);
        let mut magic_string = Self {
            intro: Default::default(),
            outro: Default::default(),
            source,
            source_len,
            first_chunk_idx: initial_chunk_idx,
            last_chunk_idx: initial_chunk_idx,
            chunks,
            chunk_by_start: Default::default(),
            chunk_by_end: Default::default(),
            // setup options
            filename: options.filename,
            indent_str: None,
        };

        magic_string.chunk_by_start.insert(0, initial_chunk_idx);
        magic_string
            .chunk_by_end
            .insert(source_len, initial_chunk_idx);

        magic_string
    }

    pub fn append(&mut self, source: impl Into<CowStr<'text>>) -> &mut Self {
        self.append_outro(source.into());
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
    pub fn append_left(
        &mut self,
        text_index: impl AssertIntoU32,
        content: impl Into<CowStr<'text>>,
    ) -> &mut Self {
        match self.by_end_mut(text_index.assert_into_u32()) {
            Some(chunk) => {
                chunk.append_outro(content.into());
            }
            None => self.append_intro(content.into()),
        }
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
    pub fn append_right(
        &mut self,
        text_index: impl AssertIntoU32,
        content: impl Into<CowStr<'text>>,
    ) -> &mut Self {
        match self.by_start_mut(text_index.assert_into_u32()) {
            Some(chunk) => {
                chunk.append_intro(content.into());
            }
            None => self.append_outro(content.into()),
        }
        self
    }

    pub fn prepend(&mut self, source: impl Into<CowStr<'text>>) -> &mut Self {
        self.prepend_intro(source.into());
        self
    }

    pub fn prepend_left(
        &mut self,
        text_index: impl AssertIntoU32,
        content: impl Into<CowStr<'text>>,
    ) -> &mut Self {
        match self.by_end_mut(text_index.assert_into_u32()) {
            Some(chunk) => chunk.prepend_outro(content.into()),
            None => self.prepend_intro(content.into()),
        }
        self
    }

    pub fn prepend_right(
        &mut self,
        text_index: impl AssertIntoU32,
        content: impl Into<CowStr<'text>>,
    ) -> &mut Self {
        match self.by_start_mut(text_index.assert_into_u32()) {
            Some(chunk) => {
                chunk.prepend_intro(content.into());
            }
            None => self.prepend_outro(content.into()),
        }
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

    fn prepend_intro(&mut self, content: impl Into<CowStr<'text>>) {
        self.intro.push_front(content.into());
    }

    fn append_outro(&mut self, content: impl Into<CowStr<'text>>) {
        self.outro.push_back(content.into());
    }

    fn prepend_outro(&mut self, content: impl Into<CowStr<'text>>) {
        self.outro.push_front(content.into());
    }

    fn append_intro(&mut self, content: impl Into<CowStr<'text>>) {
        self.intro.push_back(content.into());
    }

    fn iter_chunks(&self) -> impl Iterator<Item = &Chunk> {
        ChunkIter {
            next: Some(self.first_chunk_idx),
            chunks: &self.chunks,
        }
    }

    pub(crate) fn fragments(&'text self) -> impl Iterator<Item = &'text str> {
        let intro = self.intro.iter().map(|s| s.as_ref());
        let outro = self.outro.iter().map(|s| s.as_ref());
        let chunks = self.iter_chunks().flat_map(|c| c.fragments(&self.source));
        intro.chain(chunks).chain(outro)
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
    fn split_at(&mut self, at_index: u32) {
        if at_index == 0 || at_index >= self.source_len || self.chunk_by_end.contains_key(&at_index)
        {
            return;
        }

        let (mut candidate, mut candidate_idx, search_right) =
            if (self.source_len - at_index) > at_index {
                (self.first_chunk(), self.first_chunk_idx, true)
            } else {
                (self.last_chunk(), self.last_chunk_idx, false)
            };

        while !candidate.contains(at_index) {
            let next_idx = if search_right {
                self.chunk_by_start[&candidate.end()]
            } else {
                self.chunk_by_end[&candidate.start()]
            };
            candidate = &self.chunks[next_idx];
            candidate_idx = next_idx;
        }

        let second_half_chunk = self.chunks[candidate_idx].split(at_index);
        let second_half_span = second_half_chunk.span;
        let second_half_idx = self.chunks.push(second_half_chunk);
        let first_half_idx = candidate_idx;

        // Update the chunk_by_start/end maps
        self.chunk_by_end.insert(at_index, first_half_idx);
        self.chunk_by_start.insert(at_index, second_half_idx);
        self.chunk_by_end
            .insert(second_half_span.end(), second_half_idx);

        // Make sure the new chunk and the old chunk have correct next/prev pointers
        self.chunks[second_half_idx].next = self.chunks[first_half_idx].next;
        if let Some(second_half_next_idx) = self.chunks[second_half_idx].next {
            self.chunks[second_half_next_idx].prev = Some(second_half_idx);
        }
        self.chunks[second_half_idx].prev = Some(first_half_idx);
        self.chunks[first_half_idx].next = Some(second_half_idx);
        if first_half_idx == self.last_chunk_idx {
            self.last_chunk_idx = second_half_idx
        }
    }

    fn by_start_mut(&mut self, text_index: impl AssertIntoU32) -> Option<&mut Chunk<'text>> {
        let text_index = text_index.assert_into_u32();
        if text_index == self.source_len {
            None
        } else {
            self.split_at(text_index);
            // TODO: safety: using `unwrap_unchecked` is fine.
            let idx = self.chunk_by_start.get(&text_index).unwrap();
            Some(&mut self.chunks[*idx])
        }
    }

    fn by_end_mut(&mut self, text_index: TextSize) -> Option<&mut Chunk<'text>> {
        let text_index = text_index.assert_into_u32();
        if text_index == 0 {
            None
        } else {
            self.split_at(text_index);
            // TODO: safety: using `unwrap_unchecked` is fine.
            let idx = self.chunk_by_end.get(&text_index).unwrap();
            Some(&mut self.chunks[*idx])
        }
    }

    fn last_chunk(&self) -> &Chunk<'text> {
        &self.chunks[self.last_chunk_idx]
    }

    fn first_chunk(&self) -> &Chunk<'text> {
        &self.chunks[self.first_chunk_idx]
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

