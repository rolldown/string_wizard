use crate::{chunk::Chunk, locator::Locator, TextSize};
#[derive(Debug)]

pub struct Segment {
    /// [dst_column, source_index, src_line, src_column]
    /// - `dst_column` is calculated based on utf-16.
    /// - `src_column` is calculated based on utf-16.
    inner: [i64; 4],
    name_index: Option<i64>,
}

impl Segment {
    pub fn new(
        dst_column: i64,
        source_index: i64,
        src_line: i64,
        src_column: i64,
        name_index: Option<i64>,
    ) -> Self {
        Self {
            inner: [
                dst_column.into(),
                source_index.into(),
                src_line.into(),
                src_column.into(),
            ],
            name_index: name_index.map(|n| n.into()),
        }
    }

    pub fn dst_column(&self) -> i64 {
        self.inner[0]
    }

    pub fn source_index(&self) -> i64 {
        self.inner[1]
    }

    pub fn src_line(&self) -> i64 {
        self.inner[2]
    }

    pub fn src_column(&self) -> i64 {
        self.inner[3]
    }

    pub fn name_index(&self) -> Option<i64> {
        self.name_index
    }
}

#[derive(Debug)]
pub struct Mappings {
    generated_code_line: TextSize,
    /// `generated_code_column` is calculated based on utf-16.
    generated_code_column: TextSize,
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
        source_index: TextSize,
        source: &str,
        name_index: Option<TextSize>,
    ) {
        let mut loc = locator.locate(chunk.start());
        if let Some(edited_content) = &chunk.edited_content {
            if !edited_content.is_empty() {
                let segment = Segment::new(
                    self.generated_code_column.into(),
                    source_index.into(),
                    loc.line.into(),
                    loc.column.into(),
                    name_index.map(|n| n.into()),
                );
                self.add_segment_to_current_line(segment);
            }
            self.advance(edited_content);
        } else {
            let chunk_content = chunk.span.text(source);
            let mut new_line = true;
            for char in chunk_content.chars() {
                match char {
                    '\n' => {
                        loc.bump_line();
                        self.bump_line();
                        new_line = true;
                    }
                    _ => {
                        if new_line {
                            new_line = false;
                            let segment = Segment::new(
                                self.generated_code_column.into(),
                                source_index.into(),
                                loc.line.into(),
                                loc.column.into(),
                                None,
                            );
                            self.add_segment_to_current_line(segment);
                        }

                        let char_utf16_len = char.len_utf16() as u32;
                        loc.column += char_utf16_len;
                        self.generated_code_column += char_utf16_len;
                    }
                }
            }
        }
    }

    pub fn advance(&mut self, content: &str) {
        if content.is_empty() {
            return;
        }
        let mut lines = content.split('\n');

        // SAFETY: In any cases, lines would have at least one element.
        // "".split('\n') would create `[""]`.
        // "\n".split('\n') would create `["", ""]`.
        let last_line = unsafe { lines.next_back().unwrap_unchecked() };
        for _ in lines {
            self.bump_line();
        }
        self.generated_code_column += last_line.chars().map(|c| c.len_utf16() as u32).sum::<u32>();
    }

    fn add_segment_to_current_line(&mut self, seg: Segment) {
        self.raw[self.generated_code_line as usize].push(seg)
    }

    fn bump_line(&mut self) {
        self.generated_code_line += 1;
        self.generated_code_column = 0;
        self.raw.push(Default::default());
        debug_assert!(self.generated_code_line as usize == self.raw.len() - 1)
    }

    pub fn encoded(&self) -> String {
        let mut encoded_mappings = String::new();
        // [dst_column, source_index, src_line, src_column]
        let mut last_segment = [0i64, 0, 0, 0];
        let mut last_name_idx = 0i64;
        for (line_idx, line) in self.raw.iter().enumerate() {
            last_segment[0] = 0;
            for (segment_idx, segment) in line.iter().enumerate() {
                let diff = [
                    segment.dst_column() - last_segment[0],
                    segment.source_index() - last_segment[1],
                    segment.src_line() - last_segment[2],
                    segment.src_column() - last_segment[3],
                ];
                diff.into_iter().for_each(|diff| {
                    encode_vlq(diff, &mut encoded_mappings);
                });
                last_segment = segment.inner;

                if let Some(name_index) = segment.name_index() {
                    encode_vlq(name_index - last_name_idx, &mut encoded_mappings);
                    last_name_idx = name_index;
                }

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

pub(crate) fn encode_vlq(num: i64, out: &mut String) {
    let mut num = if num < 0 { ((-num) << 1) + 1 } else { num << 1 };

    loop {
        let mut digit = num & 0b11111;
        num >>= 5;
        if num > 0 {
            digit |= 1 << 5;
        }
        out.push(B64_CHARS[digit as usize] as char);
        if num == 0 {
            break;
        }
    }
}

const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
