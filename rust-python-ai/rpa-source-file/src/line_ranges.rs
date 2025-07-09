use crate::find_newline;
use memchr::{memchr2, memrchr2};
use rpa_text_size::{TextLen, TextRange, TextSize};
use std::ops::Add;

pub trait LineRanges {
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    ///
    /// ## Panics
    fn line_start(&self, offset: TextSize) -> TextSize;

    fn bom_start_offset(&self) -> TextSize;

    fn is_at_start_of_line(&self, offset: TextSize) -> bool {
        self.line_start(offset) == offset
    }

    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ## Panics
    ///
    fn full_line_end(&self, offset: TextSize) -> TextSize;

    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ## Panics
    ///
    fn line_end(&self, offset: TextSize) -> TextSize;

    ///
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ## Panics
    fn full_line_range(&self, offset: TextSize) -> TextRange {
        TextRange::new(self.line_start(offset), self.full_line_end(offset))
    }

    ///
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ## Panics
    fn line_range(&self, offset: TextSize) -> TextRange {
        TextRange::new(self.line_start(offset), self.line_end(offset))
    }

    ///
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ## Panics
    fn full_line_str(&self, offset: TextSize) -> &str;

    ///
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ## Panics
    fn line_str(&self, offset: TextSize) -> &str;

    ///
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ## Panics
    fn full_lines_range(&self, range: TextRange) -> TextRange {
        TextRange::new(
            self.line_start(range.start()),
            self.full_line_end(range.end()),
        )
    }

    ///
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ## Panics
    fn lines_range(&self, range: TextRange) -> TextRange {
        TextRange::new(self.line_start(range.start()), self.line_end(range.end()))
    }

    ///
    ///
    ///
    ///
    /// ## Panics
    fn contains_line_break(&self, range: TextRange) -> bool;

    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ## Panics
    fn lines_str(&self, range: TextRange) -> &str;

    ///
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ## Panics
    fn full_lines_str(&self, range: TextRange) -> &str;

    ///
    /// ## Examples
    ///
    ///
    fn count_lines(&self, range: TextRange) -> u32 {
        let mut count = 0;
        let mut line_end = self.line_end(range.start());

        loop {
            let next_line_start = self.full_line_end(line_end);

            // Reached the end of the string
            if next_line_start == line_end {
                break count;
            }

            // Range ends at the line boundary
            if line_end >= range.end() {
                break count;
            }

            count += 1;

            line_end = self.line_end(next_line_start);
        }
    }
}

impl LineRanges for str {
    fn line_start(&self, offset: TextSize) -> TextSize {
        let bytes = self[TextRange::up_to(offset)].as_bytes();
        if let Some(index) = memrchr2(b'\n', b'\r', bytes) {
            // SAFETY: Safe because `index < offset`
            TextSize::try_from(index).unwrap().add(TextSize::from(1))
        } else {
            self.bom_start_offset()
        }
    }

    fn bom_start_offset(&self) -> TextSize {
        if self.starts_with('\u{feff}') {
            // Skip the BOM.
            '\u{feff}'.text_len()
        } else {
            // Start of file.
            TextSize::default()
        }
    }

    fn full_line_end(&self, offset: TextSize) -> TextSize {
        let slice = &self[usize::from(offset)..];
        if let Some((index, line_ending)) = find_newline(slice) {
            offset + TextSize::try_from(index).unwrap() + line_ending.text_len()
        } else {
            self.text_len()
        }
    }

    fn line_end(&self, offset: TextSize) -> TextSize {
        let slice = &self[offset.to_usize()..];
        if let Some(index) = memchr2(b'\n', b'\r', slice.as_bytes()) {
            offset + TextSize::try_from(index).unwrap()
        } else {
            self.text_len()
        }
    }

    fn full_line_str(&self, offset: TextSize) -> &str {
        &self[self.full_line_range(offset)]
    }

    fn line_str(&self, offset: TextSize) -> &str {
        &self[self.line_range(offset)]
    }

    fn contains_line_break(&self, range: TextRange) -> bool {
        memchr2(b'\n', b'\r', self[range].as_bytes()).is_some()
    }

    fn lines_str(&self, range: TextRange) -> &str {
        &self[self.lines_range(range)]
    }

    fn full_lines_str(&self, range: TextRange) -> &str {
        &self[self.full_lines_range(range)]
    }
}
