use crate::BigArray;
use core::ops::{Index, IndexMut};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An array newtype usable for nested structures
///
/// In most cases, using the [`BigArray`] trait
/// is more convenient, so you should use that one.
///
/// In nesting scenarios however, the trick of using
/// `#[serde(with = ...)]` comes to its limits. For
/// these cases, we offer the `Array` struct.
///
/// [`BigArray`]: crate::BigArray
///
/// ```Rust
/// # use serde_derive::{Serialize, Deserialize};
/// #[derive(Serialize, Deserialize)]
/// struct S {
///     arr: Array<u8, 64>,
/// }
/// ```
#[derive(Eq, PartialEq, PartialOrd, Copy, Clone, Ord, Debug)]
pub struct Array<T, const N: usize>(pub [T; N]);

impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for Array<T, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(<[T; N] as BigArray<T>>::deserialize(deserializer)?))
    }
}

impl<T: Serialize, const N: usize> Serialize for Array<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <[T; N] as BigArray<T>>::serialize(&self.0, serializer)
    }
}

impl<T, I, const N: usize> Index<I> for Array<T, N>
where
    [T]: Index<I>,
{
    type Output = <[T] as Index<I>>::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0 as &[T], index)
    }
}

impl<T, I, const N: usize> IndexMut<I> for Array<T, N>
where
    [T]: IndexMut<I>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.0 as &mut [T], index)
    }
}
