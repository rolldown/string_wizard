use crate::{
    locator::Locator,
    source_map::{decoded_map::DecodedMap, mappings::Mappings, SourceMap},
    MagicString,
};

#[derive(Debug, Default)]
pub struct SourceMapOptions {
    pub include_content: bool,
}

impl<'s> MagicString<'s> {
    fn generate_decoded_source_map(&self, opts: SourceMapOptions) -> DecodedMap {
        let mut mappings = Mappings::new();
        let locator = Locator::new(&self.source);

        self.intro.iter().for_each(|frag| {
            mappings.advance(frag);
        });

        let mut names = vec![];

        self.iter_chunks().for_each(|chunk| {
            chunk.intro.iter().for_each(|frag| {
                mappings.advance(frag);
            });

            let name_idx = if chunk.keep_in_mappings && chunk.is_edited() {
                let original_content = chunk.span.text(&self.source);

                let idx = names
                    .iter()
                    .enumerate()
                    .find_map(|(idx, name)| (name == original_content).then_some(idx))
                    .unwrap_or_else(|| {
                        let next_idx = names.len();
                        names.push(original_content.to_string());
                        next_idx
                    });
                debug_assert!(idx < names.len());
                Some(idx as u32)
            } else {
                None
            };

            mappings.add_chunk(chunk, &locator, 0, &self.source, name_idx);

            chunk.outro.iter().for_each(|frag| {
                mappings.advance(frag);
            });
        });

        DecodedMap {
            version: 3,
            sources: vec!["".to_string()],
            sources_content: opts
                .include_content
                .then(|| vec![self.source.as_ref().to_string()])
                .unwrap_or_default(),
            mappings,
            names,
        }
    }

    pub fn source_map_to_mapping(&self, mut mappings: &mut Mappings) -> Vec<String> {
        let locator = Locator::new(&self.source);

        self.intro.iter().for_each(|frag| {
            mappings.advance(frag);
        });

        let mut names = vec![];

        self.iter_chunks().for_each(|chunk| {
            chunk.intro.iter().for_each(|frag| {
                mappings.advance(frag);
            });

            let original_content = chunk.span.text(&self.source);

            if self.filename.is_some() {
                let name_idx = if chunk.keep_in_mappings && chunk.is_edited() {
                    let idx = names
                        .iter()
                        .enumerate()
                        .find_map(|(idx, name)| (name == original_content).then_some(idx))
                        .unwrap_or_else(|| {
                            let next_idx = names.len();
                            names.push(original_content.to_string());
                            next_idx
                        });
                    debug_assert!(idx < names.len());
                    Some(idx as u32)
                } else {
                    None
                };

                mappings.add_chunk(chunk, &locator, 0, &self.source, name_idx);
            } else {
                mappings.advance(original_content);
            }

            chunk.outro.iter().for_each(|frag| {
                mappings.advance(frag);
            });
        });

        names
    }

    pub fn source_map(&self, opts: SourceMapOptions) -> SourceMap {
        let decoded_map = self.generate_decoded_source_map(opts);
        decoded_map.into_source_map()
    }
}
