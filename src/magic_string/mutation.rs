use crate::{basic_types::AssertIntoU32, chunk::EditOptions, CowStr, MagicString};

#[derive(Debug, Default, Clone)]
pub struct UpdateOptions {
    /// `true` will store the original content in the `name` field of the generated sourcemap.
    pub keep_original: bool,

    /// `true` will clear the `intro` and `outro` for the corresponding range.
    pub overwrite: bool,
}

impl<'text> MagicString<'text> {
    /// A shorthand for `update_with(start, end, content, Default::default())`;
    pub fn update(
        &mut self,
        start: impl AssertIntoU32,
        end: impl AssertIntoU32,
        content: impl Into<CowStr<'text>>,
    ) -> &mut Self {
        self.update_with(start, end, content, Default::default())
    }

    pub fn update_with(
        &mut self,
        start: impl AssertIntoU32,
        end: impl AssertIntoU32,
        content: impl Into<CowStr<'text>>,
        opts: UpdateOptions,
    ) -> &mut Self {
        self.update_with_inner(
            start.assert_into_u32(),
            end.assert_into_u32(),
            content.into(),
            opts,
            true,
        );
        self
    }

    pub fn remove(&mut self, start: impl AssertIntoU32, end: impl AssertIntoU32) -> &mut Self {
        self.update_with_inner(
            start.assert_into_u32(),
            end.assert_into_u32(),
            "".into(),
            UpdateOptions {
                keep_original: false,
                overwrite: true,
            },
            false,
        );

        self
    }

    /// Moves the characters from start and end to index. Returns this.
    // `move` is reserved keyword in rust, so we use `relocate` instead.
    pub fn relocate(
        &mut self,
        start: impl AssertIntoU32,
        end: impl AssertIntoU32,
        to: impl AssertIntoU32,
    ) -> &mut Self {
        let start = start.assert_into_u32();
        let end = end.assert_into_u32();
        let to = to.assert_into_u32();

        self.split_at(start);
        self.split_at(end);
        self.split_at(to);

        let first_idx = self.chunk_by_start[&start];
        let last_idx = self.chunk_by_end[&end];

        let old_left_idx = self.chunks[first_idx].prev;
        let old_right_idx = self.chunks[last_idx].next;

        let new_right_idx = self.chunk_by_start.get(&to).copied();

        // `new_right_idx` is `None` means that the `to` index is at the end of the string.
        // Moving chunks which contain the last chunk to the end is meaningless.
        if new_right_idx.is_none() && last_idx == self.last_chunk_idx {
            return self;
        }

        let new_left_idx = new_right_idx
            .map(|idx| self.chunks[idx].prev)
            // If the `to` index is at the end of the string, then the `new_right_idx` will be `None`.
            // In this case, we want to use the last chunk as the left chunk to connect the relocated chunk.
            .unwrap_or(Some(self.last_chunk_idx));

        // Adjust next/prev pointers, this remove the [start, end] range from the old position
        if let Some(old_left_idx) = old_left_idx {
            self.chunks[old_left_idx].next = old_right_idx;
        }
        if let Some(old_right_idx) = old_right_idx {
            self.chunks[old_right_idx].prev = old_left_idx;
        }

        if let Some(new_left_idx) = new_left_idx {
            self.chunks[new_left_idx].next = Some(first_idx);
        }
        if let Some(new_right_idx) = new_right_idx {
            self.chunks[new_right_idx].prev = Some(last_idx);
        }

        if self.chunks[first_idx].prev.is_none() {
            // If the `first_idx` is the first chunk, then we need to update the `first_chunk_idx`.
            self.first_chunk_idx = self.chunks[last_idx].next.unwrap();
        }
        if self.chunks[last_idx].next.is_none() {
            // If the `last_idx` is the last chunk, then we need to update the `last_chunk_idx`.
            self.last_chunk_idx = self.chunks[first_idx].prev.unwrap();
            self.chunks[last_idx].next = None;
        }
    
        if new_left_idx.is_none() {
            self.first_chunk_idx = first_idx;
        }
        if new_right_idx.is_none() {
            self.last_chunk_idx = last_idx;
        }

        self.chunks[first_idx].prev = new_left_idx;
        self.chunks[last_idx].next = new_right_idx;


        self
    }

    // --- private

    fn update_with_inner(
        &mut self,
        start: u32,
        end: u32,
        content: CowStr<'text>,
        opts: UpdateOptions,
        panic_if_start_equal_end: bool,
    ) -> &mut Self {
        let start = start as u32;
        let end = end as u32;
        if panic_if_start_equal_end && start == end {
            panic!(
                "Cannot overwrite a zero-length range – use append_left or prepend_right instead"
            )
        }
        assert!(start < end);
        self.split_at(start);
        self.split_at(end);

        let start_idx = self.chunk_by_start.get(&start).copied().unwrap();
        let end_idx = self.chunk_by_end.get(&end).copied().unwrap();

        let start_chunk = &mut self.chunks[start_idx];
        start_chunk.edit(
            content.into(),
            EditOptions {
                overwrite: opts.overwrite,
                store_name: opts.keep_original,
            },
        );

        let mut rest_chunk_idx = if start_idx != end_idx {
            start_chunk.next.unwrap()
        } else {
            return self;
        };

        while rest_chunk_idx != end_idx {
            let rest_chunk = &mut self.chunks[rest_chunk_idx];
            rest_chunk.edit("".into(), Default::default());
            rest_chunk_idx = rest_chunk.next.unwrap();
        }
        self
    }
}
