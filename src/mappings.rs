use crate::{chunk::Chunk, locator::Locator};

#[derive(Debug)]

pub struct Segment {
    // column in the generated code
    pub column: usize,
    pub source_index: usize,
    pub original_line: usize,
    pub original_column: usize,
    pub name_index: Option<usize>,
}

#[derive(Debug)]
pub struct Mappings {
    generated_code_line: usize,
    generated_code_column: usize,
    raw: Vec<Vec<Segment>>,
}

impl Mappings {
    pub fn new() -> Self {
        Self {
            generated_code_line: 0,
            generated_code_column: 0,
            raw: vec![vec![]],
        }
    }

    pub fn add_chunk(
        &mut self,
        chunk: &Chunk,
        locator: &Locator,
        source_index: usize,
        source: &str,
        name_index: Option<usize>,
    ) {
        let mut loc = locator.locate(chunk.start());
        if let Some(edited_content) = &chunk.edited_content {
            if !edited_content.is_empty() {
                let segment = Segment {
                    column: self.generated_code_column,
                    source_index,
                    original_line: loc.line,
                    original_column: loc.column,
                    name_index,
                };
                self.add_segment_to_current_line(segment);
            }
            self.advance(edited_content);
        } else {
            let chunk_content = chunk.span.text(source);
            let mut new_line = true;
            for char in chunk_content.chars() {
                if new_line {
                    new_line = false;
                    let segment = Segment {
                        column: self.generated_code_column,
                        source_index,
                        original_line: loc.line,
                        original_column: loc.column,
                        name_index: None,
                    };
                    self.add_segment_to_current_line(segment);
                }
                match char {
                    '\n' => {
                        loc.bump_line();
                        self.bump_line();
                        new_line = true;
                    }
                    _ => {
                        let char_len = char.len_utf8();
                        loc.column += char_len;
                        self.generated_code_column += char.len_utf8();
                    }
                }
            }
        }
    }

    pub fn advance(&mut self, content: &str) {
        if content.is_empty() {
            return;
        }
        let mut lines = content.lines();
        // SAFETY: The content at least has one line after checking of `content.is_empty()` .
        // `"\n".lines().collect::<Vec<_>>()` would create `[""]`.
        let last_line = unsafe { lines.next_back().unwrap_unchecked() };
        for _ in lines {
            self.bump_line();
        }
        self.generated_code_column += last_line.len();
    }

    fn add_segment_to_current_line(&mut self, seg: Segment) {
        self.raw[self.generated_code_line].push(seg)
    }

    fn bump_line(&mut self) {
        self.generated_code_line += 1;
        self.generated_code_column = 0;
        self.raw.push(Default::default());
        debug_assert!(self.generated_code_line == self.raw.len() - 1)
    }

    pub fn encoded(&self) -> String {
        let mut encoded_mappings = String::new();
        for (line_idx, line) in self.raw.iter().enumerate() {
            for (segment_idx, segment) in line.iter().enumerate() {
                let mut encoded = vec![];
                vlq::encode(segment.column as i64, &mut encoded).unwrap();
                vlq::encode(segment.source_index as i64, &mut encoded).unwrap();
                vlq::encode(segment.original_line as i64, &mut encoded).unwrap();
                vlq::encode(segment.original_column as i64, &mut encoded).unwrap();
                // if let Some(name_index) = segment.name_index {
                //     vlq::encode(name_index as i64, &mut encoded).unwrap();
                // }
                encoded_mappings.push_str(&String::from_utf8(encoded).unwrap());
                if segment_idx != line.len() - 1 {
                    encoded_mappings.push(',');
                }
            }
            if line_idx != self.raw.len() - 1 {
                encoded_mappings.push(';');
            }
        }

        encoded_mappings
    }
}
