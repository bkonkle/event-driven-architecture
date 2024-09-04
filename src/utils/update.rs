//! A struct representing an optional Update. This update can be Unchanged, set to Empty, or set to
//! a particular Value.

use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Similar to `Option`, but it has three states, `unchanged`, `empty` and `value`.
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Update<T> {
    Unchanged,
    Empty,
    Value(T),
}

impl<T> Default for Update<T> {
    fn default() -> Self {
        Self::Unchanged
    }
}

impl<T> Update<T> {
    /// Returns true if the `Update<T>` is unchanged.
    #[inline]
    pub const fn is_unchanged(&self) -> bool {
        matches!(self, Update::Unchanged)
    }

    /// Returns true if the `Update<T>` is changed, either to empty or a value.
    #[inline]
    pub const fn is_changed(&self) -> bool {
        !self.is_unchanged()
    }

    /// Returns true if the `Update<T>` is set to empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Update::Empty)
    }

    /// Returns true if the `Update<T>` contains a value.
    #[inline]
    pub const fn is_value(&self) -> bool {
        matches!(self, Update::Value(_))
    }

    /// Borrow the value, returns `None` if the the `Update<T>` is
    /// `unchanged` or `empty`, otherwise returns `Some(T)`.
    #[inline]
    pub const fn value(&self) -> Option<&T> {
        match self {
            Update::Value(value) => Some(value),
            _ => None,
        }
    }

    /// Converts the `Update<T>` to `Option<T>`.
    #[inline]
    pub fn take(self) -> Option<T> {
        match self {
            Update::Value(value) => Some(value),
            _ => None,
        }
    }

    /// Converts the `Update<T>` to `Option<Option<T>>`.
    #[allow(clippy::option_option)]
    #[inline]
    pub const fn as_opt_ref(&self) -> Option<Option<&T>> {
        match self {
            Update::Unchanged => None,
            Update::Empty => Some(None),
            Update::Value(value) => Some(Some(value)),
        }
    }

    /// Converts the `Update<T>` to `Option<Option<&U>>`.
    #[allow(clippy::option_option)]
    #[inline]
    pub fn as_opt_deref<U>(&self) -> Option<Option<&U>>
    where
        U: ?Sized,
        T: Deref<Target = U>,
    {
        match self {
            Update::Unchanged => None,
            Update::Empty => Some(None),
            Update::Value(value) => Some(Some(&**value)),
        }
    }

    /// Returns `true` if the `Update<T>` contains the given value.
    #[inline]
    pub fn contains_value<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>,
    {
        match self {
            Update::Value(y) => x == y,
            _ => false,
        }
    }

    /// Returns `true` if the `Update<T>` contains the given possibly empty
    /// value.
    #[inline]
    pub fn contains<U>(&self, x: &Option<U>) -> bool
    where
        U: PartialEq<T>,
    {
        match self {
            Update::Value(y) => matches!(x, Some(v) if v == y),
            Update::Empty => x.is_none(),
            Update::Unchanged => false,
        }
    }

    /// Maps a `Update<T>` to `Update<U>` by applying a function
    /// to the contained possibly empty value
    #[inline]
    pub fn map<U, F: FnOnce(Option<T>) -> Option<U>>(self, f: F) -> Update<U> {
        match self {
            Update::Value(v) => match f(Some(v)) {
                Some(v) => Update::Value(v),
                None => Update::Empty,
            },
            Update::Empty => match f(None) {
                Some(v) => Update::Value(v),
                None => Update::Empty,
            },
            Update::Unchanged => Update::Unchanged,
        }
    }

    /// Maps a `Update<T>` to `Update<U>` by applying a function
    /// to the contained value
    #[inline]
    pub fn map_value<U, F: FnOnce(T) -> U>(self, f: F) -> Update<U> {
        match self {
            Update::Value(v) => Update::Value(f(v)),
            Update::Empty => Update::Empty,
            Update::Unchanged => Update::Unchanged,
        }
    }

    /// Update `value` if the `Update<T>` is not unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::utils::Update;
    ///
    /// let mut value = None;
    ///
    /// Update::Value(10i32).update_to(&mut value);
    /// assert_eq!(value, Some(10));
    ///
    /// Update::Unchanged.update_to(&mut value);
    /// assert_eq!(value, Some(10));
    ///
    /// Update::Empty.update_to(&mut value);
    /// assert_eq!(value, None);
    /// ```
    pub fn update_to(self, value: &mut Option<T>) {
        match self {
            Update::Value(new) => *value = Some(new),
            Update::Empty => *value = None,
            Update::Unchanged => {}
        };
    }
}

impl<T, E> Update<Result<T, E>> {
    /// Transposes a `Update` of a [`Result`] into a [`Result`] of a
    /// `Update`.
    ///
    /// [`Update::Unchanged`] will be mapped to
    /// [`Ok`]`(`[`Update::Unchanged`]`)`. [`Update::Empty`]
    /// will be mapped to [`Ok`]`(`[`Update::Empty`]`)`.
    /// [`Update::Value`]`(`[`Ok`]`(_))` and
    /// [`Update::Value`]`(`[`Err`]`(_))` will be mapped to
    /// [`Ok`]`(`[`Update::Value`]`(_))` and [`Err`]`(_)`.
    #[inline]
    pub fn transpose(self) -> Result<Update<T>, E> {
        match self {
            Update::Unchanged => Ok(Update::Unchanged),
            Update::Empty => Ok(Update::Empty),
            Update::Value(Ok(v)) => Ok(Update::Value(v)),
            Update::Value(Err(e)) => Err(e),
        }
    }
}

impl<T: Serialize> Serialize for Update<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Update::Value(value) => value.serialize(serializer),
            _ => serializer.serialize_none(),
        }
    }
}

impl<'de, T> Deserialize<'de> for Update<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Update<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(value) => Update::Value(value),
            None => Update::Empty,
        })
    }
}

impl<T> From<Update<T>> for Option<Option<T>> {
    fn from(update: Update<T>) -> Self {
        match update {
            Update::Unchanged => None,
            Update::Empty => Some(None),
            Update::Value(value) => Some(Some(value)),
        }
    }
}

impl<T> From<Option<Option<T>>> for Update<T> {
    fn from(value: Option<Option<T>>) -> Self {
        match value {
            Some(Some(value)) => Self::Value(value),
            Some(None) => Self::Empty,
            None => Self::Unchanged,
        }
    }
}
