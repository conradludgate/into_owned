//! Provides a way to efficiently get an owned value of a type.
//!
//! ```
//! # use std::borrow::Cow;
//! use into_owned::{IntoOwned, Owned};
//!
//! /// A simple type that increments an internal counter on clone
//! pub struct Counter(usize);
//! impl Clone for Counter {
//!     fn clone(&self) -> Self {
//!         Self(self.0 + 1)
//!     }
//! }
//!
//! fn check<S: IntoOwned<Counter>>(s: S) -> Counter {
//!     s.into_owned()
//! }
//!
//! let count = check(&Counter(0)); // pass by ref
//! assert_eq!(count.0, 1); // counter was cloned.
//!
//! let count = check(Owned(Counter(0))); // pass by value
//! assert_eq!(count.0, 0); // counter was not cloned.
//!
//! let count = check(Cow::Borrowed(&Counter(0))); // pass by ref (Cow)
//! assert_eq!(count.0, 1); // counter was cloned.
//!
//! let count = check(Cow::Owned(Counter(0))); // pass by value (Cow)
//! assert_eq!(count.0, 0); // counter was not cloned.
//! ```

use std::borrow::{Borrow, Cow};
mod owned;
pub use owned::Owned;

pub trait IntoOwned<T: ?Sized + ToOwned>: Borrow<T> {
    fn into_owned(self) -> T::Owned;
}

/// Default implemetation for references.
/// [`IntoOwned::into_owned()`] will perform [`ToOwned::to_owned()`]
impl<T: ToOwned + ?Sized> IntoOwned<T> for &T {
    fn into_owned(self) -> T::Owned {
        self.to_owned()
    }
}

/// Default implemetation for Cow.
/// [`IntoOwned::into_owned()`] will perform [`Cow::into_owned()`]
impl<'a, T: ToOwned + ?Sized> IntoOwned<T> for Cow<'a, T> {
    fn into_owned(self) -> <T as ToOwned>::Owned {
        self.into_owned()
    }
}
