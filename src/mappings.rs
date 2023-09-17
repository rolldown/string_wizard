

#[derive(Debug, Default)]
pub struct Mappings {
    generated_code_line: usize,
    generated_code_column: usize,
}

impl Mappings {
    pub fn advance(&mut self, content: &str) {
        if content.is_empty() {
            return;
        }
        let mut lines = content.lines();
        // SAFETY: The content at least has one line after checking of `content.is_empty()` .
        // `"\n".lines().collect::<Vec<_>>()` would create `[""]`.
        let last_line = unsafe { lines.next_back().unwrap_unchecked() };
        for _ in lines {
            self.generated_code_line += 1;
            self.generated_code_column = 0;
        }
        self.generated_code_column += last_line.len();
    }
}
