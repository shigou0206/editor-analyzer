use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::sync::{Arc, OnceLock};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use rpa_text_size::{Ranged, TextRange, TextSize};

pub use crate::line_index::{LineIndex, OneIndexed, PositionEncoding};
pub use crate::line_ranges::LineRanges;
pub use crate::newlines::{
    Line, LineEnding, NewlineWithTrailingNewline, UniversalNewlineIterator, UniversalNewlines,
    find_newline,
};

mod line_index;
mod line_ranges;
mod newlines;

#[derive(Debug)]
pub struct SourceCode<'src, 'index> {
    text: &'src str,
    index: &'index LineIndex,
}

impl<'src, 'index> SourceCode<'src, 'index> {
    pub fn new(content: &'src str, index: &'index LineIndex) -> Self {
        Self {
            text: content,
            index,
        }
    }

    #[inline]
    pub fn line_column(&self, offset: TextSize) -> LineColumn {
        self.index.line_column(offset, self.text)
    }

    #[inline]
    pub fn source_location(
        &self,
        offset: TextSize,
        position_encoding: PositionEncoding,
    ) -> SourceLocation {
        self.index
            .source_location(offset, self.text, position_encoding)
    }

    #[inline]
    pub fn line_index(&self, offset: TextSize) -> OneIndexed {
        self.index.line_index(offset)
    }

    #[inline]
    pub fn up_to(&self, offset: TextSize) -> &'src str {
        &self.text[TextRange::up_to(offset)]
    }

    #[inline]
    pub fn after(&self, offset: TextSize) -> &'src str {
        &self.text[usize::from(offset)..]
    }

    pub fn slice<T: Ranged>(&self, ranged: T) -> &'src str {
        &self.text[ranged.range()]
    }

    pub fn line_start(&self, line: OneIndexed) -> TextSize {
        self.index.line_start(line, self.text)
    }

    pub fn line_end(&self, line: OneIndexed) -> TextSize {
        self.index.line_end(line, self.text)
    }

    pub fn line_end_exclusive(&self, line: OneIndexed) -> TextSize {
        self.index.line_end_exclusive(line, self.text)
    }

    pub fn line_range(&self, line: OneIndexed) -> TextRange {
        self.index.line_range(line, self.text)
    }

    #[inline]
    pub fn line_text(&self, index: OneIndexed) -> &'src str {
        let range = self.index.line_range(index, self.text);
        &self.text[range]
    }

    pub fn text(&self) -> &'src str {
        self.text
    }

    #[inline]
    pub fn line_count(&self) -> usize {
        self.index.line_count()
    }
}

impl PartialEq<Self> for SourceCode<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Eq for SourceCode<'_, '_> {}

pub struct SourceFileBuilder {
    name: Box<str>,
    code: Box<str>,
    index: Option<LineIndex>,
}

impl SourceFileBuilder {
    pub fn new<Name: Into<Box<str>>, Code: Into<Box<str>>>(name: Name, code: Code) -> Self {
        Self {
            name: name.into(),
            code: code.into(),
            index: None,
        }
    }

    #[must_use]
    pub fn line_index(mut self, index: LineIndex) -> Self {
        self.index = Some(index);
        self
    }

    pub fn set_line_index(&mut self, index: LineIndex) {
        self.index = Some(index);
    }

    pub fn finish(self) -> SourceFile {
        let index = if let Some(index) = self.index {
            OnceLock::from(index)
        } else {
            OnceLock::new()
        };

        SourceFile {
            inner: Arc::new(SourceFileInner {
                name: self.name,
                code: self.code,
                line_index: index,
            }),
        }
    }
}

///
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "get-size", derive(get-size2::GetSize))]
pub struct SourceFile {
    inner: Arc<SourceFileInner>,
}

impl Debug for SourceFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceFile")
            .field("name", &self.name())
            .field("code", &self.source_text())
            .finish()
    }
}

impl SourceFile {
    #[inline]
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    #[inline]
    pub fn slice(&self, range: TextRange) -> &str {
        &self.source_text()[range]
    }

    pub fn to_source_code(&self) -> SourceCode {
        SourceCode {
            text: self.source_text(),
            index: self.index(),
        }
    }

    pub fn index(&self) -> &LineIndex {
        self.inner
            .line_index
            .get_or_init(|| LineIndex::from_source_text(self.source_text()))
    }

    #[inline]
    pub fn source_text(&self) -> &str {
        &self.inner.code
    }
}

impl PartialOrd for SourceFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SourceFile {
    fn cmp(&self, other: &Self) -> Ordering {
        // Short circuit if these are the same source files
        if Arc::ptr_eq(&self.inner, &other.inner) {
            Ordering::Equal
        } else {
            self.inner.name.cmp(&other.inner.name)
        }
    }
}

#[cfg_attr(feature = "get-size", derive(get-size2::GetSize))]
struct SourceFileInner {
    name: Box<str>,
    code: Box<str>,
    line_index: OnceLock<LineIndex>,
}

impl PartialEq for SourceFileInner {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.code == other.code
    }
}

impl Eq for SourceFileInner {}

///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LineColumn {
    pub line: OneIndexed,
    pub column: OneIndexed,
}

impl Default for LineColumn {
    fn default() -> Self {
        Self {
            line: OneIndexed::MIN,
            column: OneIndexed::MIN,
        }
    }
}

impl Debug for LineColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LineColumn")
            .field("line", &self.line.get())
            .field("column", &self.column.get())
            .finish()
    }
}

impl std::fmt::Display for LineColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{line}:{column}", line = self.line, column = self.column)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceLocation {
    pub line: OneIndexed,
    ///
    pub character_offset: OneIndexed,
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            line: OneIndexed::MIN,
            character_offset: OneIndexed::MIN,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SourceRow {
    Notebook { cell: OneIndexed, line: OneIndexed },
    SourceFile { line: OneIndexed },
}

impl Display for SourceRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceRow::Notebook { cell, line } => write!(f, "cell {cell}, line {line}"),
            SourceRow::SourceFile { line } => write!(f, "line {line}"),
        }
    }
}
