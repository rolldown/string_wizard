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
