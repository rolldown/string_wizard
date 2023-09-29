// Notice that source map is designed based on utf-16 index, while rust [String] is based on utf-8 index.

pub mod decoded_map;
pub mod mappings;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceMap {
    pub version: u8,
    pub sources: Vec<String>,
    pub sources_content: Vec<String>,
    pub mappings: String,
    pub names: Vec<String>,
}

impl SourceMap {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
