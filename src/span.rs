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
        &source[self.start()..self.end()]
    }
}
