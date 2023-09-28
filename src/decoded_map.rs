use crate::{mappings::Mappings, source_map::SourceMap};

#[derive(Debug)]
pub struct DecodedMap {
    pub version: u8,
    pub sources: Vec<String>,
    pub sources_content: Vec<String>,
    pub mappings: Mappings,
    pub names: Vec<String>,
}

impl DecodedMap {
    pub fn into_source_map(self) -> SourceMap {
        SourceMap {
            version: self.version,
            sources: self.sources,
            sources_content: self.sources_content,
            mappings: self.mappings.encoded(),
            names: self.names,
        }
    }
}
