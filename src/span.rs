#[derive(Debug, Default, Clone, Copy)]
pub struct Span(pub u32, pub u32);

impl Span {
    pub fn start(&self) -> u32 {
        self.0
    }

    pub fn end(&self) -> u32 {
        self.1
    }

    pub fn text<'s>(&self, source: &'s str) -> &'s str {
        &source[self.start() as usize..self.end() as usize]
    }

    pub fn size(&self) -> usize {
        (self.1 - self.0) as usize
    }
}
