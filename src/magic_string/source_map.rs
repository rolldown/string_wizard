use std::sync::Arc;

use crate::{
  source_map::{locator::Locator, sourcemap_builder::SourcemapBuilder},
  MagicString,
};

use super::token::{Token, TokenChunk};

#[derive(Debug)]
pub struct SourceMapOptions {
  pub include_content: bool,
  pub source: Arc<str>,
  pub hires: bool,
}

impl Default for SourceMapOptions {
  fn default() -> Self {
    Self { include_content: false, source: "".into(), hires: false }
  }
}


#[derive(Debug, Clone, Default)]
pub struct SourceMap {
  pub  file: Option<Arc<str>>,
  pub  names: Vec<Arc<str>>,
  pub  source_root: Option<String>,
  pub  sources: Vec<Arc<str>>,
  pub  source_contents: Option<Vec<Arc<str>>>,
  pub  tokens: Vec<Token>,
  pub  token_chunks: Option<Vec<TokenChunk>>,
  pub  x_google_ignore_list: Option<Vec<u32>>,
}

impl SourceMap {

   pub  fn from_oxc_sourcemap(source_map: oxc_sourcemap::SourceMap) -> Self {
        Self {
          file:source_map.get_file().map(Into::into), 
          names:source_map.get_names().map(Into::into).collect::<Vec<_>>(), 
          source_root:source_map.get_source_root().map(str::to_string),
           sources:source_map.get_sources().map(Into::into).collect::<Vec<_>>(),
            source_contents:source_map.get_source_contents().map(|x| x.map(Into::into).collect::<Vec<_>>()),
            tokens: source_map.get_tokens()
                              .map( |token|
                                Token::new(
                                token.get_dst_line(),
                                token.get_dst_col(),
                                 token.get_src_line(),
                                token.get_src_col(),
                                token.get_source_id(),
                                token.get_name_id()
                                ) ).collect::<Vec<_>>(), 
            token_chunks: None,
            x_google_ignore_list:  None}
    }

    pub fn get_file(&self)-> Option<&str> {
      self.file.as_deref()

    }

    pub fn get_names(&self) -> impl Iterator<Item = &str> {
      self.names.iter().map(AsRef::as_ref)

    }

    pub fn get_source_root(&self) -> Option<&str> {
      self.source_root.as_deref()
  }

    pub fn get_sources(&self) -> impl Iterator<Item = &str> {
      self.sources.iter().map(AsRef::as_ref)
    }

    pub fn get_source_contents(&self) ->  Option<impl Iterator<Item = &str>> {
      self.source_contents.as_ref().map(|v| v.iter().map(AsRef::as_ref))

    }

    pub fn get_tokens(&self) -> impl Iterator<Item = &Token> {
      self.tokens.iter()
  }

}

impl<'s> MagicString<'s> {
  pub fn source_map(&self, opts: SourceMapOptions) -> oxc_sourcemap::SourceMap {
    let mut source_builder = SourcemapBuilder::new(opts.hires);

    source_builder.set_source_and_content(&opts.source, &self.source);

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

  pub fn create_own_sourcemap(&self,opts: SourceMapOptions) -> SourceMap {
    let oxc_source_map = self.source_map(opts);
    SourceMap::from_oxc_sourcemap(oxc_source_map)
  } 
}


impl Token {
  pub fn new(
      dst_line: u32,
      dst_col: u32,
      src_line: u32,
      src_col: u32,
      source_id: Option<u32>,
      name_id: Option<u32>,
  ) -> Self {
      Self { dst_line, dst_col, src_line, src_col, source_id, name_id }
  }

}   