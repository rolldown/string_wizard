#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub dst_line: u32,
    pub dst_col: u32,
    pub src_line: u32,
    pub src_col: u32,
    pub source_id: Option<u32>,
    pub name_id: Option<u32>,
}

 
  #[derive(Debug, Clone, Default, PartialEq, Eq)]
  pub struct TokenChunk {
      pub start: u32,
      pub end: u32,
      pub prev_dst_line: u32,
      pub prev_dst_col: u32,
      pub prev_src_line: u32,
      pub prev_src_col: u32,
      pub prev_name_id: u32,
      pub prev_source_id: u32,
  }
