use std::fmt;
use std::fmt::{Debug, Formatter};
use std::num::{NonZeroUsize, ParseIntError};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use crate::{LineColumn, SourceLocation};
use rpa_text_size::{TextLen, TextRange, TextSize};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "get-size", derive(get-size2::GetSize))]
pub struct LineIndex {
    inner: Arc<LineIndexInner>,
}

#[derive(Eq, PartialEq)]
#[cfg_attr(feature = "get-size", derive(get-size2::GetSize))]
struct LineIndexInner {
    line_starts: Vec<TextSize>,
    kind: IndexKind,
}

impl LineIndex {
    pub fn from_source_text(text: &str) -> Self {
        let mut line_starts: Vec<TextSize> = Vec::with_capacity(text.len() / 88);
        line_starts.push(TextSize::default());

        let bytes = text.as_bytes();
        let mut utf8 = false;

        assert!(u32::try_from(bytes.len()).is_ok());

        for (i, byte) in bytes.iter().enumerate() {
            utf8 |= !byte.is_ascii();

            match byte {
                // Only track one line break for `\r\n`.
                b'\r' if bytes.get(i + 1) == Some(&b'\n') => continue,
                b'\n' | b'\r' => {
                    // SAFETY: Assertion above guarantees `i <= u32::MAX`
                    #[expect(clippy::cast_possible_truncation)]
                    line_starts.push(TextSize::from(i as u32) + TextSize::from(1));
                }
                _ => {}
            }
        }

        let kind = if utf8 {
            IndexKind::Utf8
        } else {
            IndexKind::Ascii
        };

        Self {
            inner: Arc::new(LineIndexInner { line_starts, kind }),
        }
    }

    fn kind(&self) -> IndexKind {
        self.inner.kind
    }

    ///
    ///
    /// ### BOM handling
    ///
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    ///
    /// ## Panics
    ///
    pub fn line_column(&self, offset: TextSize, content: &str) -> LineColumn {
        let location = self.source_location(offset, content, PositionEncoding::Utf32);

        // Don't count the BOM character as a column, but only on the first line.
        let column = if location.line.to_zero_indexed() == 0 && content.starts_with('\u{feff}') {
            location.character_offset.saturating_sub(1)
        } else {
            location.character_offset
        };

        LineColumn {
            line: location.line,
            column,
        }
    }

    ///
    /// ### BOM handling
    ///
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    ///
    /// ## Panics
    ///
    pub fn source_location(
        &self,
        offset: TextSize,
        text: &str,
        encoding: PositionEncoding,
    ) -> SourceLocation {
        let line = self.line_index(offset);
        let line_start = self.line_start(line, text);

        if self.is_ascii() {
            return SourceLocation {
                line,
                character_offset: OneIndexed::from_zero_indexed((offset - line_start).to_usize()),
            };
        }

        match encoding {
            PositionEncoding::Utf8 => {
                let character_offset = offset - line_start;
                SourceLocation {
                    line,
                    character_offset: OneIndexed::from_zero_indexed(character_offset.to_usize()),
                }
            }
            PositionEncoding::Utf16 => {
                let up_to_character = &text[TextRange::new(line_start, offset)];
                let character = up_to_character.encode_utf16().count();

                SourceLocation {
                    line,
                    character_offset: OneIndexed::from_zero_indexed(character),
                }
            }
            PositionEncoding::Utf32 => {
                let up_to_character = &text[TextRange::new(line_start, offset)];
                let character = up_to_character.chars().count();

                SourceLocation {
                    line,
                    character_offset: OneIndexed::from_zero_indexed(character),
                }
            }
        }
    }

    pub fn line_count(&self) -> usize {
        self.line_starts().len()
    }

    pub fn is_ascii(&self) -> bool {
        self.kind().is_ascii()
    }

    ///
    /// ## Examples
    ///
    ///
    ///
    /// ## Panics
    ///
    pub fn line_index(&self, offset: TextSize) -> OneIndexed {
        match self.line_starts().binary_search(&offset) {
            // Offset is at the start of a line
            Ok(row) => OneIndexed::from_zero_indexed(row),
            Err(row) => {
                // SAFETY: Safe because the index always contains an entry for the offset 0
                OneIndexed::from_zero_indexed(row - 1)
            }
        }
    }

    pub fn line_start(&self, line: OneIndexed, contents: &str) -> TextSize {
        let row_index = line.to_zero_indexed();
        let starts = self.line_starts();

        // If start-of-line position after last line
        if row_index == starts.len() {
            contents.text_len()
        } else {
            starts[row_index]
        }
    }

    pub fn line_end(&self, line: OneIndexed, contents: &str) -> TextSize {
        let row_index = line.to_zero_indexed();
        let starts = self.line_starts();

        // If start-of-line position after last line
        if row_index.saturating_add(1) >= starts.len() {
            contents.text_len()
        } else {
            starts[row_index + 1]
        }
    }

    pub fn line_end_exclusive(&self, line: OneIndexed, contents: &str) -> TextSize {
        let row_index = line.to_zero_indexed();
        let starts = self.line_starts();

        // If start-of-line position after last line
        if row_index.saturating_add(1) >= starts.len() {
            contents.text_len()
        } else {
            starts[row_index + 1] - TextSize::new(1)
        }
    }

    pub fn line_range(&self, line: OneIndexed, contents: &str) -> TextRange {
        let starts = self.line_starts();

        if starts.len() == line.to_zero_indexed() {
            TextRange::empty(contents.text_len())
        } else {
            TextRange::new(
                self.line_start(line, contents),
                self.line_start(line.saturating_add(1), contents),
            )
        }
    }

    ///
    /// ## Examples
    ///
    /// ### ASCII only source text
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    /// ### Non-ASCII source text
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn offset(
        &self,
        position: SourceLocation,
        text: &str,
        position_encoding: PositionEncoding,
    ) -> TextSize {
        // If start-of-line position after last line
        if position.line.to_zero_indexed() > self.line_starts().len() {
            return text.text_len();
        }

        let line_range = self.line_range(position.line, text);

        let character_offset = position.character_offset.to_zero_indexed();
        let character_byte_offset = if self.is_ascii() {
            TextSize::try_from(character_offset).unwrap()
        } else {
            let line = &text[line_range];

            match position_encoding {
                PositionEncoding::Utf8 => {
                    TextSize::try_from(position.character_offset.to_zero_indexed()).unwrap()
                }
                PositionEncoding::Utf16 => {
                    let mut byte_offset = TextSize::new(0);
                    let mut utf16_code_unit_offset = 0;

                    for c in line.chars() {
                        if utf16_code_unit_offset >= character_offset {
                            break;
                        }

                        // Count characters encoded as two 16 bit words as 2 characters.
                        byte_offset += c.text_len();
                        utf16_code_unit_offset += c.len_utf16();
                    }

                    byte_offset
                }
                PositionEncoding::Utf32 => line
                    .chars()
                    .take(position.character_offset.to_zero_indexed())
                    .map(rpa_text_size::TextLen::text_len)
                    .sum(),
            }
        };

        line_range.start() + character_byte_offset.clamp(TextSize::new(0), line_range.len())
    }

    pub fn line_starts(&self) -> &[TextSize] {
        &self.inner.line_starts
    }
}

impl Deref for LineIndex {
    type Target = [TextSize];

    fn deref(&self) -> &Self::Target {
        self.line_starts()
    }
}

impl Debug for LineIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.line_starts()).finish()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "get-size", derive(get-size2::GetSize))]
enum IndexKind {
    Ascii,

    Utf8,
}

impl IndexKind {
    const fn is_ascii(self) -> bool {
        matches!(self, IndexKind::Ascii)
    }
}

///
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OneIndexed(NonZeroUsize);

impl OneIndexed {
    pub const MAX: Self = unwrap(Self::new(usize::MAX));
    // SAFETY: These constants are being initialized with non-zero values
    pub const MIN: Self = unwrap(Self::new(1));
    pub const ONE: NonZeroUsize = unwrap(NonZeroUsize::new(1));

    pub const fn new(value: usize) -> Option<Self> {
        match NonZeroUsize::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    pub const fn from_zero_indexed(value: usize) -> Self {
        Self(Self::ONE.saturating_add(value))
    }

    pub const fn get(self) -> usize {
        self.0.get()
    }

    pub const fn to_zero_indexed(self) -> usize {
        self.0.get() - 1
    }

    #[must_use]
    pub const fn saturating_add(self, rhs: usize) -> Self {
        match NonZeroUsize::new(self.0.get().saturating_add(rhs)) {
            Some(value) => Self(value),
            None => Self::MAX,
        }
    }

    #[must_use]
    pub const fn saturating_sub(self, rhs: usize) -> Self {
        match NonZeroUsize::new(self.0.get().saturating_sub(rhs)) {
            Some(value) => Self(value),
            None => Self::MIN,
        }
    }

    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0.get()).map(Self)
    }

    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.get().checked_sub(rhs.get()).and_then(Self::new)
    }
}

impl fmt::Display for OneIndexed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(&self.0.get(), f)
    }
}

const fn unwrap<T: Copy>(option: Option<T>) -> T {
    match option {
        Some(value) => value,
        None => panic!("unwrapping None"),
    }
}

impl FromStr for OneIndexed {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OneIndexed(NonZeroUsize::from_str(s)?))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PositionEncoding {
    Utf8,

    Utf16,

    Utf32,
}

#[cfg(test)]
mod tests {
    use rpa_text_size::TextSize;

    use crate::line_index::LineIndex;
    use crate::{LineColumn, OneIndexed};

    #[test]
    fn ascii_index() {
        let index = LineIndex::from_source_text("");
        assert_eq!(index.line_starts(), &[TextSize::from(0)]);

        let index = LineIndex::from_source_text("x = 1");
        assert_eq!(index.line_starts(), &[TextSize::from(0)]);

        let index = LineIndex::from_source_text("x = 1\n");
        assert_eq!(index.line_starts(), &[TextSize::from(0), TextSize::from(6)]);

        let index = LineIndex::from_source_text("x = 1\ny = 2\nz = x + y\n");
        assert_eq!(
            index.line_starts(),
            &[
                TextSize::from(0),
                TextSize::from(6),
                TextSize::from(12),
                TextSize::from(22)
            ]
        );
    }

    #[test]
    fn ascii_source_location() {
        let contents = "x = 1\ny = 2";
        let index = LineIndex::from_source_text(contents);

        // First row.
        let loc = index.line_column(TextSize::from(2), contents);
        assert_eq!(
            loc,
            LineColumn {
                line: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(2)
            }
        );

        // Second row.
        let loc = index.line_column(TextSize::from(6), contents);
        assert_eq!(
            loc,
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );

        let loc = index.line_column(TextSize::from(11), contents);
        assert_eq!(
            loc,
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(5)
            }
        );
    }

    #[test]
    fn ascii_carriage_return() {
        let contents = "x = 4\ry = 3";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(index.line_starts(), &[TextSize::from(0), TextSize::from(6)]);

        assert_eq!(
            index.line_column(TextSize::from(4), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(4)
            }
        );
        assert_eq!(
            index.line_column(TextSize::from(6), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );
        assert_eq!(
            index.line_column(TextSize::from(7), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(1)
            }
        );
    }

    #[test]
    fn ascii_carriage_return_newline() {
        let contents = "x = 4\r\ny = 3";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(index.line_starts(), &[TextSize::from(0), TextSize::from(7)]);

        assert_eq!(
            index.line_column(TextSize::from(4), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(4)
            }
        );
        assert_eq!(
            index.line_column(TextSize::from(7), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );
        assert_eq!(
            index.line_column(TextSize::from(8), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(1)
            }
        );
    }

    #[test]
    fn utf8_index() {
        let index = LineIndex::from_source_text("x = '🫣'");
        assert_eq!(index.line_count(), 1);
        assert_eq!(index.line_starts(), &[TextSize::from(0)]);

        let index = LineIndex::from_source_text("x = '🫣'\n");
        assert_eq!(index.line_count(), 2);
        assert_eq!(
            index.line_starts(),
            &[TextSize::from(0), TextSize::from(11)]
        );

        let index = LineIndex::from_source_text("x = '🫣'\ny = 2\nz = x + y\n");
        assert_eq!(index.line_count(), 4);
        assert_eq!(
            index.line_starts(),
            &[
                TextSize::from(0),
                TextSize::from(11),
                TextSize::from(17),
                TextSize::from(27)
            ]
        );

        let index = LineIndex::from_source_text("# 🫣\nclass Foo:\n    \"\"\".\"\"\"");
        assert_eq!(index.line_count(), 3);
        assert_eq!(
            index.line_starts(),
            &[TextSize::from(0), TextSize::from(7), TextSize::from(18)]
        );
    }

    #[test]
    fn utf8_carriage_return() {
        let contents = "x = '🫣'\ry = 3";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(index.line_count(), 2);
        assert_eq!(
            index.line_starts(),
            &[TextSize::from(0), TextSize::from(11)]
        );

        // Second '
        assert_eq!(
            index.line_column(TextSize::from(9), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(6)
            }
        );
        assert_eq!(
            index.line_column(TextSize::from(11), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );
        assert_eq!(
            index.line_column(TextSize::from(12), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(1)
            }
        );
    }

    #[test]
    fn utf8_carriage_return_newline() {
        let contents = "x = '🫣'\r\ny = 3";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(index.line_count(), 2);
        assert_eq!(
            index.line_starts(),
            &[TextSize::from(0), TextSize::from(12)]
        );

        // Second '
        assert_eq!(
            index.line_column(TextSize::from(9), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(6)
            }
        );
        assert_eq!(
            index.line_column(TextSize::from(12), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );
        assert_eq!(
            index.line_column(TextSize::from(13), contents),
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(1)
            }
        );
    }

    #[test]
    fn utf8_byte_offset() {
        let contents = "x = '☃'\ny = 2";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(
            index.line_starts(),
            &[TextSize::from(0), TextSize::from(10)]
        );

        // First row.
        let loc = index.line_column(TextSize::from(0), contents);
        assert_eq!(
            loc,
            LineColumn {
                line: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(0)
            }
        );

        let loc = index.line_column(TextSize::from(5), contents);
        assert_eq!(
            loc,
            LineColumn {
                line: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(5)
            }
        );

        let loc = index.line_column(TextSize::from(8), contents);
        assert_eq!(
            loc,
            LineColumn {
                line: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(6)
            }
        );

        // Second row.
        let loc = index.line_column(TextSize::from(10), contents);
        assert_eq!(
            loc,
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );

        // One-past-the-end.
        let loc = index.line_column(TextSize::from(15), contents);
        assert_eq!(
            loc,
            LineColumn {
                line: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(5)
            }
        );
    }
}
