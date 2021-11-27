use std::borrow::Borrow;

use crate::IntoOwned;

/// Due to implementation conflicts, [`IntoOwned`] cannot be directly implemented on `T`.
/// So instead, this is a light wrapper that allows the implementation.
pub struct Owned<T>(pub T);

impl<B: ?Sized + ToOwned> Borrow<B> for Owned<B::Owned> {
    fn borrow(&self) -> &B {
        self.0.borrow()
    }
}

impl<B: ?Sized + ToOwned> IntoOwned<B> for Owned<B::Owned> {
    fn into_owned(self) -> B::Owned {
        self.0
    }
}

impl<T> From<T> for Owned<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T: AsRef<U>, U> AsRef<U> for Owned<T> {
    fn as_ref(&self) -> &U {
        self.0.as_ref()
    }
}
