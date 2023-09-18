use crate::{CowStr, MagicString, TextSize};

#[derive(Debug, Default, Clone)]
pub struct UpdateOptions {
    pub store_name: bool,
    pub overwrite: bool,
}

impl<'text> MagicString<'text> {
    pub fn update(&mut self, start: TextSize, end: TextSize, content: impl Into<CowStr<'text>>) -> &mut Self {
        self.update_with(start, end, content, Default::default())
    }

    pub fn update_with(
        &mut self,
        start: TextSize,
        end: TextSize,
        content: impl Into<CowStr<'text>>,
        opts: UpdateOptions,
    ) -> &mut Self {
        assert!(start < end);
        self.split_at(start);
        self.split_at(end);

        let first = self.chunk_by_start.get(&start).unwrap();
        let end = self.chunk_by_end.get(&start).unwrap();

        // let chunk_after_next =

        self.chunks[*first].edit(content.into(), opts.overwrite, opts.store_name);

        self
    }
}
