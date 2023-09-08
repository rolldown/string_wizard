use crate::{MagicString, CowStr};

#[derive(Default)]
pub struct Joiner<'s> {
    sources: Vec<MagicString<'s>>,
}

impl<'s> Joiner<'s> {
    // --- public
    pub fn append(&mut self, source: MagicString<'s>) -> &mut Self {
        self.sources.push(source);
        self
    }

    pub fn append_raw(&mut self, raw: impl Into<CowStr<'s>>) -> &mut Self {
        self.sources.push(MagicString::new(raw));
        self
    }

    pub fn len(&self) -> usize {
        self.sources.iter().map(|s| s.len()).sum()
    }

    pub fn join(&self) -> String {
        let mut ret = String::with_capacity(self.len());
        self.fragments().for_each(|frag| {
            ret.push_str(frag);
        });
        ret
    }

    // --- private

    fn fragments(&'s self) -> impl Iterator<Item = &'s str> {
        self.sources.iter().flat_map(|c| c.fragments())
    }

    
}