use crate::{
    locator::Locator,
    MagicString,
    sourcemap_builder::SourcemapBuilder,
};

#[derive(Debug, Default)]
pub struct SourceMapOptions {
    pub include_content: bool,
}

impl<'s> MagicString<'s> {
    pub fn source_map(&self, opts: SourceMapOptions) -> sourcemap::SourceMap {
        let mut source_builder = SourcemapBuilder::new();

        source_builder.set_source("");
        if opts.include_content {
            source_builder.set_source_contents(&self.source);
        }

        let locator = Locator::new(&self.source);

        self.intro.iter().for_each(|frag| {
            source_builder.advance(frag);
        });

        self.iter_chunks().for_each(|chunk| {
            chunk.intro.iter().for_each(|frag| {
                source_builder.advance(frag);
            });

            let name = if chunk.keep_in_mappings && chunk.is_edited() {
                Some(chunk.span.text(&self.source))
            } else {
                None
            };

            source_builder.add_chunk(chunk, &locator, &self.source, name);

            chunk.outro.iter().for_each(|frag| {
                source_builder.advance(frag);
            });
        });

        source_builder.into_source_map()
    }
}
