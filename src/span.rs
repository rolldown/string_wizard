use crate::TextSize;

#[derive(Debug, Default, Clone, Copy)]
pub struct Span(pub TextSize, pub TextSize);

impl Span {
    pub fn start(&self) -> TextSize {
        self.0
    }

    pub fn end(&self) -> TextSize {
        self.1
    }

    pub fn text<'s>(&self, source: &'s str) -> &'s str {
        // This crate doesn't support usize which is u16 on 16-bit platforms.
        // So, we can safely cast TextSize/u32 to usize.
        &source[self.start() as usize..self.end() as usize]
    }
}
