use crate::{CowStr, MagicString, SourceMapOptions};
use crate::source_map::decoded_map::DecodedMap;
use crate::source_map::mappings::Mappings;
use crate::source_map::SourceMap;

pub struct JoinerOptions {
    pub separator: Option<String>,
}

#[derive(Default)]
pub struct Joiner<'s> {
    sources: Vec<MagicString<'s>>,
    separator: Option<String>,
}

impl<'s> Joiner<'s> {
    // --- public
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(options: JoinerOptions) -> Self {
        Self {
            separator: options.separator,
            ..Default::default()
        }
    }

    pub fn append(&mut self, source: MagicString<'s>) -> &mut Self {
        self.sources.push(source);
        self
    }

    pub fn append_raw(&mut self, raw: impl Into<CowStr<'s>>) -> &mut Self {
        self.sources.push(MagicString::new(raw));
        self
    }

    pub fn len(&self) -> usize {
        self.fragments().map(|s| s.len()).sum()
    }

    pub fn join(&self) -> String {
        let mut ret = String::with_capacity(self.len());
        self.fragments().for_each(|frag| {
            ret.push_str(frag);
        });
        ret
    }

    pub fn source_map(&'s self, opts: SourceMapOptions) -> SourceMap {
        let decoded_map = self.generate_source_map(opts);
        decoded_map.into_source_map()
    }

    // --- private

    fn fragments(&'s self) -> impl Iterator<Item = &'s str> {
        let mut iter = self
            .sources
            .iter()
            .flat_map(|c| self.separator.as_deref().into_iter().chain(c.fragments()));
        // Drop the first separator
        if self.separator.is_some() {
            iter.next();
        }
        iter
    }

    fn generate_source_map(&'s self, opts: SourceMapOptions) -> DecodedMap {
        let mut mappings = Mappings::new();
        let mut names = vec![];

        let separator = self.separator.as_deref().unwrap_or("\n");

        for (index, source) in self.sources.iter().enumerate() {
            if index > 0 {
                mappings.advance(&separator);
            }

            let name = source.source_map_to_mapping(&mut mappings);

            names.extend(name.iter().cloned());
        }

        // TODO: need uniqueSources https://github.com/Rich-Harris/magic-string/blob/6f6cd52270fdc8b62b1b94c73a5d19ba37b3d4dd/src/Bundle.js#L156
        DecodedMap {
            version: 3,
            sources: self.sources.iter()
                .filter_map(|source| source.filename.clone())
                .collect(),
            sources_content: self.sources.iter()
                .map(|source| {
                    opts
                        .include_content
                        .then(|| source.to_string())
                        .unwrap_or_default()
                })
                .collect(),
            mappings,
            names,
        }
    }
}