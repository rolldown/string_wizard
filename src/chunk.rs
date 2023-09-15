use std::{collections::VecDeque, borrow::Cow};

use crate::{span::Span, ChunkIdx, CowStr};

#[derive(Debug, Default)]
pub struct Chunk<'str> {
    pub intro: VecDeque<CowStr<'str>>,
    pub outro: VecDeque<CowStr<'str>>,
    span: Span,
    content: Option<CowStr<'str>>,
    pub(crate) next: Option<ChunkIdx>,
}

impl<'s> Chunk<'s> {
    pub fn new(span: Span) -> Self {
        Self {
            span,
            ..Default::default()
        }
    }
}

impl<'str> Chunk<'str> {
    pub fn start(&self) -> u32 {
        self.span.start()
    }

    pub fn end(&self) -> u32 {
        self.span.end()
    }

    pub fn contains(&self, text_index: u32) -> bool {
        self.start() < text_index && text_index < self.end()
    }

    pub fn append_outro(&mut self, content: CowStr<'str>) {
        self.outro.push_back(content)
    }

    pub fn append_intro(&mut self, content: CowStr<'str>) {
        self.intro.push_back(content)
    }

    pub fn prepend_outro(&mut self, content: CowStr<'str>) {
        self.outro.push_front(content)
    }

    pub fn prepend_intro(&mut self, content: CowStr<'str>) {
        self.intro.push_front(content)
    }

    pub fn split<'a,>(&'a mut self, text_index: u32)  -> Chunk<'str> {
        let first_slice_span = Span(self.start(), text_index);
        let last_slice_span = Span(text_index, self.end());
        let mut new_chunk = Chunk::new(last_slice_span);
        std::mem::swap(&mut new_chunk.outro, &mut self.outro);
        self.span = first_slice_span;
        new_chunk.next = self.next;
        new_chunk
    }

    pub fn fragments(&'str self, original_source: &'str CowStr<'str>) -> impl Iterator<Item = &'str str> {
        let intro_iter = self.intro.iter().map(|frag| frag.as_ref());
        let source_frag = self.content.as_deref().unwrap_or_else(|| self.span.text(original_source.as_ref()));
        let outro_iter = self.outro.iter().map(|frag| frag.as_ref());
        intro_iter.chain(Some(source_frag)).chain(outro_iter)
    }

    pub fn edit(&mut self, content: CowStr<'str>) {
        self.content = Some(content);
    }

    pub fn is_edited(&self) -> bool {
        self.content.is_some()
    }
}

