use {
    crate::TextLen,
    std::{
        convert::TryFrom,
        fmt, iter,
        num::TryFromIntError,
        ops::{Add, AddAssign, Sub, SubAssign},
    },
};

/// A measure of text length. Also, equivalently, an index into text.
///
/// This is a UTF-8 bytes offset stored as `u32`, but
/// most clients should treat it as an opaque measure.
///
/// For cases that need to escape `TextSize` and return to working directly
/// with primitive integers, `TextSize` can be converted losslessly to/from
/// `u32` via [`From`] conversions as well as losslessly be converted [`Into`]
/// `usize`. The `usize -> TextSize` direction can be done via [`TryFrom`].
///
/// These escape hatches are primarily required for unit testing and when
/// converting from UTF-8 size to another coordinate space, such as UTF-16.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "get-size", derive(get-size2::GetSize))]
pub struct TextSize {
    pub(crate) raw: u32,
}

impl fmt::Debug for TextSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl TextSize {
    /// Creates a new `TextSize` at the given `offset`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rpa_text_size::*;
    /// assert_eq!(TextSize::from(4), TextSize::new(4));
    /// ```
    pub const fn new(offset: u32) -> Self {
        Self { raw: offset }
    }

    /// The text size of some primitive text-like object.
    ///
    /// Accepts `char`, `&str`, and `&String`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rpa_text_size::*;
    /// let char_size = TextSize::of('🦀');
    /// assert_eq!(char_size, TextSize::from(4));
    ///
    /// let str_size = TextSize::of("rust-analyzer");
    /// assert_eq!(str_size, TextSize::from(13));
    /// ```
    #[inline]
    pub fn of<T: TextLen>(text: T) -> TextSize {
        text.text_len()
    }

    /// Returns current raw `offset` as u32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rpa_text_size::*;
    /// assert_eq!(TextSize::from(4).to_u32(), 4);
    /// ```
    pub const fn to_u32(&self) -> u32 {
        self.raw
    }

    /// Returns current raw `offset` as usize.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rpa_text_size::*;
    /// assert_eq!(TextSize::from(4).to_usize(), 4);
    /// ```
    pub const fn to_usize(&self) -> usize {
        self.raw as usize
    }
}

/// Methods to act like a primitive integer type, where reasonably applicable.
//  Last updated for parity with Rust 1.42.0.
impl TextSize {
    /// Checked addition. Returns `None` if overflow occurred.
    #[inline]
    pub fn checked_add(self, rhs: TextSize) -> Option<TextSize> {
        self.raw.checked_add(rhs.raw).map(|raw| TextSize { raw })
    }

    /// Checked subtraction. Returns `None` if overflow occurred.
    #[inline]
    pub fn checked_sub(self, rhs: TextSize) -> Option<TextSize> {
        self.raw.checked_sub(rhs.raw).map(|raw| TextSize { raw })
    }
}

impl From<u32> for TextSize {
    #[inline]
    fn from(raw: u32) -> Self {
        TextSize::new(raw)
    }
}

impl From<TextSize> for u32 {
    #[inline]
    fn from(value: TextSize) -> Self {
        value.to_u32()
    }
}

impl TryFrom<usize> for TextSize {
    type Error = TryFromIntError;
    #[inline]
    fn try_from(value: usize) -> Result<Self, TryFromIntError> {
        Ok(u32::try_from(value)?.into())
    }
}

impl From<TextSize> for usize {
    #[inline]
    fn from(value: TextSize) -> Self {
        value.to_usize()
    }
}

macro_rules! ops {
    (impl $Op:ident for TextSize by fn $f:ident = $op:tt) => {
        impl $Op<TextSize> for TextSize {
            type Output = TextSize;
            #[inline]
            fn $f(self, other: TextSize) -> TextSize {
                TextSize { raw: self.raw $op other.raw }
            }
        }
        impl $Op<&TextSize> for TextSize {
            type Output = TextSize;
            #[inline]
            fn $f(self, other: &TextSize) -> TextSize {
                self $op *other
            }
        }
        impl<T> $Op<T> for &TextSize
        where
            TextSize: $Op<T, Output=TextSize>,
        {
            type Output = TextSize;
            #[inline]
            fn $f(self, other: T) -> TextSize {
                *self $op other
            }
        }
    };
}

ops!(impl Add for TextSize by fn add = +);
ops!(impl Sub for TextSize by fn sub = -);

impl<A> AddAssign<A> for TextSize
where
    TextSize: Add<A, Output = TextSize>,
{
    #[inline]
    fn add_assign(&mut self, rhs: A) {
        *self = *self + rhs;
    }
}

impl<S> SubAssign<S> for TextSize
where
    TextSize: Sub<S, Output = TextSize>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: S) {
        *self = *self - rhs;
    }
}

impl<A> iter::Sum<A> for TextSize
where
    TextSize: Add<A, Output = TextSize>,
{
    #[inline]
    fn sum<I: Iterator<Item = A>>(iter: I) -> TextSize {
        iter.fold(0.into(), Add::add)
    }
}
