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

#[derive(Debug, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn bump_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }
}

#[test]
fn basic() {
    use std::ops::Range;
    let source = "string\nwizard";
    let locator = Locator::new(source);
    let line_range = |line: usize| -> Range<usize> {
        assert!(line < locator.line_offsets.len());
        if line == locator.line_offsets.len() - 1 {
            locator.line_offsets[line]..source.len()
        } else {
            locator.line_offsets[line]..(locator.line_offsets[line + 1] - 1)
        }
    };
    assert_eq!(&source[line_range(0)], "string");
    assert_eq!(&source[line_range(1)], "wizard");

    assert_eq!(locator.line_offsets[0], 0);
    assert_eq!(locator.line_offsets[1], 7);

    assert_eq!(locator.locate(2), Location { line: 0, column: 2 });
    assert_eq!(locator.locate(8), Location { line: 1, column: 1 });
}
