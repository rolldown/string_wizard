pub struct Locator {
    line_offsets: Box<[usize]>,
}

impl Locator {
    pub fn new(source: &str) -> Self {
        let mut line_offsets = vec![];
        let mut line_start_pos = 0;
        for line in source.lines() {
            line_offsets.push(line_start_pos);
            line_start_pos += 1 + line.len();
        }
        Self {
            line_offsets: line_offsets.into_boxed_slice(),
        }
    }

    pub fn locate(&self, index: usize) -> Location {
        let mut left_cursor = 0;
        let mut right_cursor = self.line_offsets.len();
        while left_cursor < right_cursor {
            let mid = (left_cursor + right_cursor) >> 1;
            if index < self.line_offsets[mid] {
                right_cursor = mid;
            } else {
                left_cursor = mid + 1;
            }
        }
        let line = left_cursor - 1;
        let column = index - self.line_offsets[line];
        Location { line, column }
    }
}

pub struct Location {
    pub line: usize,
    pub column: usize,
}
