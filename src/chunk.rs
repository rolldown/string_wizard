use crate::{CowStr, span::Span, ChunkIdx};

#[derive(Debug, Default)]
pub struct Chunk<'s> {
    intro: Vec<CowStr<'s>>,
    span: Span,
    outro: Vec<CowStr<'s>>,
    len: usize,
    pub(crate) next: Option<ChunkIdx>
}

impl<'s> Chunk<'s> {
    pub fn new(span: Span) -> Self {
        Self {
            len: span.size(),
            span,
            ..Default::default()
        }
    }
}

impl<'s> Chunk<'s> {
    pub fn fragments(&'s self, original_source: &'s CowStr<'s>) -> impl Iterator<Item = &'s str> {
        let intro_iter = self.intro.iter().map(|frag| frag.as_ref());
        let source_frag = self.span.text(original_source.as_ref());
        let outro_iter = self.outro.iter().map(|frag| frag.as_ref());
        intro_iter.chain(Some(source_frag)).chain(outro_iter)
    }

    pub fn append(&mut self, frag: CowStr<'s>) {
        self.len += frag.len();
        self.outro.push(frag.into())
    }

    pub fn prepend(&mut self, frag: CowStr<'s>) {
        self.len += frag.len();
        self.intro.push(frag.into())
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
